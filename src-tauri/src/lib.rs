use tauri::{
    menu::{Menu, MenuItem},
    tray::TrayIconBuilder,
};
use tauri::{AppHandle, Manager};
use tauri_nspanel::{
    cocoa::appkit::{NSMainMenuWindowLevel, NSWindowCollectionBehavior},
    panel_delegate, ManagerExt, WebviewWindowExt,
};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![hide])
        .plugin(tauri_nspanel::init())
        .setup(|app| {
            let panel = app.get_webview_window("sub").unwrap().to_panel().unwrap();

            panel.set_level(NSMainMenuWindowLevel + 1);

            panel.set_collection_behaviour(
                NSWindowCollectionBehavior::NSWindowCollectionBehaviorFullScreenAuxiliary,
            );

            #[allow(non_upper_case_globals)]
            const NSWindowStyleMaskNonActivatingPanel: i32 = 1 << 7;
            panel.set_style_mask(NSWindowStyleMaskNonActivatingPanel);

            let panel_delegate = panel_delegate!(PanelDelegate {
                window_did_resign_key,
            });

            let cloned_panel = panel.clone();
            panel_delegate.set_listener(Box::new(move |delegate_name: String| match delegate_name
                .as_str()
            {
                "window_did_resign_key" => {
                    cloned_panel.order_out(None);
                }
                _ => (),
            }));

            panel.set_delegate(panel_delegate);

            use tauri_plugin_global_shortcut::{
                Code, GlobalShortcutExt, Modifiers, Shortcut, ShortcutState,
            };

            let shortcut_key =
                Shortcut::new(Some(Modifiers::SHIFT | Modifiers::CONTROL), Code::KeyO);
            app.handle().plugin(
                tauri_plugin_global_shortcut::Builder::new()
                    .with_handler(move |_app, shortcut, event| {
                        if shortcut == &shortcut_key {
                            match event.state() {
                                ShortcutState::Pressed => {
                                    let panel = _app.get_webview_panel("sub").unwrap();

                                    if panel.is_visible() {
                                        panel.order_out(None);
                                    } else {
                                        panel.show();
                                    }
                                }
                                ShortcutState::Released => {}
                            }
                        }
                    })
                    .build(),
            )?;
            app.global_shortcut().register(shortcut_key)?;

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
                if window.label() == "main" {
                    api.prevent_close();
                    window.hide().unwrap();
                }
            }
        })
        .build(tauri::generate_context!())
        .expect("error while running tauri application")
        .run(|app, event| match event {
            // Dockをクリックしたときにwindowを表示する
            // Cmd + Tabでアプリを切り替えるときなどには表示されないけど、どうすればいいかわからない・・・
            tauri::RunEvent::Reopen {
                has_visible_windows,
                ..
            } => {
                if !has_visible_windows {
                    app.get_webview_window("main").unwrap().show().unwrap();
                }
            }
            _ => {}
        })
}

#[tauri::command]
fn hide(app_handle: AppHandle) {
    let panel = app_handle.get_webview_panel("sub").unwrap();

    if panel.is_visible() {
        panel.order_out(None);
    }
}
