#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod commands;

// Импортируем команды, чтобы они были в корне
use commands::{
    add_device, create_room, delete_device, delete_room, get_report, list_devices, list_rooms,
};

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            list_rooms, // было commands::list_rooms
            create_room,
            delete_room,
            delete_device,
            list_devices,
            add_device,
            get_report,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
