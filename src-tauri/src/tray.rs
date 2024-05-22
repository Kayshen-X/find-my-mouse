use tauri::{AppHandle, SystemTrayEvent, SystemTray, CustomMenuItem, SystemTrayMenu, SystemTrayMenuItem, SystemTraySubmenu, Manager, GlobalShortcutManager};
use tauri::api::dialog::message;

// 托盘菜单
pub fn menu() -> SystemTray {
    let tray_menu = SystemTrayMenu::new()
        // .add_item(CustomMenuItem::new("settings","Settings"))
        // .add_native_item(SystemTrayMenuItem::Separator) // 分割线
        .add_item(CustomMenuItem::new("quit".to_string(), "Quit")); // 退出

    // 设置在右键单击系统托盘时显示菜单
    SystemTray::new().with_menu(tray_menu)
}

// 菜单事件
pub fn handler(app: &AppHandle, event: SystemTrayEvent) {
    // 获取应用窗口
    let window = app.get_window("main").unwrap();
    let mut shortcut_manager = app.global_shortcut_manager();
    // let parent_window = Some(&window);
    // 匹配点击事件
    match event {
        SystemTrayEvent::MenuItemClick { id, .. } => match id.as_str() {
            // "settings"=>{
            //     window.show().unwrap()
            // }
            "quit" => {
                shortcut_manager.unregister("CmdOrCtrl+Shift+K").expect("unregister hot key error");
                std::process::exit(0);
            }
            _ => {}
        },
        _ => {}
    }
}