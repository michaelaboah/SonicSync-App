use base64::{
    engine::{self, general_purpose},
    Engine as _,
};
use tauri::command;

#[command]
pub async fn custom_print(window: tauri::Window) {
    println!("custome");
    window
        .eval("console.log(document.getElementById('printTarget'))")
        .unwrap();
}

#[command]
pub fn write_to_pdf(path: &str, base64: &str) {
    let bytes = general_purpose::STANDARD.decode(base64).unwrap();
    std::fs::write(path, &bytes).unwrap();
}
