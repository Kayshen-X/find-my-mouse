// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod tray;

use rdev::{display_size, EventType, simulate};
use tauri::{GlobalShortcutManager, Manager};


// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}


fn send(event_type: &EventType) {
    match simulate(event_type) {
        Ok(()) => (),
        Err(SimulateError) => {
            println!("We could not send {:?}", event_type);
        }
    }
}

fn main() {
    let mut app = tauri::Builder::default()
        .system_tray(tray::menu())
        .on_system_tray_event(tray::handler)
        .invoke_handler(tauri::generate_handler![greet])
        .build(tauri::generate_context!())
        .expect("error while running tauri application");


    let mut shortcut_manager = app.global_shortcut_manager();
    let shortcut_manager_for_a = shortcut_manager.register("CmdOrCtrl+Shift+K", move || {
        let (w, h) = display_size().unwrap();
        send(&EventType::MouseMove { x: (w / 2) as f64, y: (h / 2) as f64 });
    });

    if let Err(err) = shortcut_manager_for_a {
        eprintln!("Error registering shortcut A: {:?}", err);
    }


    app.get_window("main").unwrap().hide().unwrap();
    #[cfg(target_os = "macos")]
    app.set_activation_policy(tauri::ActivationPolicy::Accessory);
    app.run(|_app_handle, _event| {});
}
