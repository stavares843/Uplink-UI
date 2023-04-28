use std::collections::{BTreeMap, HashMap, HashSet};

use common::{
    icons::outline::Shape as Icon,
    language::get_local_text,
    state::{Action, Identity, State, ToastNotification},
    warp_runner::{RayGunCmd, WarpCmd},
    WARP_CMD_CH,
};
use dioxus::prelude::*;
use dioxus_router::*;
use futures::{channel::oneshot, StreamExt};
use kit::{
    components::user_image::UserImage,
    elements::{
        button::Button,
        checkbox::Checkbox,
        input::{Input, Options, SpecialCharsAction, Validation},
        label::Label,
        Appearance,
    },
};
use uuid::Uuid;
use warp::{crypto::DID, logging::tracing::log};

use crate::UPLINK_ROUTES;

#[derive(Props)]
pub struct Props<'a> {
    oncreate: EventHandler<'a, MouseEvent>,
}

#[allow(non_snake_case)]
pub fn CreateGroup<'a>(cx: Scope<'a, Props<'a>>) -> Element<'a> {
    log::trace!("rendering create_group");
    let state = use_shared_state::<State>(cx)?;
    let router = use_router(cx);
    let friend_prefix = use_state(cx, String::new);
    let selected_friends: &UseState<HashSet<DID>> = use_state(cx, HashSet::new);
    let chat_with: &UseState<Option<Uuid>> = use_state(cx, || None);
    let group_name = use_state(cx, || Some(String::new()));
    let friends_list = HashMap::from_iter(
        state
            .read()
            .friend_identities()
            .iter()
            .map(|id| (id.did_key(), id.clone())),
    );

    if let Some(id) = *chat_with.get() {
        chat_with.set(None);
        state.write().mutate(Action::ChatWith(&id, true));
        if state.read().ui.is_minimal_view() {
            state.write().mutate(Action::SidebarHidden(true));
        }
        router.replace_route(UPLINK_ROUTES.chat, None, None);
    }

    // the leading underscore is to pass this to a prop named "friends"
    let _friends = State::get_friends_by_first_letter(friends_list);

    let ch = use_coroutine(cx, |mut rx: UnboundedReceiver<()>| {
        to_owned![selected_friends, chat_with, group_name];
        async move {
            let warp_cmd_tx = WARP_CMD_CH.tx.clone();
            while rx.next().await.is_some() {
                let recipients: Vec<DID> = selected_friends.current().iter().cloned().collect();
                let group_name: Option<String> = group_name.current().as_ref().clone();
                let group_name_string = group_name.clone().unwrap_or_default();

                let (tx, rx) = oneshot::channel();
                let cmd = RayGunCmd::CreateGroupConversation {
                    recipients,
                    group_name: if group_name_string.is_empty()
                        || group_name_string.chars().all(char::is_whitespace)
                    {
                        None
                    } else {
                        group_name
                    },
                    rsp: tx,
                };

                if let Err(e) = warp_cmd_tx.send(WarpCmd::RayGun(cmd)) {
                    log::error!("failed to send warp command: {}", e);
                    continue;
                }

                let rsp = rx.await.expect("command canceled");

                let id = match rsp {
                    Ok(c) => c,
                    Err(e) => {
                        log::error!("failed to create conversation: {}", e);
                        continue;
                    }
                };
                chat_with.set(Some(id));
            }
        }
    });

    cx.render(rsx!(
        div {
            id: "create-group",
            margin_right: "8px",
            aria_label: "Create Group",
            div {
                id: "create-group-name",
                class: "create-group-name",
                div {
                    align_items: "start",
                    Label {
                        text: get_local_text("messages.group-name"),
                    },
                }
                Input {
                        placeholder:  get_local_text("messages.group-name"),
                        default_text: group_name.get().clone().unwrap_or_default(),
                        aria_label: "groupname-input".into(),
                        options: Options {
                            with_clear_btn: true,
                            ..get_input_options()
                        },
                        onreturn: move |(v, is_valid, _): (String, bool, _)| {
                            if !is_valid {
                                group_name.set(None);
                                return;
                            }
                            group_name.set(Some(v));
                        },
                    },
            },
            div {
                class: "search-input",
                Label {
                    text: "Users".into(),
                },
                Input {
                    // todo: filter friends on input
                    placeholder: get_local_text("uplink.search-placeholder"),
                    disabled: false,
                    aria_label: "chat-search-input".into(),
                    icon: Icon::MagnifyingGlass,
                    options: Options {
                        with_clear_btn: true,
                        react_to_esc_key: true,
                        ..Options::default()
                    },
                    onchange: move |(v, _): (String, _)| {
                        friend_prefix.set(v);
                    },
                }
            }
            render_friends {
                friends: _friends,
                name_prefix: friend_prefix.clone(),
                selected_friends: selected_friends.clone()
            },
            Button {
                text: get_local_text("messages.create-group-chat"),
                aria_label: "create-dm-button".into(),
                appearance: Appearance::Primary,
                onpress: move |e| {
                    log::info!("create dm button");
                    if group_name.get().is_some() {
                        ch.send(());
                        cx.props.oncreate.call(e);
                    } else {
                        state
                        .write()
                        .mutate(common::state::Action::AddToastNotification(
                            ToastNotification::init(
                                "".into(),
                                get_local_text("messages.group-name-invalid"),
                                None,
                                3,
                            ),
                        ));
                    }
                }
            }
        }
    ))
}

