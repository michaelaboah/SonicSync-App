use super::layouts::AveryLabelPage;
use base64::{engine::general_purpose, Engine as _};
use std::io::{BufWriter, Cursor};
use tauri;

#[tauri::command]
pub fn print_4x20_labels(labels: Vec<String>) -> String {
    let page = AveryLabelPage::new(labels);

    let mut buffer = Cursor::new(vec![]);
    page.write_to_bytes(&mut buffer).unwrap();

    dbg!(general_purpose::STANDARD.encode(buffer.into_inner()))
}
