use dioxus::prelude::*;
use fermi::use_atom_ref;
use ui_kit::{elements::{switch::Switch, Appearance, button::Button}, icons::Icon};

use crate::{components::settings::SettingSection, STATE};

#[allow(non_snake_case)]
pub fn DeveloperSettings(cx: Scope) -> Element {
    let state = use_atom_ref(&cx, STATE);

    cx.render(rsx!(
        div {
            id: "settings-developer",
            SettingSection {
                section_label: "Developer Mode".into(),
                section_description: "Enabling developer mode adds logging and displays helpful debug information on the UI.".into(),
                Switch {
                    
                }
            },
            SettingSection {
                section_label: "Open Codebase".into(),
                section_description: "Opens the codebase in your default web browser.".into(),
                Button {
                    text: "Open Codebase".into(),
                    appearance: Appearance::Secondary,
                    icon: Icon::CodeBracketSquare,
                    onpress: |_| {
                        let _ = open::that("https://github.com/Satellite-im/Uplink-UI_Kit/tree/main/uplink_skeleton");
                    }
                }
            },
            SettingSection {
                section_label: "Open Cache".into(),
                section_description: "Open the cache in your default file browser.".into(),
                Button {
                    text: "Open Folder".into(),
                    appearance: Appearance::Secondary,
                    icon: Icon::FolderOpen,
                    onpress: |_| {
                        let cache_path = dirs::home_dir()
                            .unwrap_or_default()
                            .join(".uplink/")
                            .into_os_string()
                            .into_string()
                            .unwrap_or_default();
                        let _ = opener::open(&cache_path);
                    }
                }
            },
            SettingSection {
                section_label: "Compress & Download Cache".into(),
                section_description: "For debugging with other developers, you can compress your cache to zip and share it. Don't do this if this is a real account you use.".into(),
                Button {
                    text: "Compress".into(),
                    appearance: Appearance::Secondary,
                    icon: Icon::ArchiveBoxArrowDown,
                    onpress: |_| {
                    }
                }
            },
            SettingSection {
                section_label: "Clear Cache".into(),
                section_description: "Reset your account, basically.".into(),
                Button {
                    text: "Clear".into(),
                    appearance: Appearance::Danger,
                    icon: Icon::Trash,
                    onpress: move |_| {
                        state.write().clear();
                    }
                }
            }
        }
    ))
}
