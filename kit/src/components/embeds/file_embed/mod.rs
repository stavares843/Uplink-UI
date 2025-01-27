use std::ffi::OsStr;
use std::path::PathBuf;

use crate::elements::button::Button;
use crate::elements::Appearance;
use common::icons::outline::Shape as Icon;
use common::icons::Icon as IconElement;
use common::is_audio;
use common::is_video;
use common::return_correct_icon;
use common::utils::local_file_path::get_fixed_path_to_load_local_file;
use common::STATIC_ARGS;
use dioxus_html::input_data::keyboard_types::Modifiers;

use dioxus::prelude::*;

use humansize::format_size;
use humansize::DECIMAL;
use mime::IMAGE_JPEG;
use mime::IMAGE_PNG;
use mime::IMAGE_SVG;
use warp::constellation::Progression;

#[derive(Props)]
pub struct Props<'a> {
    // The filename of the file
    filename: String,

    // The local filepath of the file
    filepath: Option<PathBuf>,

    // The size of the file in bytes
    filesize: Option<usize>,

    // The type of the file (e.g. "PDF", "JPEG")
    kind: Option<String>,

    // Whether the file is coming from a remote user, or we sent it.
    remote: Option<bool>,

    // The icon to use to represent the file
    attachment_icon: Option<Icon>,

    // used for the button. defaults to a download icon
    button_icon: Option<Icon>,

    // The thumbnail for the file. If existent
    thumbnail: Option<String>,

    // Whether the file is coming from attachments or not
    is_from_attachments: Option<bool>,

    big: Option<bool>,

    // used to show download button, if nothing is passed, button will render
    with_download_button: Option<bool>,

    download_pending: Option<bool>,

    // called shen the icon is clicked
    on_press: EventHandler<'a, Option<PathBuf>>,

    progress: Option<&'a Progression>,
}

