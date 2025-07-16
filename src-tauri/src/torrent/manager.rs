// Torrent manager using rqbit session
use librqbit::{
    AddTorrent, AddTorrentOptions, AddTorrentResponse, ManagedTorrent, ManagedTorrentState, Session,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use chrono::Utc;
use hex;


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

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
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
pub struct PeerInfo {
    pub id: String,
    pub address: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OverallStats {
    pub active_torrents: usize,
    pub total_download_speed: u64,
    pub total_upload_speed: u64,
    pub total_torrents: usize,
}

struct ActiveTorrent {
    info: TorrentInfo,
    handle: Arc<ManagedTorrent>,
    is_paused: bool,
}

impl std::fmt::Debug for ActiveTorrent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ActiveTorrent")
            .field("info", &self.info)
            .field("is_paused", &self.is_paused)
            .finish()
    }
}

pub struct TorrentManager {
    session: Arc<Session>,
    torrents: Arc<Mutex<HashMap<String, ActiveTorrent>>>,
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
        let add_torrent = AddTorrent::Url(magnet_uri.into());

        let options = AddTorrentOptions {
            overwrite: true,
            ..Default::default()
        };

        let response = self
            .session
            .add_torrent(add_torrent, Some(options))
            .await
            .map_err(|e| e.to_string())?;

        let (_id, handle) = match response {
            AddTorrentResponse::Added(id, handle) => (id, handle),
            AddTorrentResponse::AlreadyManaged(id, handle) => (id, handle),
            _ => return Err("Failed to add torrent: list only response".into()),
        };
        let info_hash_bytes = handle.info_hash().0;
        let torrent_id = hex::encode(info_hash_bytes);
        let info_hash = torrent_id.clone();

