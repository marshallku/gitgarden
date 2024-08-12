use std::path::Path;

use base64::{engine::general_purpose, Engine as _};

pub fn encode_from_path(path: &str) -> String {
    let assets_path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("src")
        .join("assets");
    let file_path = assets_path.join(path);
    let file = std::fs::read(file_path);

    if let Err(e) = file {
        eprintln!("Error reading file: {:?}", e);
        String::new()
    } else {
        let encoded = general_purpose::STANDARD_NO_PAD.encode(file.unwrap());
        encoded
    }
}
