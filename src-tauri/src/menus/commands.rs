use tauri::command;

#[command]
pub async fn custom_print(window: tauri::Window) {
    println!("custome");
    window
        .eval("console.log(document.getElementById('printTarget'))")
        .unwrap();
}