        let torrent_info = TorrentInfo {
            id: torrent_id.clone(),
            name: handle.name().unwrap_or_else(|| "Loading metadata...".to_string()),
            info_hash,
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

        let managed_torrent = ActiveTorrent {
            info: torrent_info,
            handle,
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
        let torrent_to_remove = self.torrents.lock().unwrap().remove(torrent_id);

        if let Some(managed_torrent) = torrent_to_remove {
            self.session
                .delete(managed_torrent.handle.info_hash().into(), false)
                .await
                .ok();
            drop(managed_torrent);
            Ok(())
        } else {
            Err(format!("Torrent with ID {} not found", torrent_id))
        }
    }

    pub async fn pause_torrent(&self, torrent_id: &str) -> Result<(), String> {
        let handle = {
            let torrents = self.torrents.lock().unwrap();
            torrents.get(torrent_id).map(|t| t.handle.clone())
        };

        if let Some(handle) = handle {
            self.session
                .pause(&handle)
                .await
                .map_err(|e| e.to_string())?;
            let mut torrents = self.torrents.lock().unwrap();
            if let Some(managed_torrent) = torrents.get_mut(torrent_id) {
                managed_torrent.is_paused = true;
                managed_torrent.info.status = TorrentStatus::Paused;
            }
            Ok(())
        } else {
            Err(format!("Torrent with ID {} not found", torrent_id))
        }
    }

    pub async fn resume_torrent(&self, torrent_id: &str) -> Result<(), String> {
        let handle = {
            let torrents = self.torrents.lock().unwrap();
            torrents.get(torrent_id).map(|t| t.handle.clone())
        };

        if let Some(handle) = handle {
            self.session
                .unpause(&handle)
                .await
                .map_err(|e| e.to_string())?;
            let mut torrents = self.torrents.lock().unwrap();
            if let Some(managed_torrent) = torrents.get_mut(torrent_id) {
                managed_torrent.is_paused = false;
            }
            // The status will be updated in the next tick
            Ok(())
        } else {
            Err(format!("Torrent with ID {} not found", torrent_id))
        }
    }

    pub async fn update_torrent_stats(
        &self,
    ) -> (Vec<TorrentInfo>, Vec<TorrentInfo>, Vec<String>) {
        let mut updated_torrents = Vec::new();
        let mut completed_torrents = Vec::new();
        let mut errored_torrents = Vec::new();

        {
            let mut torrents = self.torrents.lock().unwrap();

            for (_torrent_id, managed_torrent) in torrents.iter_mut() {
                if managed_torrent.is_paused {
                    if managed_torrent.info.status != TorrentStatus::Paused {
                        managed_torrent.info.status = TorrentStatus::Paused;
                        updated_torrents.push(managed_torrent.info.clone());
                    }
                    continue;
                }

                let stats = managed_torrent.handle.stats();
                let mut updated_info = managed_torrent.info.clone();

                if let Some(name) = managed_torrent.handle.name() {
                    if updated_info.name != name {
                        updated_info.name = name;
                    }
                }

                if stats.total_bytes > 0 {
                    updated_info.size = stats.total_bytes;
                }
                updated_info.downloaded = stats.progress_bytes;
                updated_info.uploaded = stats.uploaded_bytes;
                if stats.total_bytes > 0 {
                    updated_info.progress =
                        (stats.progress_bytes as f64 / stats.total_bytes as f64) * 100.0;
                }

                if let Some(live) = &stats.live {
                    // Convert mbps (megabits per second) to bytes per second.
                    updated_info.download_speed = (live.download_speed.mbps * 125_000.0) as u64;
                    updated_info.upload_speed = (live.upload_speed.mbps * 125_000.0) as u64;

                    if updated_info.download_speed > 0 && updated_info.progress < 100.0 {
                        let remaining_bytes =
                            updated_info.size.saturating_sub(updated_info.downloaded);
                        updated_info.eta = Some(remaining_bytes / updated_info.download_speed);
                    } else {
                        updated_info.eta = None;
                    }
                    // TODO: Figure out how to get peer/seeder counts from librqbit.
                    // The docs are not clear, and LiveStats doesn't seem to contain this info directly.
                    // updated_info.peers = live.snapshot.peers.all as u32;
                    // updated_info.seeders = live.snapshot.peers.complete as u32;
                } else {
                    updated_info.download_speed = 0;
                    updated_info.upload_speed = 0;
                    updated_info.eta = None;
                    updated_info.peers = 0;
                    updated_info.seeders = 0;
                }

                let old_status = updated_info.status.clone();
                
                managed_torrent.handle.with_state(|state| {
                    updated_info.status = match state {
                        ManagedTorrentState::Initializing(_) => TorrentStatus::Queued,
                        ManagedTorrentState::Live(_) => {
                            if stats.finished {
                                TorrentStatus::Seeding
                            } else if updated_info.progress > 0.0 {
                                TorrentStatus::Downloading
                            } else {
                                TorrentStatus::MetadataDownload
                            }
                        }
                        ManagedTorrentState::Paused(_) => TorrentStatus::Paused,
                        ManagedTorrentState::Error(_) => TorrentStatus::Error,
                        _ => updated_info.status,
                    };
                });

                if updated_info.status == TorrentStatus::Error {
                    errored_torrents.push(updated_info.id.clone());
                }

                if (stats.finished || updated_info.progress >= 100.0)
                    && updated_info.completed_at.is_none()
                {
                    updated_info.completed_at = Some(Utc::now().to_rfc3339());
                    if updated_info.status != TorrentStatus::Completed {
                        updated_info.status = TorrentStatus::Completed;
                        completed_torrents.push(updated_info.clone());
                    }
                }

                if old_status != updated_info.status
                    || matches!(updated_info.status, TorrentStatus::Downloading | TorrentStatus::Seeding)
                {
                    updated_torrents.push(updated_info.clone());
                }

                managed_torrent.info = updated_info;
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
        let torrents = self.torrents.lock().unwrap();
        let (total_download_speed, total_upload_speed) = torrents.values()
            .filter(|mt| !mt.is_paused)
            .fold((0, 0), |(ds, us), mt| (ds + mt.info.download_speed, us + mt.info.upload_speed));

        OverallStats {
            active_torrents: self.get_active_torrents_count(),
            total_download_speed,
            total_upload_speed,
            total_torrents: torrents.len(),
        }
    }
} 