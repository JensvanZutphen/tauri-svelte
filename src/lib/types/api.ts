// Tauri API response TypeScript interfaces for TorChat

export interface ApiResponse<T> {
  success: boolean;
  data?: T;
  error?: string;
}

export interface TorrentListResponse {
  torrents: import('./torrent.js').TorrentInfo[];
}

export interface AddTorrentResponse {
  torrentId: string;
  name: string;
  infoHash: string;
}

export interface PeerInfo {
  id: string;
  address: string;
  port: number;
  clientName?: string;
  downloadSpeed: number;
  uploadSpeed: number;
  progress: number;
  flags: string[];
}

export interface PeerListResponse {
  torrentId: string;
  peers: PeerInfo[];
}

export interface TorrentStatsResponse {
  totalDownloaded: number;
  totalUploaded: number;
  globalDownloadSpeed: number;
  globalUploadSpeed: number;
  activeTorrents: number;
  seedingTorrents: number;
  downloadingTorrents: number;
}

// Events emitted from Rust backend
export interface TorrentProgressEvent {
  torrentId: string;
  progress: number;
  downloadSpeed: number;
  uploadSpeed: number;
  peers: number;
  seeders: number;
  leechers: number;
}

export interface TorrentCompletedEvent {
  torrentId: string;
  name: string;
  downloadPath: string;
}

export interface TorrentErrorEvent {
  torrentId: string;
  error: string;
}

export interface PeerDiscoveredEvent {
  torrentId: string;
  peerId: string;
  address: string;
  port: number;
} 