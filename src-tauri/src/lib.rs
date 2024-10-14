use tauri::{
    menu::{Menu, MenuItem},
    tray::TrayIconBuilder,
    Manager,
};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_nspanel::init())
        .setup(|app| {
            let sub = tauri::WebviewWindowBuilder::new(
                app,
                "sub",
                tauri::WebviewUrl::App("index2.html".into()),
            )
            .build()
            .unwrap();
            sub.hide().unwrap();

            #[cfg(desktop)]
            {
                use tauri_plugin_global_shortcut::{
                    Code, GlobalShortcutExt, Modifiers, Shortcut, ShortcutState,
                };

                let shortcut_key =
                    Shortcut::new(Some(Modifiers::SHIFT | Modifiers::CONTROL), Code::KeyO);
                app.handle().plugin(
                    tauri_plugin_global_shortcut::Builder::new()
                        .with_handler(move |_app, shortcut, event| {
                            println!("{:?}", shortcut);
                            if shortcut == &shortcut_key {
                                match event.state() {
                                    ShortcutState::Pressed => {
                                        if sub.is_visible().unwrap() {
                                            sub.hide().unwrap();
                                        } else {
                                            sub.show().unwrap();
                                            sub.set_always_on_top(true).unwrap();
                                        }
                                    }
                                    ShortcutState::Released => {
                                        println!("Released!");
                                    }
                                }
                            }
                        })
                        .build(),
                )?;

                app.global_shortcut().register(shortcut_key)?;
            }

            let quit_i = MenuItem::with_id(app, "quit", "終了する", true, None::<&str>)?;
            let menu = Menu::with_items(app, &[&quit_i])?;

            let _ = TrayIconBuilder::new()
                .icon(app.default_window_icon().unwrap().clone())
                .menu(&menu)
                .on_menu_event(|app, event| match event.id.as_ref() {
                    "quit" => {
                        app.exit(0);
                    }
                    _ => {}
                })
                .build(app)?;

            Ok(())
        })
        .on_window_event(|window, event| {
            if let tauri::WindowEvent::CloseRequested { api, .. } = event {
                api.prevent_close();
                tauri::AppHandle::hide(window.app_handle()).unwrap();
            }
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
