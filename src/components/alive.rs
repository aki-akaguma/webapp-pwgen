use crate::backends::alive;
use dioxus::prelude::*;

/// the component of checking alive the backend
#[component]
pub fn Alive() -> Element {
    let mut alive_sig = use_signal(String::new);
    use_future(move || async move {
        alive().await.unwrap();
        alive_sig.set("".to_string());
    });
    rsx! {
        div { class: "alive" }
    }
}
