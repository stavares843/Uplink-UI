use dioxus::prelude::*;
use fermi::use_atom_ref;
use fluent_templates::Loader;
use ui_kit::{elements::{switch::Switch, select::Select }};

use crate::{components::settings::SettingSection, utils::language::{change_language, get_available_languages, APP_LANG}, state::Action, LOCALES, STATE};

#[allow(non_snake_case)]
pub fn GeneralSettings(cx: Scope) -> Element {    
    let state = use_atom_ref(&cx, STATE);
    let initial_lang_value = state.read().settings.language.clone();
    let app_lang = &*APP_LANG.read();
    let app_language_text = LOCALES
    .lookup(app_lang, "settings.general-app-language")
    .unwrap_or_default().clone();

    let change_language_text = LOCALES
    .lookup(app_lang, "settings.general-change-language")
    .unwrap_or_default().clone();

    cx.render(rsx!(
        div {
            id: "settings-general",
            SettingSection {
                section_label: "Splash Screen".into(),
                section_description: "Disabling the splash screen could speed up load times.".into(),
                Switch {
          
                }
            },
            SettingSection {
                section_label: app_language_text,
                section_description: change_language_text,
                Select {
                    initial_value: initial_lang_value,
                    options: get_available_languages(),
                    onselect: move |value| {
                        let new_app_lang = change_language(value);
                        state.write().mutate(Action::SetLanguage(new_app_lang));
                    }
                }
            },
        }
    ))
}
