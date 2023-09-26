// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use database::{database_insert, find_by_model, fuzzy_by_model, start_db};
use tauri::{self, command, Manager};
// use tauri_plugin_log::LogTarget;
mod database;
mod dialogs;
mod menus;
fn main() {
    let ctx = tauri::generate_context!();

    let menu = menus::bar::generate_menu_bar(&ctx.package_info().name);

    tauri::Builder::default()
        .setup(move |app| {
            let mut app_data_dir = app.path_resolver().app_local_data_dir().expect(
                "A 'Local App Data Directory' was not found on your system. Cannot run program",
            );

            println!("\n App data dir path: {:?}", &app_data_dir);

            let db = start_db(&mut app_data_dir);
            app.manage(db);
            Ok(())
        })
        .plugin(tauri_plugin_store::Builder::default().build())
        // .plugin(
        //     tauri_plugin_log::Builder::default()
        //         .targets([LogTarget::LogDir, LogTarget::Stdout, LogTarget::Webview])
        //         .build(),
        // )
        .plugin(tauri_plugin_window_state::Builder::default().build())
        .plugin(tauri_plugin_persisted_scope::init())
        .menu(menu)
        .on_menu_event(menus::events::menu_event_handler)
        .invoke_handler(tauri::generate_handler![
            database_insert,
            fuzzy_by_model,
            find_by_model,
            database::get_all_items,
            database::delete_all,
            database::delete_by_model,
            database::update_by_model,
            database::find_many_by_model,
            menus::events::save,
            menus::commands::custom_print,
            menus::commands::write_to_pdf,
            print_dialog,
        ])
        .run(ctx)
        .expect("error while running tauri application");
}

use base64::{engine::general_purpose, Engine as _};

// #[cfg(target_os = "macos")]
use cocoa::base::{id, BOOL, YES};
use objc::{
    class,
    declare::ClassDecl,
    msg_send,
    runtime::{Class, Sel},
    sel, sel_impl,
};

use std::ptr::null;

#[command]
fn print_dialog(window: tauri::Window) {
    let pdf_data = std::fs::read("example.txt").unwrap();

    let _ = window.with_webview(move |view| {
        let bytes: &[u8] = &general_purpose::STANDARD.decode(&pdf_data).unwrap();

        let can_print: BOOL = unsafe {
            msg_send![
              view.inner(),
              respondsToSelector: sel!(printOperationWithPrintInfo:)
            ]
        };
        if can_print != YES {
            return ();
        }
        println!("Hello!");
        ffi::run_print_dialog(null(), null(), pdf_data);
    });
}

use std::ffi::c_void;

#[swift_bridge::bridge]
pub mod ffi {
    extern "Swift" {
        fn run_print_dialog(view: *const c_void, ns_window: *const c_void, data: Vec<u8>);
    }
}
