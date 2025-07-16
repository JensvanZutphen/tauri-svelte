// Chat-related TypeScript interfaces for TorChat

export interface ChatMessage {
  id: string;
  torrentId: string;
  peerId: string;
  peerName?: string;
  content: string;
  timestamp: Date;
  type: MessageType;
  encrypted?: boolean;
  signature?: string;
}

export enum MessageType {
  TEXT = 'text',
  JOIN = 'join',
  LEAVE = 'leave',
  TYPING = 'typing',
  SYSTEM = 'system'
}

export interface ChatPeer {
  id: string;
  name?: string;
  avatar?: string;
  isOnline: boolean;
  lastSeen: Date;
  publicKey?: string;
  webrtcSupported: boolean;
  connectionQuality: ConnectionQuality;
}

export enum ConnectionQuality {
  EXCELLENT = 'excellent',
  GOOD = 'good', 
  FAIR = 'fair',
  POOR = 'poor',
  DISCONNECTED = 'disconnected'
}

export interface ChatRoom {
  torrentId: string;
  torrentName: string;
  peers: ChatPeer[];
  messages: ChatMessage[];
  lastActivity: Date;
  unreadCount: number;
  isActive: boolean;
}

export interface WebRTCConnection {
  peerId: string;
  dataChannel?: RTCDataChannel;
  peerConnection?: RTCPeerConnection;
  connectionState: RTCPeerConnectionState;
  lastActivity: Date;
}

export interface ChatProtocolMessage {
  type: 'join' | 'leave' | 'message' | 'typing' | 'ping' | 'pong';
  peerId: string;
  data?: any;
  timestamp: number;
  signature?: string;
} 