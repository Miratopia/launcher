fn main() {
    tauri_build::build();

    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR");
    let config_path = std::path::Path::new(&manifest_dir).join("tauri.conf.json");
    println!("cargo:rerun-if-changed={}", config_path.display());

    let content = std::fs::read_to_string(&config_path).expect("failed to read tauri.conf.json");
    let v: serde_json::Value = serde_json::from_str(&content).expect("invalid tauri.conf.json");
    let product_name = v
        .get("productName")
        .and_then(|x| x.as_str())
        .expect("tauri.conf.json: missing productName");
    println!("cargo:rustc-env=TAURI_PRODUCT_NAME={product_name}");
}
