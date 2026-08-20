use std::fs;
use std::path::Path;

fn main() {
    println!("cargo:rerun-if-changed=static/css");

    let files = [
        "static/css/base/reset.css",
        "static/css/components/buttons.css",
        "static/css/components/card.css",
        "static/css/components/form.css",
        "static/css/components/modal.css",
        "static/css/components/search.css",
        "static/css/components/toast.css",
        "static/css/components/header_toolbar.css",
        "static/css/components/sidebar.css",
    ];

    let mut merged = String::new();
    for file in files {
        let content = fs::read_to_string(file)
            .unwrap_or_else(|e| panic!("failed to read {file}: {e}"));
        merged.push_str(&format!("/* --- {file} --- */\n"));
        merged.push_str(&content);
        merged.push('\n');
    }

    fs::write("static/css/main.css", merged)
        .expect("failed to write main.css");
}