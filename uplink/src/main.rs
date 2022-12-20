use std::fs;
use std::sync::RwLock;

use dioxus::prelude::*;
use dioxus_desktop::tao::dpi::LogicalSize;
use dioxus_desktop::tao::menu::AboutMetadata;
use dioxus_desktop::tao::platform::macos::WindowBuilderExtMacOS;
use dioxus_desktop::{tao, Config};

use state::State;
use tao::menu::{MenuBar as Menu, MenuItem};
use tao::window::WindowBuilder;
use ui_kit::icons::IconElement;
use ui_kit::{components::nav::Route as UIRoute, icons::Icon};

use ui_kit::STYLE as UIKIT_STYLES;
use utils::language::APP_LANG;

use crate::components::media::popout_player::PopoutPlayer;
use crate::layouts::files::FilesLayout;
use crate::layouts::friends::FriendsLayout;
use crate::layouts::settings::settings::SettingsLayout;
use crate::{components::chat::RouteInfo, layouts::chat::ChatLayout};
use dioxus_router::*;
pub const APP_STYLE: &str = include_str!("./compiled_styles.css");
use fermi::prelude::*;
pub mod components;
pub mod config;
pub mod layouts;
pub mod state;
pub mod testing;
pub mod utils;

use fluent_templates::{static_loader, Loader};

pub static STATE: AtomRef<State> = |_| State::load().unwrap();

static_loader! {
    static LOCALES = {
        locales: "./locales",
        fallback_language: "en-US",
        // Removes unicode isolating marks around arguments, you typically
        // should only set to false when testing.
        customise: |bundle| bundle.set_use_isolating(false),
    };
}

fn main() {
    // Initalized the cache dir if needed
    let cache_path = dirs::home_dir()
        .unwrap_or_default()
        .join(".uplink/")
        .into_os_string()
        .into_string()
        .unwrap_or_default();

    let _ = fs::create_dir_all(&cache_path);

    let mut main_menu = Menu::new();
    let mut app_menu = Menu::new();
    let mut edit_menu = Menu::new();
    let mut window_menu = Menu::new();

    app_menu.add_native_item(MenuItem::Quit);
    app_menu.add_native_item(MenuItem::About(
        String::from("Uplink"),
        AboutMetadata::default(),
    ));
    // add native shortcuts to `edit_menu` menu
    // in macOS native item are required to get keyboard shortcut
    // to works correctly
    edit_menu.add_native_item(MenuItem::Undo);
    edit_menu.add_native_item(MenuItem::Redo);
    edit_menu.add_native_item(MenuItem::Separator);
    edit_menu.add_native_item(MenuItem::Cut);
    edit_menu.add_native_item(MenuItem::Copy);
    edit_menu.add_native_item(MenuItem::Paste);
    edit_menu.add_native_item(MenuItem::SelectAll);

    window_menu.add_native_item(MenuItem::Minimize);
    window_menu.add_native_item(MenuItem::Zoom);
    window_menu.add_native_item(MenuItem::Separator);
    window_menu.add_native_item(MenuItem::ShowAll);
    window_menu.add_native_item(MenuItem::EnterFullScreen);
    window_menu.add_native_item(MenuItem::Separator);
    window_menu.add_native_item(MenuItem::CloseWindow);

    main_menu.add_submenu("Uplink", true, app_menu);
    main_menu.add_submenu("Edit", true, edit_menu);
    main_menu.add_submenu("Window", true, window_menu);

    let title = LOCALES
        .lookup(&APP_LANG.read(), "uplink")
        .unwrap_or_default();

    let mut window = WindowBuilder::new()
        .with_title(title)
        .with_resizable(true)
        .with_inner_size(LogicalSize::new(950.0, 600.0))
        .with_min_inner_size(LogicalSize::new(300.0, 500.0));

    #[cfg(target_os = "macos")]
    {
        window = window
            .with_has_shadow(true)
            .with_title_hidden(true)
            .with_transparent(true)
            .with_fullsize_content_view(true)
            .with_titlebar_transparent(true)
        // .with_movable_by_window_background(true)
    }

    let config = Config::default();

    dioxus_desktop::launch_cfg(app, config.with_window(window.with_menu(main_menu)))
}

fn app(cx: Scope) -> Element {
    use_init_atom_root(cx);
    /*let state = match State::load() {
        Ok(s) => s,
        Err(_) => State::default(),
    };*/

    //let _ = use_shared_state_provider(&cx, || STATE);

    let state = use_atom_ref(&cx, STATE);
    //let state = RwLock::new(state_ns);

    let user_lang_saved = state.read().settings.language.clone();
    utils::language::change_language(user_lang_saved);

    let pending_friends = state.read().friends.incoming_requests.len();

    let chat_route = UIRoute {
        to: "/",
        name: "Chat".to_owned(),
        icon: Icon::ChatBubbleBottomCenterText,
        ..UIRoute::default()
    };
    let settings_route = UIRoute {
        to: "/settings",
        name: "Settings".to_owned(),
        icon: Icon::Cog,
        ..UIRoute::default()
    };
    let friends_route = UIRoute {
        to: "/friends",
        name: "Friends".to_owned(),
        icon: Icon::Users,
        with_badge: if pending_friends > 0 {
            Some(pending_friends.to_string())
        } else {
            None
        },
        loading: None,
    };
    let files_route = UIRoute {
        to: "/files",
        name: "Files".to_owned(),
        icon: Icon::Folder,
        ..UIRoute::default()
    };
    let routes = vec![
        chat_route.clone(),
        files_route.clone(),
        //friends_route.clone(),
        settings_route.clone(),
    ];

    cx.render(rsx! (
        style { "{UIKIT_STYLES} {APP_STYLE}" },
        div {
            id: "app-wrap",
            div {
                id: "pre-release",
                IconElement {
                    icon: Icon::Beaker,
                },
                p {
                    "Pre-release"
                }
            },
            state.read().ui.popout_player.then(|| rsx!(
                PopoutPlayer {}
            )),
            Router {
                Route {
                    to: "/",
                    ChatLayout {
                        route_info: RouteInfo {
                            routes: routes.clone(),
                            active: chat_route.clone(),
                        }
                    }
                },
                Route {
                    to: "/settings",
                    SettingsLayout {
                        route_info: RouteInfo {
                            routes: routes.clone(),
                            active: settings_route.clone(),
                        }
                    }
                },
                Route {
                    to: "/friends",
                    FriendsLayout {
                        route_info: RouteInfo {
                            routes: routes.clone(),
                            active: friends_route.clone(),
                        }
                    }
                },
                Route {
                    to: "/files",
                    FilesLayout {
                        route_info: RouteInfo {
                            routes: routes.clone(),
                            active: files_route.clone(),
                        }
                    }
                }
            }
        }
    ))
}
