// Tauri commands for torrent operations
use super::manager::{TorrentInfo, TorrentManager, OverallStats};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tauri::State;

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub error: Option<String>,
}

impl<T> ApiResponse<T> {
    pub fn success(data: T) -> Self {
        Self {
            success: true,
            data: Some(data),
            error: None,
        }
    }

    pub fn error(error: String) -> Self {
        Self {
            success: false,
            data: None,
            error: Some(error),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AddTorrentRequest {
    pub magnet_uri: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AddTorrentResponse {
    pub torrent_id: String,
    pub name: String,
    pub info_hash: String,
}

// Tauri command to add a torrent
#[tauri::command]
pub async fn add_torrent(
    request: AddTorrentRequest,
    manager: State<'_, Arc<TorrentManager>>,
) -> Result<ApiResponse<AddTorrentResponse>, String> {
    match manager.add_torrent(&request.magnet_uri).await {
        Ok(torrent_id) => {
            // Get the torrent info to return details
            let torrents = manager.get_torrents();
            if let Some(torrent) = torrents.iter().find(|t| t.id == torrent_id) {
                let response = AddTorrentResponse {
                    torrent_id: torrent.id.clone(),
                    name: torrent.name.clone(),
                    info_hash: torrent.info_hash.clone(),
                };
                Ok(ApiResponse::success(response))
            } else {
                Ok(ApiResponse::error("Torrent added but not found in list".to_string()))
            }
        }
        Err(e) => Ok(ApiResponse::error(format!("Failed to add torrent: {}", e))),
    }
}

// Tauri command to get all torrents
#[tauri::command]
pub async fn get_torrents(
    manager: State<'_, Arc<TorrentManager>>,
) -> Result<ApiResponse<Vec<TorrentInfo>>, String> {
    let torrents = manager.get_torrents();
    Ok(ApiResponse::success(torrents))
}

// Tauri command to remove a torrent
#[tauri::command]
pub async fn remove_torrent(
    torrent_id: String,
    manager: State<'_, Arc<TorrentManager>>,
) -> Result<ApiResponse<()>, String> {
    match manager.remove_torrent(&torrent_id).await {
        Ok(_) => Ok(ApiResponse::success(())),
        Err(e) => Ok(ApiResponse::error(format!("Failed to remove torrent: {}", e))),
    }
}

// Tauri command to pause a torrent
#[tauri::command]
pub async fn pause_torrent(
    torrent_id: String,
    manager: State<'_, Arc<TorrentManager>>,
) -> Result<ApiResponse<()>, String> {
    match manager.pause_torrent(&torrent_id).await {
        Ok(_) => Ok(ApiResponse::success(())),
        Err(e) => Ok(ApiResponse::error(format!("Failed to pause torrent: {}", e))),
    }
}

// Tauri command to resume a torrent
#[tauri::command]
pub async fn resume_torrent(
    torrent_id: String,
    manager: State<'_, Arc<TorrentManager>>,
) -> Result<ApiResponse<()>, String> {
    match manager.resume_torrent(&torrent_id).await {
        Ok(_) => Ok(ApiResponse::success(())),
        Err(e) => Ok(ApiResponse::error(format!("Failed to resume torrent: {}", e))),
    }
}

// Tauri command to get torrent stats
#[tauri::command]
pub async fn get_torrent_stats(
    torrent_id: String,
    manager: State<'_, Arc<TorrentManager>>,
) -> Result<ApiResponse<Option<TorrentInfo>>, String> {
    let torrent = manager.get_torrent_by_id(&torrent_id);
    Ok(ApiResponse::success(torrent))
}

// Tauri command to get overall statistics
#[tauri::command]
pub async fn get_overall_stats(
    manager: State<'_, Arc<TorrentManager>>,
) -> Result<ApiResponse<OverallStats>, String> {
    let stats = manager.get_overall_stats();
    Ok(ApiResponse::success(stats))
} 