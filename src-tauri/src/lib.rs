pub mod models;
pub mod managers;
pub mod utils;
pub mod commands;

use std::sync::{Arc, Mutex};
use models::ControllerState;
use managers::ControllerManager;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let controller_manager = Arc::new(Mutex::new(ControllerManager::new()));

    tauri::Builder::default()
        .manage(ControllerState(controller_manager.clone()))
        .invoke_handler(tauri::generate_handler![
            commands::get_gamepads,
            commands::get_gamepad_state
        ])
        .setup(|_app| Ok(()))
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
