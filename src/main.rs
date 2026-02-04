use async_sleep_aki::delayed_call;
use dioxus::prelude::*;
#[cfg(feature = "desktop")]
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

    dioxus::LaunchBuilder::new()
        // Set the desktop config
        .with_cfg(desktop! {
            Config::default().with_menu(None).with_window(
                WindowBuilder::new()
                    .with_maximized(false)
                    .with_title("Password generator"),
            )
        })
        // Set the server config only if we are building the server target
        .with_cfg(server_only! {
            ServeConfig::builder()
                // Enable incremental rendering
                .incremental(
                    dioxus::server::IncrementalRendererConfig::new()
                        // Store static files in the public directory where other static assets
                        // like wasm are stored
                        .static_dir(
                            std::env::current_exe()
                                .unwrap()
                                .parent()
                                .unwrap()
                                .join("public")
                        )
                        // Don't clear the public folder on every build. The public folder has
                        // other files including the wasm
                        // binary and static assets required for the app to run
                        .clear_cache(false)
                )
                .enable_out_of_order_streaming()
        })
        .launch(|| {
            rsx! {
                Router::<Route> {}
            }
        });
}

#[server(endpoint = "static_routes", output = server_fn::codec::Json)]
async fn static_routes() -> Result<Vec<String>, ServerFnError> {
    // The `Routable` trait has a `static_routes` method that returns all static routes in the enum
    Ok(Route::static_routes()
        .iter()
        .map(ToString::to_string)
        .collect())
}

#[derive(Routable, Clone, PartialEq)]
enum Route {
    #[route("/")]
    Index,
    #[route("/pre")]
    Pre,
    #[route("/app")]
    App,
}

#[component]
fn Index() -> Element {
    use_future(move || async move {
        navigator().replace(Route::Pre {});
    });
    rsx! {}
}

const PRE_STYLE: Asset = asset!("/assets/pre/stylesheet.css");
const PRE_LOADING: Asset = asset!("/assets/pre/loading.css");
const PRE_BGIMAGE: Asset = asset!("/assets/pre/background-image.jpg");
const PRE_BGIMAGE_AVIF: Asset = asset!(
    "/assets/pre/background-image.jpg",
    ImageAssetOptions::new().with_format(ImageFormat::Avif)
);
const PRE_OVERLAY: Asset = asset!("/assets/pre/overlay.svg");

#[component]
fn Pre() -> Element {
    let mut is_loading = use_signal(|| false);
    /*
        document::Stylesheet { href: CSS_BOOTSTRAP }
        document::Script { src: "https://cdn.tailwindcss.com" }
    */
    rsx! {
        document::Stylesheet { href: CSS_MAIN }
        document::Stylesheet { href: PRE_STYLE }
        document::Stylesheet { href: PRE_LOADING }
        div {
            id: "pre-shell",
            onclick: move |_evt| {
                is_loading.set(true);
                spawn(
                    delayed_call(
                        500,
                        async move {
                            navigator().replace(Route::App {});
                        },
                    ),
                );
            },
            div { id: "menter",
                picture {
                    source { src: PRE_BGIMAGE_AVIF, r#type: "image/avif" }
                    img { src: PRE_BGIMAGE, alt: "image", width: "360" }
                }
                div { class: "overlay-string",
                    img { src: PRE_OVERLAY, width: "360", height: "120" }
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
}

const FAVICON: Asset = asset!("/assets/favicon.ico");
const CSS_TAILWIND: Asset = asset!("/assets/css/tailwind.css");
const CSS_MAIN: Asset = asset!("/assets/css/main.css");

/// the component of dioxus `App`
#[component]
fn App() -> Element {
    /*
        document::Script { src: "https://cdn.tailwindcss.com" }
    */
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
        document::Stylesheet { href: CSS_MAIN }
        document::Stylesheet { href: CSS_TAILWIND }

        Alive {}
        Info {}
        div {
            id: "app-shell",
            class: "flex flex-col w-full min-w-[260px] max-w-[700px] min-h-screen mx-auto relative overflow-x-hidden bg-[#add8e6]",
            Home {}
            Version {}
        }
    }
}
