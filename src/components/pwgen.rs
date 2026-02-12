use dioxus::prelude::*;

const CSS_PWGEN: Asset = asset!("/assets/css/pwgen.css");
const IMG_APP_TITLE: Asset = asset!(
    "/assets/app.png",
    ImageAssetOptions::new()
        .with_size(ImageSize::Manual {
            width: 140,
            height: 140
        })
        .with_format(ImageFormat::Webp)
);

/// the component of `tap tap tap beat`
#[component]
pub fn Pwgen() -> Element {
    let title1_s = "Password Generator";
    let title2_s = "Create secure passwords tailored to your needs.";
    let passwords = use_signal(Vec::<String>::new);
    rsx! {
        document::Stylesheet { href: CSS_PWGEN }
        div { id: "pwgen", class: "container mx-auto px-4",
            div { class: "flex items-center gap-4",
                img { src: IMG_APP_TITLE, width: "70px" }
                div { id: "pwgen-header", class: "text-center my-4",
                    h1 { class: "text-5xl app-title app-title-bold", "{title1_s}" }
                    p { class: "text-lg app-title mt-2", "{title2_s}" }
                }
            }
            div { class: "bg-gray-800 border border-gray-600 rounded-xl shadow-2xl",
                div { class: "p-6",
                    PwgenContentInputs { passwords }
                }
            }
            if !passwords.is_empty() {
                PwgenContentGeneratedPasswordsElement { passwords }
            }
        }
    }
}

#[component]
fn PwgenContentInputs(passwords: Signal<Vec<String>>) -> Element {
    let mut password_length = use_signal(|| 15);
    let mut include_lowercase = use_signal(|| true);
    let mut include_uppercase = use_signal(|| true);
    let mut include_numbers = use_signal(|| true);
    let mut include_symbols = use_signal(|| false);
    // generate passwords on initial render
    use_future(move || async move {
        let params = PwgenParams {
            include_lowercase: include_lowercase(),
            include_uppercase: include_uppercase(),
            include_numbers: include_numbers(),
            include_symbols: include_symbols(),
        };
        let new_passwords = generate_passwords(password_length(), params);
        passwords.set(new_passwords);
    });
    rsx! {
        div { class: "flex flex-wrap gap-4",
            div { class: "w-full",
                label { class: "app-range-label", r#for: "passwordLength",
                    "Password Length: {password_length}"
                }
                input {
                    id: "passwordLength",
                    r#type: "range",
                    class: "app-range-bar",
                    min: "4",
                    max: "32",
                    value: "{password_length}",
                    oninput: move |evt| {
                        let v = evt.value().parse::<i32>().unwrap();
                        password_length.set(v);
                    },
                }
            }
            div { class: "w-full",
                div { class: "flex flex-wrap",
                    PwgenContentCB {
                        id: "includeLowercase",
                        label1: "Lowercase",
                        label2: "(a-z)",
                        is_checked: include_lowercase(),
                        onchange: move |_evt| {
                            include_lowercase.set(!include_lowercase());
                        },
                    }
                    PwgenContentCB {
                        id: "includeUppercase",
                        label1: "Uppercase",
                        label2: "(A-Z)",
                        is_checked: include_uppercase(),
                        onchange: move |_evt| {
                            include_uppercase.set(!include_uppercase());
                        },
                    }
                    PwgenContentCB {
                        id: "includeNumbers",
                        label1: "Numbers",
                        label2: "(0-9)",
                        is_checked: include_numbers(),
                        onchange: move |_evt| {
                            include_numbers.set(!include_numbers());
                        },
                    }
                    PwgenContentCB {
                        id: "includeSymbols",
                        label1: "Symbols",
                        label2: "(!@#$...)",
                        is_checked: include_symbols(),
                        onchange: move |_evt| {
                            include_symbols.set(!include_symbols());
                        },
                    }
                }
            }
        }
        div { class: "grid mt-4",
            button {
                class: "app-gen-btn",
                onclick: move |_evt| {
                    let params = PwgenParams {
                        include_lowercase: include_lowercase(),
                        include_uppercase: include_uppercase(),
                        include_numbers: include_numbers(),
                        include_symbols: include_symbols(),
                    };
                    let new_passwords = generate_passwords(password_length(), params);
                    passwords.set(new_passwords);
                },
                "Generate Passwords"
            }
        }
    }
}