#[allow(non_snake_case)]
pub fn FileEmbed<'a>(cx: Scope<'a, Props<'a>>) -> Element<'a> {
    //log::trace!("rendering file embed: {}", cx.props.filename);
    let file_extension = std::path::Path::new(&cx.props.filename)
        .extension()
        .and_then(OsStr::to_str)
        .map(|s| {
            if s.len() > 6 {
                format!(".{}...", &s[0..4])
            } else {
                format!(".{}", s)
            }
        })
        .unwrap_or_default();
    let file_extension_is_empty = file_extension.is_empty();
    let filename = &cx.props.filename;
    let download_pending = cx.props.download_pending.unwrap_or(false);
    let is_from_attachments = cx.props.is_from_attachments.unwrap_or(false);
    let btn_icon = if !download_pending {
        cx.props.button_icon.unwrap_or(Icon::ArrowDown)
    } else {
        Icon::DocumentArrowDown // TODO: Should this be an animated download icon? What is the purpose of this?
    };

    let with_download_button = if let Some(with_download_button) = cx.props.with_download_button {
        with_download_button
    } else if let Some(is_from_attachments) = cx.props.is_from_attachments {
        is_from_attachments
    } else {
        false
    };

    let is_pending = cx.props.progress.is_some();

    let mut file_size_pending = String::new();

    let perc = if let Some(p) = cx.props.progress {
        match p {
            Progression::CurrentProgress {
                name: _,
                current,
                total,
            } => match total {
                Some(size) => {
                    file_size_pending
                        .push_str(&format_args!("{}", format_size(*size, DECIMAL)).to_string());
                    if *current > 0 && *size > 0 {
                        current * 100 / size
                    } else {
                        0
                    }
                }
                None => 0,
            },
            Progression::ProgressComplete { name: _, total } => {
                if let Some(size) = total {
                    file_size_pending
                        .push_str(&format_args!("{}", format_size(*size, DECIMAL)).to_string());
                };
                100
            }
            Progression::ProgressFailed {
                name: _,
                last_size: _,
                error: _,
            } => {
                file_size_pending.push_str("Failed");
                0
            }
        }
    } else {
        0
    };

    // show one of the 3:
    // kind
    // kind - size
    // size
    let file_description = match cx.props.filesize {
        Some(filesize) => {
            let size = format_size(filesize, DECIMAL);
            match cx.props.kind.as_ref() {
                Some(kind) => format!("{kind} - {size}"),
                None => size,
            }
        }
        None => {
            if file_size_pending.is_empty() {
                cx.props.kind.clone().unwrap_or_default()
            } else {
                file_size_pending
            }
        }
    };
    let remote = cx.props.remote.unwrap_or_default();
    let thumbnail = cx.props.thumbnail.clone().unwrap_or_default();
    let has_thumbnail = !thumbnail.is_empty();
    let file_name_with_extension = cx.props.filename.to_string();
    let temp_dir = STATIC_ARGS
        .temp_files
        .join(file_name_with_extension.clone());
    let is_video_or_audio =
        is_video(&file_name_with_extension) || is_audio(&file_name_with_extension);
    let is_video = is_video(&file_name_with_extension);

    cx.render(rsx! (
        div {
            class: {
                format_args!(
                    "file-embed {} {}",
                    if remote {
                        "remote"
                    } else { "" },
                    if cx.props.big.unwrap_or_default() {
                        "big"
                    } else { "" }
                )
            },
            aria_label: {
                format_args!(
                    "file-embed{}",
                    if remote {
                        "-remote"
                    } else { "" }
                )
            },
                rsx!(
                    div {
                        class: format_args!("{}", if has_thumbnail {""} else {"icon"}),
                        aria_label: "file-icon",
                        if has_thumbnail {
                            rsx!(
                                div {
                                    class: "image-container",
                                    aria_label: "message-image-container",
                                    img {
                                        aria_label: "message-image",
                                        onclick: move |mouse_event_data: Event<MouseData>|
                                        if mouse_event_data.modifiers() != Modifiers::CONTROL && !is_from_attachments {
                                            cx.props.on_press.call(Some(temp_dir.clone()));
                                        },
                                        class: format_args!(
                                            "image {} expandable-image",
                                            if cx.props.big.unwrap_or_default() {
                                                "big"
                                            } else { "" }
                                        ),
                                        src: "{thumbnail}",
                                    },
                                    show_download_or_minus_button_if_enabled(cx, with_download_button, btn_icon),
                                   }
                                    )
                        } else if let Some(filepath) = cx.props.filepath.clone() {
                            let is_image_or_video = is_image(filename.clone()) || is_video;
                            if is_image_or_video && filepath.exists() {
                                let fixed_path = get_fixed_path_to_load_local_file(filepath.clone());
                                rsx!(img {
                                    class: "image-preview-modal",
                                    aria_label: "image-preview-modal",
                                    src: "{fixed_path}",
                                    onclick: move |e| e.stop_propagation(),
                                })
                            } else {
                                rsx!(
                                    div {
                                        height: "60px",
                                        width: "60px",
                                        margin: "30px 0",
                                        IconElement {
                                            icon: cx.props.attachment_icon.unwrap_or(return_correct_icon(&file_name_with_extension.clone()))
                                        }
                                        if !file_extension_is_empty {
                                            rsx!( label {
                                                class: "file-embed-type",
                                                "{file_extension}"
                                            })
                                        }
                                    }
                                    )
                            }
                        } else {
                            rsx!(
                                div {
                                    class: "document-container",
                                    height: "60px",
                                    onclick: move |mouse_event_data: Event<MouseData>| {
                                        if mouse_event_data.modifiers() != Modifiers::CONTROL && is_video_or_audio && !is_from_attachments {
                                            cx.props.on_press.call(Some(temp_dir.clone()));
                                        }
                                    },
                                    IconElement {
                                        icon: cx.props.attachment_icon.unwrap_or(return_correct_icon(&file_name_with_extension.clone()))
                                    }
                                    if !file_extension_is_empty {
                                        rsx!( label {
                                            class: "file-embed-type",
                                            "{file_extension}"
                                        })
                                    }
                                    if !is_from_attachments {
                                        rsx!( div {
                                            class: "button-position",
                                            show_download_or_minus_button_if_enabled(cx, with_download_button, btn_icon),
                                        })
                                    }
                                }
                                )
                        }
                    }
                    div {
                            class: "file-info",
                            width: "100%",
                            aria_label: "file-info",
                            p {
                                class: "name",
                                aria_label: "file-name",
                                "{filename}"
                            },
                            p {
                                class: "meta",
                                aria_label: "file-meta",
                                "{file_description}"
                            }
                        },
                        if !has_thumbnail && is_from_attachments {
                            rsx!(show_download_or_minus_button_if_enabled(cx, with_download_button, btn_icon))
                        }
                    if is_pending {
                        rsx!(div {
                            class: "upload-bar",
                            div {
                                class: "upload-progress",
                                style: format_args!("width: {}%", perc)
                            }
                        })
                    }
                )
        }
    ))
}

fn is_image(filename: String) -> bool {
    let parts_of_filename: Vec<&str> = filename.split('.').collect();
    let mime = match parts_of_filename.last() {
        Some(m) => match *m {
            "png" => IMAGE_PNG.to_string(),
            "jpg" => IMAGE_JPEG.to_string(),
            "jpeg" => IMAGE_JPEG.to_string(),
            "svg" => IMAGE_SVG.to_string(),
            &_ => "".to_string(),
        },
        None => "".to_string(),
    };

    if mime.is_empty() {
        return false;
    }

    true
}

fn show_download_or_minus_button_if_enabled<'a>(
    cx: Scope<'a, Props<'a>>,
    with_download_button: bool,
    btn_icon: common::icons::outline::Shape,
) -> Element<'a> {
    if with_download_button {
        cx.render(rsx!(
            div {
                id: "file-embed-action-button",
                Button {
                    icon: btn_icon,
                    appearance: Appearance::Primary,
                    aria_label: "attachment-button".into(),
                    onpress: move |_| cx.props.on_press.call(None),
                }
            }
        ))
    } else {
        None
    }
}
