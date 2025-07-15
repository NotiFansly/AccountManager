// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod api;
mod models;

use crate::api::dashboard::{create_account, sync_all_data, sync_data, sync_data_enhanced};
use crate::api::fansly::{
    fetch_fansly_data, fetch_followers_and_subscribers, fetch_subscription_tiers,
};
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
        .manage(Mutex::new(AppState {
            client: reqwest::Client::new(),
            api_base_url: "https://api.notifansly.xyz".to_string(),
        }))
        .setup(|app| {
            // Initialize autostart plugin in setup
            #[cfg(desktop)]
            {
                use tauri_plugin_autostart::MacosLauncher;
                use tauri_plugin_autostart::ManagerExt;

                app.handle().plugin(tauri_plugin_autostart::init(
                    MacosLauncher::LaunchAgent,
                    Some(vec!["--minimized"]), // You can add startup flags here
                ))?;

                // Optionally enable autostart by default
                let autostart_manager = app.autolaunch();
                let _ = autostart_manager.enable();
            }

            // Create menu items with correct syntax
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

            // Create system tray
            let _tray = TrayIconBuilder::with_id("main-tray")
                .tooltip("Fansly Sync App")
                .icon(app.default_window_icon().unwrap().clone())
                .menu(&menu)
                .show_menu_on_left_click(false)
                .on_menu_event(move |app, event| match event.id.as_ref() {
                    "quit" => {
                        app.exit(0);
                    }
                    "show" => {
                        let windows = app.webview_windows();
                        if let Some(window) = windows.values().next() {
                            let _ = window.show();
                            let _ = window.set_focus();
                        }
                    }
                    "hide" => {
                        let windows = app.webview_windows();
                        if let Some(window) = windows.values().next() {
                            let _ = window.hide();
                        }
                    }
                    "sync_now" => {
                        // Trigger sync via event
                        let windows = app.webview_windows();
                        if let Some(window) = windows.values().next() {
                            let _ = window.emit("tray-sync-now", ());
                        }
                    }
                    "toggle_auto_sync" => {
                        // Toggle auto sync via event
                        let windows = app.webview_windows();
                        if let Some(window) = windows.values().next() {
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
                            // Toggle window visibility on left click
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

            // This code runs in the background when the app starts.
            let handle = app.handle().clone();
            tauri::async_runtime::spawn(async move {
                if let Ok(updater) = handle.updater() {
                    match updater.check().await {
                        Ok(Some(update)) => {
                            println!(
                                "Update found: {} (current version: {})",
                                update.version, update.current_version
                            );
                            if let Err(e) = update.download_and_install(|_, _| {}, || {}).await {
                                eprintln!("Failed to install update: {}", e);
                            } else {
                                println!("Update installed, restarting application...");
                                handle.restart();
                            }
                        }
                        Ok(None) => {
                            println!("App is up to date.");
                        }
                        Err(e) => {
                            eprintln!("Failed to check for updates: {}", e);
                        }
                    }
                } else {
                    eprintln!("Failed to get updater instance.");
                }
            });

            Ok(())
        })
        .on_window_event(|window, event| {
            match event {
                WindowEvent::CloseRequested { api, .. } => {
                    // Hide the window instead of closing it
                    window.hide().unwrap();
                    api.prevent_close();
                }
                _ => {}
            }
        })
        .invoke_handler(tauri::generate_handler![
            create_account,
            fetch_fansly_data,
            sync_data,
            sync_data_enhanced,
            sync_all_data,
            fetch_followers_and_subscribers,
            fetch_subscription_tiers
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
