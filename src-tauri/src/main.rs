// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod api;
mod models;

// Import the new, streamlined command functions
use crate::api::backend::{sync_all_creator_data, verify_creator_account};
use crate::api::fansly::get_fansly_profile;
use crate::models::AppState;

use reqwest;
use tauri::{
    menu::{Menu, MenuItem},
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
    Emitter, Manager, WindowEvent,
};
use tauri_plugin_updater::UpdaterExt;
use tokio::sync::Mutex;

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_updater::Builder::new().build())
        // Manage the AppState with the new API URL
        .manage(Mutex::new(AppState {
            client: reqwest::Client::new(),
            // --- UPDATED: Point to your new local API service ---
            api_base_url: "https://api.notifansly.xyz".to_string(),
        }))
        .setup(|app| {
            // This entire setup block is preserved as requested.
            #[cfg(desktop)]
            {
                use tauri_plugin_autostart::MacosLauncher;
                use tauri_plugin_autostart::ManagerExt;

                app.handle().plugin(tauri_plugin_autostart::init(
                    MacosLauncher::LaunchAgent,
                    Some(vec!["--minimized"]),
                ))?;
                let autostart_manager = app.autolaunch();
                let _ = autostart_manager.enable();
            }

            let quit = MenuItem::with_id(app, "quit", "Quit", true, None::<&str>)?;
            let show = MenuItem::with_id(app, "show", "Show Window", true, None::<&str>)?;
            let hide = MenuItem::with_id(app, "hide", "Hide Window", true, None::<&str>)?;
            let sync_now = MenuItem::with_id(app, "sync_now", "Sync Now", true, None::<&str>)?;
            let toggle_auto_sync = MenuItem::with_id(
                app,
                "toggle_auto_sync",
                "Toggle Auto Sync",
                true,
                None::<&str>,
            )?;
            let menu = Menu::with_items(app, &[&show, &hide, &sync_now, &toggle_auto_sync, &quit])?;
            let _tray = TrayIconBuilder::with_id("main-tray")
                .tooltip("NotiFansly Sync")
                .icon(app.default_window_icon().unwrap().clone())
                .menu(&menu)
                .show_menu_on_left_click(false)
                .on_menu_event(move |app, event| match event.id.as_ref() {
                    "quit" => app.exit(0),
                    "show" => {
                        if let Some(window) = app.get_webview_window("main") {
                            let _ = window.show();
                            let _ = window.set_focus();
                        }
                    }
                    "hide" => {
                        if let Some(window) = app.get_webview_window("main") {
                            let _ = window.hide();
                        }
                    }
                    "sync_now" => {
                        if let Some(window) = app.get_webview_window("main") {
                            let _ = window.emit("tray-sync-now", ());
                        }
                    }
                    "toggle_auto_sync" => {
                        if let Some(window) = app.get_webview_window("main") {
                            let _ = window.emit("tray-toggle-auto-sync", ());
                        }
                    }
                    _ => {}
                })
                .on_tray_icon_event(|tray, event| {
                    if let TrayIconEvent::Click {
                        button: MouseButton::Left,
                        button_state: MouseButtonState::Up,
                        ..
                    } = event
                    {
                        let app = tray.app_handle();
                        if let Some(webview_window) = app.get_webview_window("main") {
                            if webview_window.is_visible().unwrap_or(false) {
                                let _ = webview_window.hide();
                            } else {
                                let _ = webview_window.show();
                                let _ = webview_window.set_focus();
                            }
                        }
                    }
                })
                .build(app)?;

            let handle = app.handle().clone();
            tauri::async_runtime::spawn(async move {
                if let Ok(updater) = handle.updater() {
                    match updater.check().await {
                        Ok(Some(update)) => {
                            if let Err(e) = update.download_and_install(|_, _| {}, || {}).await {
                                eprintln!("Failed to install update: {}", e);
                            } else {
                                handle.restart();
                            }
                        }
                        Err(e) => eprintln!("Failed to check for updates: {}", e),
                        _ => {}
                    }
                }
            });
            Ok(())
        })
        .on_window_event(|window, event| {
            if let WindowEvent::CloseRequested { api, .. } = event {
                window.hide().unwrap();
                api.prevent_close();
            }
        })
        // --- UPDATED: Register the new, streamlined handlers ---
        .invoke_handler(tauri::generate_handler![
            get_fansly_profile,
            verify_creator_account,
            sync_all_creator_data
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
