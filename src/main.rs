use async_sleep_aki::delayed_call;
use dioxus::prelude::*;
#[cfg(all(not(debug_assertions), feature = "desktop"))]
use dioxus_desktop::{Config, WindowBuilder};

use components::*;
use views::Home;

mod backends;
mod components;
mod views;

fn main() {
    // You can set the ports and IP manually with env vars:
    //   server launch:
    //     IP="0.0.0.0" PORT=8080 ./server

    // You can supplement panic on  firefox browser.
    #[cfg(feature = "web")]
    console_error_panic_hook::set_once();

    #[cfg(not(debug_assertions))]
    let level = dioxus_logger::tracing::Level::INFO;
    #[cfg(debug_assertions)]
    let level = dioxus_logger::tracing::Level::DEBUG;
    dioxus_logger::init(level).expect("failed to init logger");

    // In the case of release desktop and release mobile,
    // connect backend calls to public api
    #[cfg(not(debug_assertions))]
    #[cfg(any(feature = "desktop", feature = "mobile"))]
    {
        // Specify the URL that previously delpoyed the public webapp.
        // This webapp was created with `dx bundle --web`.
        let backend_url = "https://aki.omusubi.org/pwgen";
        dioxus_fullstack::set_server_url(backend_url);
    }

    // In the case of only release desktop, set a window title
    #[cfg(all(not(debug_assertions), feature = "desktop"))]
    dioxus::LaunchBuilder::new()
        .with_cfg(
            Config::default().with_menu(None).with_window(
                WindowBuilder::new()
                    .with_maximized(false)
                    .with_title("Password generator"),
            ),
        )
        //.launch(base_route);
        .launch(App);

    // In the other case, simple launch app
    #[cfg(any(debug_assertions, not(feature = "desktop")))]
    dioxus::launch(base_route);
}

#[cfg(not(feature = "mobile"))]
#[derive(Routable, Clone, PartialEq)]
enum Route {
    #[route("/app")]
    App,
    #[route("/")]
    Index,
    #[route("/pre")]
    Pre,
}

#[cfg(feature = "mobile")]
#[derive(Routable, Clone, PartialEq)]
enum Route {
    #[route("/")]
    App,
}

#[component]
fn base_route() -> Element {
    rsx! {
        Router::<Route> {}
    }
}

#[component]
fn Index() -> Element {
    #[cfg(all(feature = "web", debug_assertions))]
    let JS: &str = "window.location.replace('pre');";
    #[cfg(not(all(feature = "web", debug_assertions)))]
    let JS: &str = "window.location.replace('pre.html');";
    use_future(move || async move {
        //dioxus_logger::tracing::info!("INDEX:1");
        let _r = document::eval(JS).await.unwrap();
        //dioxus_logger::tracing::info!("INDEX:2:{_r}");
    });
    rsx! {}
}

#[component]
fn Pre() -> Element {
    let mut is_loading = use_signal(|| false);
    rsx! {
        document::Link { rel: "stylesheet", href: "pre-res/stylesheet.css" }
        document::Link { rel: "stylesheet", href: "pre-res/loading.css" }
        div {
            id: "menter",
            onclick: move |_evt| {
                is_loading.set(true);
                #[cfg(debug_assertions)]
                let js = r#"window.location.replace('app');"#;
                #[cfg(not(debug_assertions))]
                let js = r#"window.location.replace('https://aki.omusubi.org/pwgen/app');"#;
                spawn(
                    delayed_call(
                        500,
                        async move {
                            let _ = document::eval(js).await;
                        },
                    ),
                );
            },
            div {
                img { src: "pre-res/overlay.svg", width: "360", height: "720" }
            }
            div { id: "sentence",
                p { id: "title", "Password generator" }
                p { id: "prompt", "Tap to Start" }
            }
            if is_loading() {
                div { class: "overlay",
                    div { class: "spinner-outer",
                        div { class: "spinner" }
                    }
                }
            }
        }
    }
}

const FAVICON: Asset = asset!("/assets/favicon.ico");

/// the component of dioxus `App`
#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "preconnect", href: "https://fonts.googleapis.com" }
        document::Link {
            rel: "preconnect",
            href: "https://fonts.gstatic.com",
            crossorigin: "",
        }
        document::Link {
            rel: "stylesheet",
            href: concat!(
                "https://fonts.googleapis.com/css2?",
                "family=Inter:wght@400;500;700&display=swap",
            ),
        }

        MyStyle {}
        Alive {}
        Info {}
        Home {}
        Version {}
    }
}

/// the component of `main` style sheet
#[cfg(not(feature = "inline_style"))]
#[component]
fn MyStyle() -> Element {
    rsx! {
        document::Stylesheet { href: asset!("/assets/css/bootstrap.min.css") }
        document::Stylesheet { href: asset!("/assets/css/main.css") }
    }
}

#[cfg(feature = "inline_style")]
#[component]
fn MyStyle() -> Element {
    const BOOTSTRAP_CSS: &str = const_css_minify::minify!("../assets/css/bootstrap.min.css");
    const MAIN_CSS: &str = const_css_minify::minify!("../assets/css/main.css");
    rsx! {
        style { "{MAIN_CSS}" }
        style { "{BOOTSTRAP_CSS}" }
    }
}
