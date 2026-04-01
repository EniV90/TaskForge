use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn rust_generate_button_text(status: String) -> String {
    match status.to_uppercase().as_str() {
        "PENDING" => "edit".to_string(),
        "DONE" => "delete".to_string(),
        _ => "An error occured".to_string(),
    }
}
