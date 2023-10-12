use super::layouts::{AveryLabelPage, CableLabel};
use base64::{engine::general_purpose, Engine as _};
use std::{
    fs,
    io::{BufWriter, Cursor},
    path::{Path, PathBuf},
};
use tauri::{self, api::dialog::blocking, Window};

#[tauri::command]
pub fn print_4x20_labels(
    handle: tauri::AppHandle,
    window: tauri::Window,
    labels: Vec<serde_json::Value>,
) -> String {
    let mut fonts_path = handle.path_resolver().resource_dir().unwrap();
    let tmp_dir = handle.path_resolver().app_local_data_dir().unwrap();

    fonts_path.push("resources/fonts/Roboto");

    let mut formatted = Vec::with_capacity(labels.len());
    let labels_len = labels.len();
    for value in labels {
        let d = serde_json::from_value::<CableLabel>(value).unwrap();
        formatted.push(d);
    }

    assert_eq!(labels_len, formatted.len());

    let page = AveryLabelPage::new(formatted, fonts_path, tmp_dir).unwrap();

    let mut buffer = Cursor::new(vec![]);
    page.write_to_bytes(&mut buffer).unwrap();

    general_purpose::STANDARD.encode(buffer.into_inner())
}

#[test]
fn cable_labels() {
    let labels = vec![CableLabel::default()];

    let page = AveryLabelPage::new(labels, "resources/fonts/Roboto", PathBuf::new("/tmp"));
    page.write_to_file("test_pdf.pdf");
}
