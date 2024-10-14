use tauri::{
    menu::{MenuBuilder, MenuItem},
    tray::{MouseButton, TrayIconBuilder},
    Emitter,
};
use tauri::{AppHandle, Manager};
use tauri_nspanel::{
    cocoa::appkit::{NSMainMenuWindowLevel, NSWindowCollectionBehavior},
    panel_delegate, ManagerExt, WebviewWindowExt,
};

pub const MAIN_LABEL: &str = "main";
pub const TASK_ACTION_BAR_LABEL: &str = "task_action_bar";

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![hide, show_task])
        .plugin(tauri_nspanel::init())
        .setup(|app| {
            let panel = app
                .get_webview_window(TASK_ACTION_BAR_LABEL)
                .unwrap()
                .to_panel()
                .unwrap();

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
                                    let panel =
                                        _app.get_webview_panel(TASK_ACTION_BAR_LABEL).unwrap();

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

            let open_main_i = MenuItem::with_id(
                app,
                "open_main",
                "メインウィンドウを開く",
                true,
                None::<&str>,
            )?;
            let open_task_action_bar_i = MenuItem::with_id(
                app,
                "open_task_action_bar_i",
                "タスクアクションバーを開く",
                true,
                Some("Shift+Control+O"),
            )?;
            let quit_i = MenuItem::with_id(app, "quit", "終了する", true, None::<&str>)?;
            let menu = MenuBuilder::new(app)
                .items(&[&open_main_i, &open_task_action_bar_i])
                .separator()
                .item(&quit_i)
                .build()
                .unwrap();

            let _ = TrayIconBuilder::new()
                .menu_on_left_click(false)
                .icon(app.default_window_icon().unwrap().clone())
                .menu(&menu)
                .on_tray_icon_event(|icon, event| match event {
                    tauri::tray::TrayIconEvent::Click { button, .. }
                        if button == MouseButton::Left =>
                    {
                        let window = icon.app_handle().get_webview_window(MAIN_LABEL).unwrap();
                        window.show().unwrap();
                        window.set_focus().unwrap();
                    }
                    _ => {}
                })
                .on_menu_event(move |app, event| match event.id.as_ref() {
                    id if id == quit_i.id().as_ref() => {
                        app.exit(0);
                    }
                    id if id == open_main_i.id().as_ref() => {
                        let window = app.get_webview_window(MAIN_LABEL).unwrap();
                        window.show().unwrap();
                        window.set_focus().unwrap();
                    }
                    id if id == open_task_action_bar_i.id().as_ref() => {
                        let panel = app.get_webview_panel(TASK_ACTION_BAR_LABEL).unwrap();
                        panel.show();
                    }
                    _ => {}
                })
                .build(app)?;

            Ok(())
        })
        .on_window_event(|window, event| {
            if let tauri::WindowEvent::CloseRequested { api, .. } = event {
                if window.label() == MAIN_LABEL {
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
                    let window = app.get_webview_window(MAIN_LABEL).unwrap();
                    window.show().unwrap();
                    window.set_focus().unwrap();
                }
            }
            _ => {}
        })
}

#[tauri::command]
fn hide(app_handle: AppHandle) {
    let panel = app_handle.get_webview_panel(TASK_ACTION_BAR_LABEL).unwrap();

    if panel.is_visible() {
        panel.order_out(None);
    }
}

#[tauri::command]
fn show_task(app_handle: AppHandle, task: String) {
    let main_window = app_handle.get_webview_window(MAIN_LABEL).unwrap();
    let task_action_bar = app_handle.get_webview_panel(TASK_ACTION_BAR_LABEL).unwrap();

    task_action_bar.order_out(None);
    main_window.show().unwrap();
    main_window.set_focus().unwrap();

    main_window.emit("task", task).unwrap();
}
