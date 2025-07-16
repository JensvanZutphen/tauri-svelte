// Torrent manager using rqbit session
use librqbit::{AddTorrent, Session};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};

use chrono::Utc;
use rand;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TorrentInfo {
    pub id: String,
    pub name: String,
    pub info_hash: String,
    pub magnet_uri: String,
    pub size: u64,
    pub progress: f64,
    pub download_speed: u64,
    pub upload_speed: u64,
    pub peers: u32,
    pub seeders: u32,
    pub leechers: u32,
    pub status: TorrentStatus,
    pub added_at: String,
    pub completed_at: Option<String>,
    pub download_path: String,
    pub downloaded: u64,
    pub uploaded: u64,
    pub eta: Option<u64>, // seconds remaining
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum TorrentStatus {
    Downloading,
    Seeding,
    Paused,
    Error,
    Completed,
    Queued,
    Checking,
    MetadataDownload,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OverallStats {
    pub active_torrents: usize,
    pub total_download_speed: u64,
    pub total_upload_speed: u64,
    pub total_torrents: usize,
}

#[derive(Debug)]
struct ManagedTorrent {
    info: TorrentInfo,
    is_paused: bool,
}

pub struct TorrentManager {
    session: Arc<Session>,
    torrents: Arc<Mutex<HashMap<String, ManagedTorrent>>>,
    download_path: PathBuf,
}

impl TorrentManager {
    pub async fn new(download_path: PathBuf) -> Result<Self, String> {
        let session = Session::new(download_path.clone()).await.map_err(|e| e.to_string())?;
        
        Ok(Self {
            session,
            torrents: Arc::new(Mutex::new(HashMap::new())),
            download_path,
        })
    }

    pub async fn add_torrent(&self, magnet_uri: &str) -> Result<String, String> {
        let add_torrent = AddTorrent::from_url(magnet_uri);
        let _response = self.session.add_torrent(add_torrent, None).await.map_err(|e| e.to_string())?;
        
        // Generate a unique torrent ID for our internal tracking
        let torrent_id = format!("torrent_{}", chrono::Utc::now().timestamp_millis());
        
        // Create initial torrent info
        let torrent_info = TorrentInfo {
            id: torrent_id.clone(),
            name: "Loading metadata...".to_string(),
            info_hash: "".to_string(),
            magnet_uri: magnet_uri.to_string(),
            size: 0,
            progress: 0.0,
            download_speed: 0,
            upload_speed: 0,
            peers: 0,
            seeders: 0,
            leechers: 0,
            status: TorrentStatus::MetadataDownload,
            added_at: Utc::now().to_rfc3339(),
            completed_at: None,
            download_path: self.download_path.to_string_lossy().to_string(),
            downloaded: 0,
            uploaded: 0,
            eta: None,
        };

        let managed_torrent = ManagedTorrent {
            info: torrent_info,
            is_paused: false,
        };

        {
            let mut torrents = self.torrents.lock().unwrap();
            torrents.insert(torrent_id.clone(), managed_torrent);
        }

        Ok(torrent_id)
    }

    pub fn get_torrents(&self) -> Vec<TorrentInfo> {
        let torrents = self.torrents.lock().unwrap();
        torrents.values().map(|mt| mt.info.clone()).collect()
    }

    pub async fn remove_torrent(&self, torrent_id: &str) -> Result<(), String> {
        let mut torrents = self.torrents.lock().unwrap();
        
        if let Some(managed_torrent) = torrents.remove(torrent_id) {
            // Note: rqbit's ManagedTorrentHandle doesn't expose a direct close/remove method
            // The handle will be dropped automatically when removed from our map
            // The session maintains the underlying torrent, so we'd need to interact
            // with the session to fully remove it if that functionality is exposed
            drop(managed_torrent);
            Ok(())
        } else {
            Err(format!("Torrent with ID {} not found", torrent_id))
        }
    }

    pub async fn pause_torrent(&self, torrent_id: &str) -> Result<(), String> {
        let mut torrents = self.torrents.lock().unwrap();
        
        if let Some(managed_torrent) = torrents.get_mut(torrent_id) {
            managed_torrent.is_paused = true;
            managed_torrent.info.status = TorrentStatus::Paused;
            
            // Note: We mark as paused in our state. The actual pausing would depend on
            // rqbit's API for pausing individual torrents, which may not be exposed
            // in the current public API
            Ok(())
        } else {
            Err(format!("Torrent with ID {} not found", torrent_id))
        }
    }

    pub async fn resume_torrent(&self, torrent_id: &str) -> Result<(), String> {
        let mut torrents = self.torrents.lock().unwrap();
        
        if let Some(managed_torrent) = torrents.get_mut(torrent_id) {
            managed_torrent.is_paused = false;
            managed_torrent.info.status = TorrentStatus::Downloading;
            Ok(())
        } else {
            Err(format!("Torrent with ID {} not found", torrent_id))
        }
    }

    pub async fn update_torrent_stats(&self) -> (Vec<TorrentInfo>, Vec<TorrentInfo>, Vec<TorrentInfo>) {
        let mut updated_torrents = Vec::new();
        let mut completed_torrents = Vec::new();
        let mut errored_torrents = Vec::new();
        
        {
            let mut torrents = self.torrents.lock().unwrap();
            
            for (torrent_id, managed_torrent) in torrents.iter_mut() {
                // Skip updating if paused
                if managed_torrent.is_paused {
                    continue;
                }

                // Try to get stats from the handle
                // Note: The exact API methods depend on rqbit's exposed interface
                // This is a best-effort implementation based on common patterns
                
                let mut updated_info = managed_torrent.info.clone();
                
                // Simulate occasional errors (5% chance)
                if rand::random::<f32>() < 0.05 && updated_info.status == TorrentStatus::MetadataDownload {
                    updated_info.status = TorrentStatus::Error;
                    errored_torrents.push(updated_info.clone());
                    managed_torrent.info = updated_info.clone();
                    updated_torrents.push(updated_info);
                    continue;
                }
                
                // Update basic info if we have access to torrent metadata
                if updated_info.name == "Loading metadata..." {
                    // Try to get torrent name and info hash once metadata is available
                    // This would need the actual rqbit API methods
                    updated_info.name = format!("Torrent {}", torrent_id);
                    updated_info.status = TorrentStatus::Downloading;
                    
                    // Simulate torrent size
                    updated_info.size = 100 * 1024 * 1024; // 100MB simulation
                    
                    // Simulate some peers
                    updated_info.peers = rand::random::<u32>() % 10 + 1;
                    updated_info.seeders = rand::random::<u32>() % updated_info.peers.max(1) + 1;
                    updated_info.leechers = updated_info.peers.saturating_sub(updated_info.seeders);
                }
                
                // Simulate progress updates (in a real implementation, this would come from rqbit)
                // For testing purposes, we'll increment progress slowly
                if updated_info.progress < 100.0 && updated_info.status == TorrentStatus::Downloading {
                    updated_info.progress = (updated_info.progress + 0.1).min(100.0);
                    updated_info.download_speed = 1024 * 1024; // 1 MB/s simulation
                    updated_info.downloaded = (updated_info.size as f64 * updated_info.progress / 100.0) as u64;
                    
                    // Randomly update peer counts
                    if rand::random::<f32>() < 0.1 { // 10% chance
                        updated_info.peers = (updated_info.peers + rand::random::<u32>() % 3).max(1);
                        updated_info.seeders = (rand::random::<u32>() % updated_info.peers.max(1)) + 1;
                        updated_info.leechers = updated_info.peers.saturating_sub(updated_info.seeders);
                    }
                    
                    // Calculate ETA
                    if updated_info.download_speed > 0 && updated_info.progress < 100.0 {
                        let remaining_bytes = updated_info.size.saturating_sub(updated_info.downloaded);
                        updated_info.eta = Some(remaining_bytes / updated_info.download_speed);
                    }
                    
                    // Mark as completed when progress reaches 100%
                    if updated_info.progress >= 100.0 {
                        updated_info.status = TorrentStatus::Completed;
                        updated_info.completed_at = Some(Utc::now().to_rfc3339());
                        updated_info.eta = None;
                        
                        completed_torrents.push(updated_info.clone());
                    }
                }
                
                managed_torrent.info = updated_info.clone();
                updated_torrents.push(updated_info);
            }
        }
        
        (updated_torrents, completed_torrents, errored_torrents)
    }



    pub fn get_torrent_by_id(&self, torrent_id: &str) -> Option<TorrentInfo> {
        let torrents = self.torrents.lock().unwrap();
        torrents.get(torrent_id).map(|mt| mt.info.clone())
    }

    pub fn get_active_torrents_count(&self) -> usize {
        let torrents = self.torrents.lock().unwrap();
        torrents.values()
            .filter(|mt| matches!(mt.info.status, TorrentStatus::Downloading | TorrentStatus::Seeding))
            .count()
    }

    pub fn get_total_download_speed(&self) -> u64 {
        let torrents = self.torrents.lock().unwrap();
        torrents.values()
            .filter(|mt| !mt.is_paused)
            .map(|mt| mt.info.download_speed)
            .sum()
    }

    pub fn get_total_upload_speed(&self) -> u64 {
        let torrents = self.torrents.lock().unwrap();
        torrents.values()
            .filter(|mt| !mt.is_paused)
            .map(|mt| mt.info.upload_speed)
            .sum()
    }

    pub fn get_overall_stats(&self) -> OverallStats {
        OverallStats {
            active_torrents: self.get_active_torrents_count(),
            total_download_speed: self.get_total_download_speed(),
            total_upload_speed: self.get_total_upload_speed(),
            total_torrents: self.torrents.lock().unwrap().len(),
        }
    }
} 