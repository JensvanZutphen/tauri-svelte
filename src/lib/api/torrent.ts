// TypeScript API layer for Tauri torrent commands
import { invoke } from '@tauri-apps/api/core';
import type { 
  ApiResponse, 
  AddTorrentResponse,
  TorrentListResponse 
} from '../types/api.js';
import type { TorrentInfo } from '../types/torrent.js';

// Request types for Tauri commands
interface AddTorrentRequest {
  magnet_uri: string;
}

// Torrent API functions
export class TorrentAPI {
  
  /**
   * Add a new torrent by magnet URI
   */
  static async addTorrent(magnetUri: string): Promise<AddTorrentResponse> {
    const request: AddTorrentRequest = { magnet_uri: magnetUri };
    const response: ApiResponse<AddTorrentResponse> = await invoke('add_torrent', { request });
    
    if (!response.success || !response.data) {
      throw new Error(response.error || 'Failed to add torrent');
    }
    
    return response.data;
  }

  /**
   * Get list of all torrents
   */
  static async getTorrents(): Promise<TorrentInfo[]> {
    const response: ApiResponse<TorrentInfo[]> = await invoke('get_torrents');
    
    if (!response.success || !response.data) {
      throw new Error(response.error || 'Failed to get torrents');
    }
    
    return response.data;
  }

  /**
   * Remove a torrent by ID
   */
  static async removeTorrent(torrentId: string): Promise<void> {
    const response: ApiResponse<void> = await invoke('remove_torrent', { torrent_id: torrentId });
    
    if (!response.success) {
      throw new Error(response.error || 'Failed to remove torrent');
    }
  }

  /**
   * Pause a torrent by ID
   */
  static async pauseTorrent(torrentId: string): Promise<void> {
    const response: ApiResponse<void> = await invoke('pause_torrent', { torrent_id: torrentId });
    
    if (!response.success) {
      throw new Error(response.error || 'Failed to pause torrent');
    }
  }

  /**
   * Resume a torrent by ID
   */
  static async resumeTorrent(torrentId: string): Promise<void> {
    const response: ApiResponse<void> = await invoke('resume_torrent', { torrent_id: torrentId });
    
    if (!response.success) {
      throw new Error(response.error || 'Failed to resume torrent');
    }
  }

  /**
   * Get stats for a specific torrent
   */
  static async getTorrentStats(torrentId: string): Promise<TorrentInfo | null> {
    const response: ApiResponse<TorrentInfo | null> = await invoke('get_torrent_stats', { torrent_id: torrentId });
    
    if (!response.success) {
      throw new Error(response.error || 'Failed to get torrent stats');
    }
    
    return response.data || null;
  }
} 