// use base64::{engine::general_purpose, Engine as _};
//
// #[cfg(target_os = "macos")]
// use cocoa::base::{id, BOOL, YES};
// use objc::{
//     class,
//     declare::ClassDecl,
//     msg_send,
//     runtime::{Class, Sel},
//     sel, sel_impl,
// };
//
// use std::ptr::null;
//
// #[tauri::command]
// pub fn print_dialog(window: tauri::Window) {
//     let pdf_data = std::fs::read("example.txt").unwrap();
//
//     let _ = window.with_webview(move |view| {
//         let bytes: &[u8] = &general_purpose::STANDARD.decode(&pdf_data).unwrap();
//
//         let can_print: BOOL = unsafe {
//             msg_send![
//               view.inner(),
//               respondsToSelector: sel!(printOperationWithPrintInfo:)
//             ]
//         };
//         if can_print != YES {
//             return ();
//         }
//
//          ffi::run_print_dialog(null(), null(), pdf_data);
//     });
// }
//
// use std::ffi::c_void;