#[derive(PartialEq, Props)]
pub struct FriendsProps {
    friends: BTreeMap<char, Vec<Identity>>,
    name_prefix: UseState<String>,
    selected_friends: UseState<HashSet<DID>>,
}

fn render_friends(cx: Scope<FriendsProps>) -> Element {
    let name_prefix = cx.props.name_prefix.get();
    cx.render(rsx!(
        div {
            class: "friend-list vertically-scrollable",
            cx.props.friends.iter().map(
                |(letter, sorted_friends)| {
                    let group_letter = letter.to_string();
                    rsx!(
                        div {
                            key: "friend-group-{group_letter}",
                            class: "friend-group",
                            sorted_friends.iter().filter(|friend| {
                                let name = friend.username();
                                if name.len() < name_prefix.len() {
                                    false
                                } else {
                                    &name[..(name_prefix.len())] == name_prefix
                                }
                            } ).map(|_friend| {
                                rsx!(
                                render_friend {
                                    friend: _friend.clone(),
                                    selected_friends: cx.props.selected_friends.clone()
                                }
                            )})
                        }
                    )
                }
            ),
        }
    ))
}

#[derive(PartialEq, Props)]
pub struct FriendProps {
    friend: Identity,
    selected_friends: UseState<HashSet<DID>>,
}
fn render_friend(cx: Scope<FriendProps>) -> Element {
    let is_checked = use_state(cx, || false);
    if !*is_checked.current()
        && cx
            .props
            .selected_friends
            .current()
            .contains(&cx.props.friend.did_key())
    {
        is_checked.set(true);
    }

    let update_fn = || {
        let friend_did = cx.props.friend.did_key();
        let new_value = !*is_checked.get();
        is_checked.set(new_value);
        let mut friends = cx.props.selected_friends.get().clone();
        if new_value {
            friends.insert(friend_did);
        } else {
            friends.remove(&friend_did);
        }
        cx.props.selected_friends.set(friends);
    };

    cx.render(rsx!(
        div {
            class: "friend-container",
            aria_label: "Friend Container",
            UserImage {
                platform: cx.props.friend.platform().into(),
                status: cx.props.friend.identity_status().into(),
                image: cx.props.friend.profile_picture()
                on_press: move |_| {
                    update_fn();
                },
            },
            div {
                class: "flex-1",
                p {
                    aria_label: "friend-name",
                    onclick: move |_| {
                        update_fn();
                    },
                    cx.props.friend.username(),
                },
            },
            Checkbox {
                disabled: false,
                width: "1em".into(),
                height: "1em".into(),
                is_checked: *is_checked.get(),
                on_click: move |_| {
                    update_fn();
                }
            }
        }
    ))
}

pub fn get_input_options() -> Options {
    // Set up validation options for the input field
    let group_name_validation_options = Validation {
        // The input should have a maximum length of 64
        max_length: Some(64),
        // The input should have a minimum length of 0
        min_length: Some(0),
        // The input should only contain alphanumeric characters
        alpha_numeric_only: true,
        // The input can contain any whitespace
        no_whitespace: false,
        // The input component validation is shared - if you need to allow just colons in, set this to true
        ignore_colons: false,
        // The input should allow any special characters
        // if you need special chars, just pass a vec! with each char necessary, mainly if alpha_numeric_only is true
        special_chars: Some((SpecialCharsAction::Allow, vec![' '])),
    };

    // Set up options for the input field
    Options {
        // Enable validation for the input field with the specified options
        with_validation: Some(group_name_validation_options),
        clear_on_submit: false,
        // Use the default options for the remaining fields
        ..Options::default()
    }
}
