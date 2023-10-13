// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use database::{database_insert, find_by_model, fuzzy_by_model, start_db};
use tauri::{self, api::dialog::blocking, Manager};
// use tauri_plugin_log::LogTarget;
mod database;
mod menus;
mod printing;

fn main() {
    std::env::set_var("CG_PDF_VERBOSE", "true");
    let ctx = tauri::generate_context!();

    let menu = menus::bar::generate_menu_bar(&ctx.package_info().name);

    let build = tauri::Builder::default()
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
        .plugin(tauri_plugin_printing_ext::init())
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
            database::open_database_folder,
            menus::events::save,
            printing::commands::print_4x20_labels,
        ]);

    build
        .run(ctx)
        .expect("error while running tauri application");
}
