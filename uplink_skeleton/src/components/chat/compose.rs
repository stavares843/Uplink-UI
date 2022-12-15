use std::time::SystemTime;

use dioxus::prelude::*;
use ui_kit::{layout::{topbar::Topbar, chatbar::{Chatbar, Reply}}, components::{user_image::UserImage, indicator::{Status, Platform}, context_menu::{ContextMenu, ContextItem}, message_group::MessageGroup, message::{Message, Order}, message_divider::MessageDivider, message_reply::MessageReply, file_embed::FileEmbed, message_typing::MessageTyping, user_image_group::UserImageGroup}, elements::{button::Button, tooltip::{Tooltip, ArrowPosition}, Appearance}, icons::Icon};
use warp::multipass::identity::Identity;
use warp::raygun::Message as RaygunMessage;

use crate::{state::{State, Action}, components::chat::sidebar::build_participants};

use super::sidebar::build_participants_names;

#[allow(non_snake_case)]
pub fn Compose(cx: Scope) -> Element {
    let state: UseSharedState<State> = use_context::<State>(&cx).unwrap();
    let active_chat = state.read().get_active_chat().unwrap_or_default();

    // TODO: Mockup purposes only.
    let some_time_long_ago = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap_or_default();

    let without_me = state.read().get_without_me(active_chat.participants.clone());
    let active_participant = without_me.first();

    let active_participant = match active_participant {
        Some(u) => u.clone(),
        None => Identity::default(),
    };

    let subtext = active_participant.status_message().unwrap_or_default();

    let is_favorite = state.read().is_favorite(&active_chat);

    let reply_message = match state.read().get_active_chat().unwrap_or_default().replying_to {
        Some(m) => m.value().join("\n").to_string(),
        None => "".into(),
    };

    let first_image = active_participant.graphics().profile_picture();
    let participants_name = build_participants_names(&without_me);

    cx.render(rsx!(
        div {
            id: "compose",
            Topbar {
                with_back_button: false,
                controls: cx.render(
                    rsx! (
                        Button {
                            icon: Icon::Heart,
                            appearance: if is_favorite { Appearance::Primary } else { Appearance::Secondary },
                            tooltip: cx.render(rsx!(
                                Tooltip { 
                                    arrow_position: ArrowPosition::Top, 
                                    text: String::from("Add Favorite")
                                }
                            )),
                            onpress: move |_| {
                                state.write().mutate(Action::ToggleFavorite(active_chat.clone()));
                            }
                        },
                        Button {
                            icon: Icon::Phone,
                            appearance: Appearance::Secondary,
                            tooltip: cx.render(rsx!(
                                Tooltip { 
                                    arrow_position: ArrowPosition::Top, 
                                    text: String::from("Audio Call")
                                }
                            )),
                        },
                        Button {
                            icon: Icon::VideoCamera,
                            appearance: Appearance::Secondary,
                            tooltip: cx.render(rsx!(
                                Tooltip { 
                                    arrow_position: ArrowPosition::Top, 
                                    text: String::from("Video Call")
                                }
                            )),
                        },
                    )
                ),
                cx.render(
                    rsx! (
                        if without_me.len() < 2 {rsx! (
                            UserImage {
                                platform: Platform::Mobile,
                                status: Status::Online,
                                image: first_image
                            }
                        )} else {rsx! (
                            UserImageGroup {
                                participants: build_participants(&without_me)
                            }
                        )}
                        div {
                            class: "user-info",
                            p {
                                class: "username",
                                "{participants_name}"
                            },
                            p {
                                class: "status",
                                "{subtext}"
                            }
                        }
                    )
                ),
            },
            div {
                id: "messages",
                MessageGroup {
                    user_image: cx.render(rsx!(
                        UserImage {
                            platform: Platform::Mobile,
                            status: Status::Online
                        }
                    )),
                    with_sender: "John Doe | Satellite.im".into(),
                    timestamp: some_time_long_ago,
                    Message {
                        order: Order::First,
                        with_text: "This is a message to reply to.".into()
                    },
                    FileEmbed {
                        filename: "Fake.zip".into(),
                        filesize: 3821939,
                        kind: "archive/zip".into(),
                        icon: Icon::ArchiveBoxArrowDown,
                    },
                    Message {
                        order: Order::Middle,
                        with_text: "Another one.".into()
                    },
                    MessageReply {
                        with_text: "This is a message to reply to.".into(),
                        remote: false,
                        with_prefix: "In reply to yourself.".into(),
                        user_image: cx.render(rsx!(
                            UserImage {
                                platform: Platform::Mobile,
                                status: Status::Online
                            }
                        ))
                    },
                    Message {
                        order: Order::Last
                        with_text: "It is for these reasons that I regard the decision last year to shift our efforts in space from low to high gear as among the most important decisions that will be made during my incumbency in the office of the Presidency.".into()
                    }
                },
                MessageDivider {
                    text: "New messages".into(),
                    timestamp: some_time_long_ago,
                },
                MessageGroup {
                    user_image: cx.render(rsx!(
                        UserImage {
                            platform: Platform::Desktop,
                            status: Status::Idle
                        }
                    )),
                    remote: true,
                    with_sender: "Jane Doe | Satellite.im".into(),
                    timestamp: some_time_long_ago,
                    ContextMenu {
                        id: "message-temp".into(),
                        items: cx.render(rsx!(
                            ContextItem {
                                icon: Icon::ArrowLongLeft,
                                text: String::from("Reply"),
                                onpress: move |_| {
                                    let mut reply = RaygunMessage::default();
                                    let chat = state.read().get_active_chat().unwrap_or_default();
                                    reply.set_value(vec!["A Message, with a context menu! (right click me)".into()]);
                                    state.write().mutate(Action::StartReplying(chat, reply.clone()));

                                }
                            },
                            ContextItem {
                                icon: Icon::FaceSmile,
                                text: String::from("React"),
                            },
                        )),
                        Message {
                            remote: true,
                            order: Order::First,
                            with_text: "A Message, with a context menu! (right click me)".into()
                        },
                    },
                    MessageReply {
                        with_text: "Some random message".into(),
                        remote: true,
                        remote_message: true,
                        with_prefix: "Replied to Jane Doe's message".into(),
                        user_image: cx.render(rsx!(
                            UserImage {
                                platform: Platform::Mobile,
                                status: Status::Online
                                
                            }
                        ))
                    },
                    Message {
                        remote: true,
                        order: Order::Middle,
                        with_text: "That is an interesting fake message. I'll put something random too.".into()
                    },
                    MessageReply {
                        with_text: "This is a message to reply to.".into(),
                        remote: true,
                        remote_message: false,
                        with_prefix: "Replied to you".into(),
                        user_image: cx.render(rsx!(
                            UserImage {
                                platform: Platform::Mobile,
                                status: Status::Online
                            }
                        ))
                    },
                    Message {
                        remote: true,
                        order: Order::Last
                        with_text: "It is for these reasons that I regard the decision last year to shift our efforts in space from low to high gear as among the most important decisions that will be made during my incumbency in the office of the Presidency.".into()
                    }
                    FileEmbed {
                        remote: true,
                        filename: "Fake.zip".into(),
                        filesize: 3821939,
                        kind: "archive/zip".into(),
                        icon: Icon::ArchiveBoxArrowDown,
                    },
                },
                MessageTyping {
                    user_image: cx.render(rsx!(
                        UserImage {
                            platform: Platform::Mobile,
                            status: Status::Online
                        }
                    ))
                }
            },
            Chatbar {
                controls: cx.render(rsx!(
                    Button {
                        icon: Icon::ChevronDoubleRight,
                        appearance: Appearance::Secondary,
                        tooltip: cx.render(rsx!(
                            Tooltip { 
                                arrow_position: ArrowPosition::Bottom, 
                                text: String::from("Send")
                            }
                        )),
                    },
                )),
                with_replying_to: cx.render(rsx!(
                    state.read().get_active_chat().unwrap_or_default().replying_to.is_some().then(|| rsx!(
                        Reply {
                            remote: {
                                let our_did = state.read().account.identity.did_key();
                                let their_did = state.read().get_active_chat().unwrap_or_default().replying_to.clone().unwrap_or_default().sender();
                                our_did != their_did
                            },
                            onclose: move |_| {
                                let new_chat = &state.read().get_active_chat().unwrap_or_default();
                                state.write().mutate(Action::CancelReply(new_chat.clone()))
                            },
                            message: reply_message,
                            UserImage {
                                platform: Platform::Mobile,
                                status: Status::Online
                            },
                        }
                    ))
                )),
                with_file_upload: cx.render(rsx!(
                    Button {
                        icon: Icon::Plus,
                        appearance: Appearance::Primary,
                        tooltip: cx.render(rsx!(
                            Tooltip { 
                                arrow_position: ArrowPosition::Bottom, 
                                text: String::from("Upload")
                            }
                        ))
                    }
                ))
            }
        }  
    ))
}
