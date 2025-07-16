// TorChat Tauri Application
mod torrent;

use std::path::PathBuf;
use std::sync::Arc;
use tauri::Manager;
use torrent::{commands::*, events::EventManager, TorrentManager};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            // Get app data directory for downloads
            let app_data_dir = app
                .path()
                .app_data_dir()
                .unwrap_or_else(|_| PathBuf::from("./downloads"));

            // Create downloads directory
            let downloads_dir = app_data_dir.join("downloads");
            if let Err(e) = std::fs::create_dir_all(&downloads_dir) {
                eprintln!("Failed to create downloads directory: {}", e);
                return Err(format!("Failed to create downloads directory: {}", e).into());
            }

            // Initialize torrent manager
            let app_handle = app.handle().clone();
            tauri::async_runtime::spawn(async move {
                match TorrentManager::new(downloads_dir).await {
                    Ok(manager) => {
                        let manager = Arc::new(manager);
                        
                        // Store the manager in app state
                        app_handle.manage(manager.clone());

                        // Initialize event manager
                        let event_manager = EventManager::new(manager.clone(), app_handle.clone());
                        event_manager.start_monitoring().await;

                        println!("TorChat initialized successfully");
                    }
                    Err(e) => {
                        eprintln!("Failed to initialize torrent manager: {}", e.to_string());
                    }
                }
            });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            add_torrent,
            get_torrents,
            remove_torrent,
            pause_torrent,
            resume_torrent,
            get_torrent_stats,
            get_overall_stats
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
