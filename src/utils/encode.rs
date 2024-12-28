use std::path::Path;

use base64::{engine::general_purpose, Engine as _};

pub fn encode_from_path(path: &str) -> String {
    let assets_path = Path::new(env!("CARGO_MANIFEST_DIR")).join("assets");
    let file_path = assets_path.join(path);
    let file = std::fs::read(file_path.clone());

    if let Err(e) = file {
        eprintln!(
            "Error reading file: {}, {:?}",
            file_path.to_str().unwrap_or(path),
            e
        );
        String::new()
    } else {
        general_purpose::STANDARD_NO_PAD.encode(file.unwrap_or_default())
    }
}
