// Torrent events for frontend communication
use super::manager::{TorrentInfo, TorrentManager};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use std::time::Duration;
use tauri::{AppHandle, Emitter};
use tokio::time;
use rand;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TorrentProgressEvent {
    pub torrent_id: String,
    pub progress: f64,
    pub download_speed: u64,
    pub upload_speed: u64,
    pub peers: u32,
    pub eta: Option<u64>, // seconds remaining
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TorrentCompletedEvent {
    pub torrent_id: String,
    pub name: String,
    pub download_path: String,
    pub total_size: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TorrentErrorEvent {
    pub torrent_id: String,
    pub error_message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PeerConnectedEvent {
    pub torrent_id: String,
    pub peer_id: String,
    pub peer_address: String,
    pub webrtc_capable: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PeerDisconnectedEvent {
    pub torrent_id: String,
    pub peer_id: String,
}

pub struct EventManager {
    torrent_manager: Arc<TorrentManager>,
    app_handle: AppHandle,
}

impl EventManager {
    pub fn new(torrent_manager: Arc<TorrentManager>, app_handle: AppHandle) -> Self {
        Self {
            torrent_manager,
            app_handle,
        }
    }

    pub async fn start_monitoring(&self) {
        let torrent_manager = self.torrent_manager.clone();
        let event_manager = EventManager::new(self.torrent_manager.clone(), self.app_handle.clone());

        // Start periodic stats updates
        tokio::spawn(async move {
            let mut interval = time::interval(Duration::from_secs(1));
            
            loop {
                interval.tick().await;
                let (updated_torrents, completed_torrents, errored_torrents) = torrent_manager.update_torrent_stats().await;
                
                // Emit progress events for all active torrents
                for torrent in &updated_torrents {
                    let progress_event = EventManager::torrent_info_to_progress_event(torrent);
                    event_manager.emit_torrent_progress(progress_event);
                    
                    // Simulate peer connections/disconnections for active torrents
                    if torrent.peers > 0 && rand::random::<f32>() < 0.05 { // 5% chance
                        let peer_event = PeerConnectedEvent {
                            torrent_id: torrent.id.clone(),
                            peer_id: format!("peer_{}", rand::random::<u32>()),
                            peer_address: format!("192.168.1.{}", rand::random::<u8>()),
                            webrtc_capable: rand::random::<bool>(),
                        };
                        event_manager.emit_peer_connected(peer_event);
                    }
                    
                    if torrent.peers > 1 && rand::random::<f32>() < 0.03 { // 3% chance
                        let peer_event = PeerDisconnectedEvent {
                            torrent_id: torrent.id.clone(),
                            peer_id: format!("peer_{}", rand::random::<u32>()),
                        };
                        event_manager.emit_peer_disconnected(peer_event);
                    }
                }
                
                // Emit completion events for newly completed torrents
                for torrent in &completed_torrents {
                    let completed_event = TorrentCompletedEvent {
                        torrent_id: torrent.id.clone(),
                        name: torrent.name.clone(),
                        download_path: torrent.download_path.clone(),
                        total_size: torrent.size,
                    };
                    event_manager.emit_torrent_completed(completed_event);
                }
                
                // Emit error events for torrents that encountered errors
                for torrent in &errored_torrents {
                    let error_event = TorrentErrorEvent {
                        torrent_id: torrent.id.clone(),
                        error_message: "Failed to download metadata".to_string(),
                    };
                    event_manager.emit_torrent_error(error_event);
                }
            }
        });
    }

    pub fn emit_torrent_progress(&self, event: TorrentProgressEvent) {
        let _ = self.app_handle.emit("torrent-progress", &event);
    }

    pub fn emit_torrent_completed(&self, event: TorrentCompletedEvent) {
        let _ = self.app_handle.emit("torrent-completed", &event);
    }

    pub fn emit_torrent_error(&self, event: TorrentErrorEvent) {
        let _ = self.app_handle.emit("torrent-error", &event);
    }

    pub fn emit_peer_connected(&self, event: PeerConnectedEvent) {
        let _ = self.app_handle.emit("peer-connected", &event);
    }

    pub fn emit_peer_disconnected(&self, event: PeerDisconnectedEvent) {
        let _ = self.app_handle.emit("peer-disconnected", &event);
    }

    // Helper function to convert TorrentInfo to progress event
    pub fn torrent_info_to_progress_event(torrent: &TorrentInfo) -> TorrentProgressEvent {
        TorrentProgressEvent {
            torrent_id: torrent.id.clone(),
            progress: torrent.progress,
            download_speed: torrent.download_speed,
            upload_speed: torrent.upload_speed,
            peers: torrent.peers,
            eta: if torrent.download_speed > 0 && torrent.progress < 100.0 {
                let remaining_bytes = (torrent.size as f64 * (100.0 - torrent.progress) / 100.0) as u64;
                Some(remaining_bytes / torrent.download_speed)
            } else {
                None
            },
        }
    }
} 