// build.rs
use rust_version_info_file::rust_version_info_file;

fn main() {
    let path = {
        let dir = std::env::var("OUT_DIR").unwrap();
        format!("{dir}/rust-version-info.txt")
    };
    rust_version_info_file(path.as_str(), "Cargo.toml");
    //
    android_native_proc();
    web_proc();
}

fn android_native_proc() {
    // android
    // override build.gradle.kts
    let path = format!(
        "target/dx/{}/release/android/app/app/build.gradle.kts",
        env!("CARGO_PKG_NAME")
    );
    if std::fs::exists(&path).unwrap() {
        let vc = std::fs::read_to_string("resources/android/versionCode").unwrap();
        let vc = vc.trim();
        let s = std::fs::read_to_string(&path).unwrap();
        let lines: Vec<&str> = s.split('\n').collect();
        let mut outs: Vec<String> = Vec::new();
        for line in lines {
            if let Some(_idx) = line.find("versionCode") {
                outs.push(format!("        versionCode = {}", vc));
            } else if let Some(_idx) = line.find("versionName") {
                outs.push(format!(
                    "        versionName = \"{}.{}\"",
                    env!("CARGO_PKG_VERSION"),
                    vc
                ));
            } else if let Some(_idx) = line.find("minSdk") {
                outs.push("        minSdk = 30".to_string());
            } else {
                outs.push(line.to_string());
            }
        }
        //
        let _ = std::fs::write(&path, outs.join("\n"));
    }
}

fn web_proc() {
    // web
    // override pre.html
    let path = format!(
        "target/dx/{}/debug/web/public/pre.html",
        env!("CARGO_PKG_NAME")
    );
    if std::fs::exists(&path).unwrap() {
        let s = std::fs::read_to_string(&path).unwrap();
        let lines: Vec<&str> = s.split('\n').collect();
        let mut outs: Vec<String> = Vec::new();
        let ms = "https://aki.omusubi.org/pwgen/home";
        for line in lines {
            if let Some(_idx) = line.find(ms) {
                outs.push(line.replace(ms, "home"));
            } else {
                outs.push(line.to_string());
            }
        }
        //
        let _ = std::fs::write(&path, outs.join("\n"));
    }
}
