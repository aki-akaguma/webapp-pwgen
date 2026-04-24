use browserinfocm::BrowserInfoCm;
use browserinfocm::BrowserInfoState;
use dioxus::prelude::*;

/// the component of browser information
#[component]
pub fn Info() -> Element {
    // Signals for storing data gathered by BrowserInfoCm.
    let state_sig = use_signal(BrowserInfoState::default);

    rsx! {
        BrowserInfoCm { state: state_sig }
    }
}
