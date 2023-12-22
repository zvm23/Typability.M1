//main.rs

#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod setup;
#[cfg(target_os = "macos")]
mod macos_m1_visual_effect;

fn main() {
    tauri::Builder::default()
        .setup(setup::init)
        .invoke_handler(tauri::generate_handler![
            setup::get_args,
            setup::apply_mac_vibrancy
            ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
