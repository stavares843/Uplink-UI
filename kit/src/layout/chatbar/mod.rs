use dioxus::prelude::*;
use warp::constellation::file::File;

use crate::{
    components::file_embed::FileEmbed,
    elements::{button::Button, label::Label, textarea, Appearance},
};

use common::{icons, language::get_local_text};

pub type To = &'static str;

#[derive(Clone, PartialEq)]
pub struct Route {
    pub to: To,
    pub icon: icons::outline::Shape,
    pub name: &'static str,
}

#[derive(Default)]
pub struct ReplyInfo<'a> {
    pub user_image: Element<'a>,
    pub message: String,
}

#[derive(Props)]
pub struct Props<'a> {
    id: String,
    placeholder: String,
    with_replying_to: Option<Element<'a>>,
    with_file_upload: Option<Element<'a>>,
    extensions: Option<Element<'a>>,
    controls: Option<Element<'a>>,
    value: Option<String>,
    loading: Option<bool>,
    onchange: EventHandler<'a, String>,
    onreturn: EventHandler<'a, String>,
    #[props(default = false)]
    is_disabled: bool,
    ignore_focus: bool,
}

#[derive(Props)]
pub struct ReplyProps<'a> {
    label: String,
    remote: Option<bool>,
    message: String,
    attachments: Option<Vec<File>>,
    onclose: EventHandler<'a>,
    children: Element<'a>,
}

#[allow(non_snake_case)]
pub fn Reply<'a>(cx: Scope<'a, ReplyProps<'a>>) -> Element<'a> {
    let remote = cx.props.remote.unwrap_or_default();

    let has_attachments = cx
        .props
        .attachments
        .as_ref()
        .map(|v| !v.is_empty())
        .unwrap_or(false);

    let attachment_list = cx.props.attachments.as_ref().map(|vec| {
        vec.iter().map(|file| {
            let key = file.id();
            rsx!(FileEmbed {
                key: "{key}",
                filename: file.name(),
                filesize: file.size(),
                with_download_button: false,
                remote: remote,
                on_press: move |_| {},
            })
        })
    });

    cx.render(rsx! (
        div {
            class: "inline-reply",
            aria_label: "inline-reply",
            Label {
                text: cx.props.label.clone(),
            },
            Button {
                small: true,
                aria_label: "close-reply".into(),
                appearance: Appearance::Secondary,
                icon: icons::outline::Shape::XMark,
                onpress: move |_| cx.props.onclose.call(()),
            },
            div {
                class: "content",
                aria_label: "content",
                remote.then(|| rsx!(&cx.props.children)),
                p {
                    class: {
                        format_args!("reply-text message {}", if remote { "remote" } else { "" })
                    },
                    aria_label: {
                        format_args!("reply-text-message{}", if remote { "-remote" } else { "" })
                    },
                    "{cx.props.message}"
                    has_attachments.then(|| {
                        rsx!(
                            attachment_list.map(|list| {
                                rsx!( list )
                            })
                        )
                    })
                }
                (!remote).then(|| rsx!(&cx.props.children)),
            }

        }
    ))
}

#[allow(non_snake_case)]
pub fn Chatbar<'a>(cx: Scope<'a, Props<'a>>) -> Element<'a> {
    let controlled_input_id = &cx.props.id;
    cx.render(rsx!(
        div {
            class: "chatbar disable-select",
            cx.props.with_replying_to.as_ref(),
            cx.props.with_file_upload.as_ref(),
            textarea::Input {
                key: "{controlled_input_id}",
                id: controlled_input_id.clone(),
                loading: cx.props.loading.unwrap_or_default(),
                placeholder: cx.props.placeholder.clone(),
                ignore_focus: cx.props.ignore_focus,
                value: if cx.props.is_disabled { get_local_text("messages.not-friends")} else { cx.props.value.clone().unwrap_or_default()},
                onchange: move |(v, _)| cx.props.onchange.call(v),
                onreturn: move |(v, is_valid, _)| {
                    if is_valid {
                        cx.props.onreturn.call(v);
                    }
                },
                is_disabled: cx.props.is_disabled,
            },
            cx.props.extensions.as_ref(),
            div {
                class: "controls",
                cx.props.controls.as_ref()
            }
        }
    ))
}
