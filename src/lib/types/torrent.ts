// Torrent-related TypeScript interfaces for TorChat

export interface TorrentInfo {
  id: string;
  name: string;
  infoHash: string;
  magnetUri: string;
  size: number;
  progress: number;
  downloadSpeed: number;
  uploadSpeed: number;
  peers: number;
  seeders: number;
  leechers: number;
  status: TorrentStatus;
  addedAt: Date;
  completedAt?: Date;
  downloadPath: string;
  files: TorrentFile[];
}

export interface TorrentFile {
  path: string;
  size: number;
  downloaded: number;
  priority: number;
}

export enum TorrentStatus {
  DOWNLOADING = 'downloading',
  SEEDING = 'seeding', 
  PAUSED = 'paused',
  ERROR = 'error',
  COMPLETED = 'completed',
  QUEUED = 'queued'
}

export interface AddTorrentRequest {
  magnetUri?: string;
  torrentFile?: Uint8Array;
  downloadPath?: string;
  paused?: boolean;
}

export interface TorrentProgress {
  id: string;
  progress: number;
  downloadSpeed: number;
  uploadSpeed: number;
  peers: number;
} 