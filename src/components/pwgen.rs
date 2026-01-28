use dioxus::prelude::*;

/// the component of `tap tap tap beat`
#[component]
pub fn Pwgen() -> Element {
    rsx! {
        MyStyle {}
        div { id: "root",
            div { id: "pwgen-content", PwgenContent {} }
        }
    }
}

#[cfg(not(feature = "inline_style"))]
#[component]
fn MyStyle() -> Element {
    rsx! {
        document::Stylesheet { href: asset!("/assets/css/pwgen.css") }
    }
}

#[cfg(feature = "inline_style")]
#[component]
fn MyStyle() -> Element {
    const TAPB_CSS: &str = const_css_minify::minify!("../../assets/css/pwgen.css");
    rsx! {
        style { "{TAPB_CSS}" }
    }
}

#[component]
fn PwgenContent() -> Element {
    let mut password_length = use_signal(|| 15);
    let mut include_lowercase = use_signal(|| true);
    let mut include_uppercase = use_signal(|| true);
    let mut include_numbers = use_signal(|| true);
    let mut include_symbols = use_signal(|| false);
    let mut passwords = use_signal(|| Vec::<String>::new());
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
        div { id: "pwgen",
            div { id: "pwgen-header", class: "text-center mb-5",
                h1 { class: "display-4 app-title", "Password Generator" }
                p { class: "lead app-title", "Create secure passwords tailored to your needs." }
            }
            div { class: "card bg-dark border-secondary shadow-lg",
                div { class: "card-body text-light",
                    div { class: "row g-4",
                        div { class: "col-12",
                            label { class: "form-label", r#for: "passwordLength",
                                "Password Length: {password_length}"
                            }
                            input {
                                id: "passwordLength",
                                r#type: "range",
                                class: "form-range",
                                min: "4",
                                max: "32",
                                value: "{password_length}",
                                onchange: move |evt| {
                                    let v = evt.value().parse::<i32>().unwrap();
                                    password_length.set(v);
                                },
                            }
                        }
                        div { class: "col-12",
                            div { class: "row",
                                PwgenContentCB {
                                    id: "includeLowercase",
                                    label: "Lowercase (a-z)",
                                    is_checked: include_lowercase(),
                                    onchange: move |_evt| {
                                        include_lowercase.set(!include_lowercase());
                                    },
                                }
                                PwgenContentCB {
                                    id: "includeUppercase",
                                    label: "Uppercase (A-Z)",
                                    is_checked: include_uppercase(),
                                    onchange: move |_evt| {
                                        include_uppercase.set(!include_uppercase());
                                    },
                                }
                                PwgenContentCB {
                                    id: "includeNumbers",
                                    label: "Numbers (0-9)",
                                    is_checked: include_numbers(),
                                    onchange: move |_evt| {
                                        include_numbers.set(!include_numbers());
                                    },
                                }
                                PwgenContentCB {
                                    id: "includeSymbols",
                                    label: "Symbols (!@#$...)",
                                    is_checked: include_symbols(),
                                    onchange: move |_evt| {
                                        include_symbols.set(!include_symbols());
                                    },
                                }
                            }
                        }
                    }
                    div { class: "d-grid mt-4",
                        button {
                            class: "btn btn-primary btn-lg",
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
            if !passwords.is_empty() {
                PwgenContentGeneratedPasswordsElement { passwords }
            }
        }
    }
}

#[component]
fn PwgenContentCB(
    id: String,
    label: String,
    is_checked: bool,
    onchange: EventHandler<FormEvent>,
) -> Element {
    rsx! {
        div { class: "col-md-6",
            div { class: "form-check form-switch",
                input {
                    id: id.clone(),
                    class: "form-check-input",
                    r#type: "checkbox",
                    r#role: "switch",
                    checked: "{is_checked}",
                    onchange: move |evt| onchange.call(evt),
                }
                label { class: "form-check-label", r#for: id, "{label}" }
            }
        }
    }
}

#[component]
fn PwgenContentGeneratedPasswordsElement(passwords: ReadSignal<Vec<String>>) -> Element {
    let mut copied_index = use_signal(|| Option::<i32>::None);
    rsx! {
        div { class: "mt-5",
            h2 { class: "text-center mb-4 app-title", "Generated Passwords" }
            ul { class: "list-group list-group-flush",
                for (i , password) in passwords().into_iter().enumerate() {
                    {
                        let i = i as i32;
                        let is_current = if let Some(ci) = copied_index() {
                            if ci == i { true } else { false }
                        } else {
                            false
                        };
                        rsx! {
                            PwgenContentGeneratedPassword {
                                is_current,
                                password: password.clone(),
                                i,
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
    }
}

#[component]
fn PwgenContentGeneratedPassword(
    is_current: bool,
    password: String,
    i: i32,
    onclick: EventHandler<MouseEvent>,
) -> Element {
    let btn_class = if is_current {
        "btn-success"
    } else {
        "btn-info"
    };
    rsx! {
        li { class: "list-group-item bg-transparent text-light border-secondary d-flex justify-content-between align-items-center",
            span { class: "font-monospace fs-5 password-text", "{password}" }
            button {
                class: "btn {btn_class} btn-sm",
                onclick: move |evt| onclick.call(evt),
                {
                    let s = if is_current { "Copied!" } else { "Copy" };
                    rsx! { "{s}" }
                }
            }
        }
    }
}

async fn copy_to_clipboard(password: String) -> Result<()> {
    let js = format!("navigator.clipboard.writeText('{}');", password);
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
