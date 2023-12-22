// setup.rs

use tauri::{App, Manager};

use crate::macos_m1_visual_effect;

#[cfg(target_os = "macos")]
pub fn init(app: &mut App) -> std::result::Result<(), Box<dyn std::error::Error>> {
let window= app.get_window("typability").unwrap();
    macos_m1_visual_effect::apply_visual_effect(window);

    Ok(())
}

#[tauri::command]
#[cfg(target_os = "macos")]
pub fn apply_mac_vibrancy(window: tauri::Window) {
    crate::macos_m1_visual_effect::apply_visual_effect(window);
}

#[tauri::command]
pub fn get_args() -> Result<Vec<String>, ()> {
  let system_args: Vec<String> = std::env::args().collect();
  Ok(system_args)
}
