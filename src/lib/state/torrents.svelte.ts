// Torrent state management using Svelte 5 runes

import type { TorrentInfo, TorrentProgress, TorrentStatus } from '../types/torrent.js';

// Global torrent state using Svelte 5 runes
export const torrentState = $state({
  torrents: [] as TorrentInfo[],
  selectedTorrentId: null as string | null,
  isLoading: false
});

// Derived state functions (Svelte 5 doesn't allow exporting $derived from modules)
export function getActiveTorrents() {
  return torrentState.torrents.filter(t => 
    t.status === 'downloading' || 
    t.status === 'seeding'
  );
}

export function getDownloadingTorrents() {
  return torrentState.torrents.filter(t => t.status === 'downloading');
}

export function getSeedingTorrents() {
  return torrentState.torrents.filter(t => t.status === 'seeding');
}

export function getSelectedTorrent() {
  return torrentState.selectedTorrentId 
    ? torrentState.torrents.find(t => t.id === torrentState.selectedTorrentId) 
    : null;
}

export function getTotalDownloadSpeed() {
  return torrentState.torrents.reduce((total, torrent) => total + torrent.downloadSpeed, 0);
}

export function getTotalUploadSpeed() {
  return torrentState.torrents.reduce((total, torrent) => total + torrent.uploadSpeed, 0);
}

export function getGlobalProgress() {
  const downloading = getDownloadingTorrents();
  if (downloading.length === 0) return 0;
  
  const totalProgress = downloading.reduce((sum, torrent) => sum + torrent.progress, 0);
  return totalProgress / downloading.length;
}

// State management functions
export function addTorrent(torrent: TorrentInfo) {
  torrentState.torrents.push(torrent);
}

export function removeTorrent(torrentId: string) {
  const index = torrentState.torrents.findIndex(t => t.id === torrentId);
  if (index !== -1) {
    torrentState.torrents.splice(index, 1);
  }
  
  // Clear selection if removed torrent was selected
  if (torrentState.selectedTorrentId === torrentId) {
    torrentState.selectedTorrentId = null;
  }
}

export function updateTorrent(torrentId: string, updates: Partial<TorrentInfo>) {
  const index = torrentState.torrents.findIndex(t => t.id === torrentId);
  if (index !== -1) {
    Object.assign(torrentState.torrents[index], updates);
  }
}

export function updateTorrentProgress(progress: TorrentProgress) {
  const index = torrentState.torrents.findIndex(t => t.id === progress.id);
  if (index !== -1) {
    Object.assign(torrentState.torrents[index], {
      progress: progress.progress,
      downloadSpeed: progress.downloadSpeed,
      uploadSpeed: progress.uploadSpeed,
      peers: progress.peers
    });
  }
}

export function setTorrentStatus(torrentId: string, status: TorrentStatus) {
  updateTorrent(torrentId, { status });
}

export function selectTorrent(torrentId: string | null) {
  torrentState.selectedTorrentId = torrentId;
}

export function setLoading(loading: boolean) {
  torrentState.isLoading = loading;
}

export function clearTorrents() {
  torrentState.torrents.length = 0;
  torrentState.selectedTorrentId = null;
} 