#[component]
fn PwgenContentCB(
    id: String,
    label1: String,
    label2: String,
    is_checked: bool,
    onchange: EventHandler<FormEvent>,
) -> Element {
    rsx! {
        div { class: "md:w-1/2 w-full mb-3",
            div { class: "flex items-center",
                div { class: "relative inline-block w-11 h-6 mr-2",
                    input {
                        id: id.clone(),
                        class: "peer app-switch-bar",
                        r#type: "checkbox",
                        r#role: "switch",
                        checked: "{is_checked}",
                        onchange: move |evt| onchange.call(evt),
                    }
                    label { class: "app-switch-notch", r#for: id.clone() }
                }
                label { class: "app-switch-label", r#for: id.clone(),
                    "{label1}"
                    label { class: "app-switch-label-mono", r#for: id, " {label2}" }
                }
            }
        }
    }
}

#[component]
fn PwgenContentGeneratedPasswordsElement(passwords: ReadSignal<Vec<String>>) -> Element {
    let title_s = "Generated Passwords";
    rsx! {
        div { class: "mt-5",
            h2 { class: "text-center mb-4 text-2xl app-title app-title-bold", "{title_s}" }
            ul { class: "border-t border-gray-600",
                PwgenContentGeneratedPasswordsElement2 { passwords }
            }
        }
    }
}

#[component]
fn PwgenContentGeneratedPasswordsElement2(passwords: ReadSignal<Vec<String>>) -> Element {
    let mut copied_index = use_signal(|| Option::<i32>::None);
    rsx! {
        for (i , password) in passwords().into_iter().enumerate() {
            {
                let i = i as i32;
                let is_current = if let Some(ci) = copied_index() { ci == i } else { false };
                rsx! {
                    PwgenContentGeneratedPassword {
                        is_current,
                        password: password.clone(),
                        onclick: move |_evt| {
                            let password = password.clone();
                            async move {
                                let _ = copy_to_clipboard(password).await;
                                copied_index.set(Some(i));
                                spawn(
                                    async_sleep_aki::delayed_call(
                                        2000,
                                        async move {
                                            copied_index.set(None);
                                        },
                                    ),
                                );
                            }
                        },
                    }
                }
            }
        }
    }
}

#[component]
fn PwgenContentGeneratedPassword(
    is_current: bool,
    password: String,
    onclick: EventHandler<MouseEvent>,
) -> Element {
    let btn_class = if is_current {
        "bg-cyan-700 hover:bg-cyan-900"
    } else {
        "bg-cyan-500 hover:bg-cyan-700"
    };
    let btn_label = if is_current { "Copied!" } else { "Copy" };
    rsx! {
        li { class: "flex justify-between items-center py-3 border-b border-gray-600",
            span { class: "app-password-text", "{password}" }
            button {
                class: "{btn_class} app-copy-btn",
                onclick: move |evt| onclick.call(evt),
                "{btn_label}"
            }
        }
    }
}

async fn copy_to_clipboard(password: String) -> Result<()> {
    let js = format!("{{navigator.clipboard.writeText('{}');}}", password);
    let _v = document::eval(&js).await?;
    Ok(())
}

fn generate_passwords(len: i32, pa: PwgenParams) -> Vec<String> {
    let mut new_passwords: Vec<String> = Vec::new();
    let mut i = 0;
    while i < 8 {
        let s = generate_password(len, pa);
        new_passwords.push(s);
        i += 1;
    }
    new_passwords
}

#[derive(Debug, Clone, Copy)]
struct PwgenParams {
    include_lowercase: bool,
    include_uppercase: bool,
    include_numbers: bool,
    include_symbols: bool,
}

fn generate_password(len: i32, pa: PwgenParams) -> String {
    const LOWERCASE_CS: &str = "abcdefghijklmnopqrstuvwxyz";
    const UPPERCASE_CS: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
    const NUMBERS_CS: &str = "01234567890123456789";
    const SYMBOLS_CS: &str = "!@#$%^&*_+-=|;:,?";
    //const SYMBOLS_CS: &str = "!@#$%^&*()_+-=[]{}|;:,.<>?";
    //
    let mut charset = "".to_string();
    if pa.include_lowercase {
        charset += LOWERCASE_CS;
        charset += LOWERCASE_CS;
    }
    if pa.include_uppercase {
        charset += UPPERCASE_CS;
    }
    if pa.include_numbers {
        charset += NUMBERS_CS;
    }
    if pa.include_symbols {
        charset += SYMBOLS_CS;
    }
    if charset.is_empty() {
        // Default to lowercase if no character type is selected
        charset += LOWERCASE_CS;
    }
    let charset: Vec<u8> = charset.into_bytes();
    let mut password: Vec<u8> = Vec::new();
    let mut i = 0;
    while i < len {
        let idx = fastrand::u32(0..(charset.len() as u32)) as usize;
        password.push(charset[idx]);
        i += 1;
    }
    String::from_utf8_lossy(&password).to_string()
}
