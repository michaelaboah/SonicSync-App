use super::layouts::{AveryLabelPage, CableLabel};
use base64::{engine::general_purpose, Engine as _};
use std::io::{BufWriter, Cursor};
use tauri;

#[tauri::command]
pub fn print_4x20_labels(labels: Vec<serde_json::Value>) -> String {
    let mut formatted = Vec::with_capacity(labels.len());
    let labels_len = labels.len();
    for value in labels {
        let d = serde_json::from_value::<CableLabel>(value).unwrap();
        formatted.push(d);
    }

    assert_eq!(labels_len, formatted.len());

    let page = AveryLabelPage::new(formatted);

    // page.write_to_file("test_pdf.pdf");

    let mut buffer = Cursor::new(vec![]);
    page.write_to_bytes(&mut buffer).unwrap();

    general_purpose::STANDARD.encode(buffer.into_inner())
    // format!("")
}

#[test]
fn cable_labels() {
    let labels = vec![CableLabel::default()];

    let page = AveryLabelPage::new(labels);
    page.write_to_file("test_pdf.pdf");
}
