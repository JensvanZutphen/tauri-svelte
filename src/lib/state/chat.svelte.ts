// Chat state management using Svelte 5 runes

import type { 
  ChatRoom, 
  ChatMessage, 
  ChatPeer, 
  WebRTCConnection,
  MessageType 
} from '../types/chat.js';

// Global chat state using Svelte 5 runes
export const chatState = $state({
  chatRooms: new Map<string, ChatRoom>(),
  activeChatRoom: null as string | null,
  webrtcConnections: new Map<string, WebRTCConnection>(),
  isTyping: new Map<string, string[]>() // torrentId -> peerIds[]
});

// Derived states
export const currentChatRoom = $derived(
  chatState.activeChatRoom ? chatState.chatRooms.get(chatState.activeChatRoom) : null
);

export const totalUnreadMessages = $derived(() => {
  let total = 0;
  for (const room of chatState.chatRooms.values()) {
    total += room.unreadCount;
  }
  return total;
});

export const connectedPeers = $derived(() => {
  const peers: ChatPeer[] = [];
  for (const room of chatState.chatRooms.values()) {
    peers.push(...room.peers.filter(p => p.isOnline));
  }
  return peers;
});

export const totalOnlinePeers = $derived(connectedPeers.length);

export const chatRoomsList = $derived(Array.from(chatState.chatRooms.values()));

export const activeChatMessages = $derived(
  currentChatRoom?.messages || []
);

export const activeChatPeers = $derived(
  currentChatRoom?.peers || []
);

// Chat room management
export function createChatRoom(torrentId: string, torrentName: string): ChatRoom {
  const room: ChatRoom = {
    torrentId,
    torrentName,
    peers: [],
    messages: [],
    lastActivity: new Date(),
    unreadCount: 0,
    isActive: false
  };
  
  chatState.chatRooms.set(torrentId, room);
  return room;
}

export function getChatRoom(torrentId: string): ChatRoom | undefined {
  return chatState.chatRooms.get(torrentId);
}

export function setActiveChatRoom(torrentId: string | null) {
  // Mark previous room as inactive
  if (chatState.activeChatRoom) {
    const prevRoom = chatState.chatRooms.get(chatState.activeChatRoom);
    if (prevRoom) {
      prevRoom.isActive = false;
      prevRoom.unreadCount = 0; // Clear unread when switching
    }
  }
  
  chatState.activeChatRoom = torrentId;
  
  // Mark new room as active
  if (torrentId) {
    const room = chatState.chatRooms.get(torrentId);
    if (room) {
      room.isActive = true;
      room.unreadCount = 0; // Clear unread when activating
    }
  }
}

export function removeChatRoom(torrentId: string) {
  chatState.chatRooms.delete(torrentId);
  
  if (chatState.activeChatRoom === torrentId) {
    chatState.activeChatRoom = null;
  }
  
  // Clean up WebRTC connections for this room
  const connectionsToRemove: string[] = [];
  for (const [peerId, connection] of chatState.webrtcConnections) {
    if (connection.peerId.startsWith(torrentId)) {
      connectionsToRemove.push(peerId);
    }
  }
  
  connectionsToRemove.forEach(peerId => {
    const connection = chatState.webrtcConnections.get(peerId);
    if (connection?.dataChannel) {
      connection.dataChannel.close();
    }
    if (connection?.peerConnection) {
      connection.peerConnection.close();
    }
    chatState.webrtcConnections.delete(peerId);
  });
}

// Peer management
export function addPeerToRoom(torrentId: string, peer: ChatPeer) {
  const room = chatState.chatRooms.get(torrentId);
  if (room) {
    const existingIndex = room.peers.findIndex(p => p.id === peer.id);
    if (existingIndex !== -1) {
      room.peers[existingIndex] = peer;
    } else {
      room.peers.push(peer);
    }
    room.lastActivity = new Date();
  }
}

export function removePeerFromRoom(torrentId: string, peerId: string) {
  const room = chatState.chatRooms.get(torrentId);
  if (room) {
    const index = room.peers.findIndex(p => p.id === peerId);
    if (index !== -1) {
      room.peers.splice(index, 1);
    }
    room.lastActivity = new Date();
  }
  
  // Close WebRTC connection if exists
  const connection = chatState.webrtcConnections.get(peerId);
  if (connection) {
    if (connection.dataChannel) {
      connection.dataChannel.close();
    }
    if (connection.peerConnection) {
      connection.peerConnection.close();
    }
    chatState.webrtcConnections.delete(peerId);
  }
}

export function updatePeerStatus(torrentId: string, peerId: string, isOnline: boolean) {
  const room = chatState.chatRooms.get(torrentId);
  if (room) {
    const peer = room.peers.find(p => p.id === peerId);
    if (peer) {
      peer.isOnline = isOnline;
      peer.lastSeen = new Date();
    }
  }
}

// Message management
export function addMessage(message: ChatMessage) {
  const room = chatState.chatRooms.get(message.torrentId);
  if (room) {
    room.messages.push(message);
    room.lastActivity = new Date();
    
    // Increment unread count if room is not active
    if (!room.isActive && message.type === 'text') {
      room.unreadCount++;
    }
  }
}

export function addSystemMessage(torrentId: string, content: string) {
  const message: ChatMessage = {
    id: crypto.randomUUID(),
    torrentId,
    peerId: 'system',
    content,
    timestamp: new Date(),
    type: 'system' as MessageType
  };
  
  addMessage(message);
}

// WebRTC connection management
export function addWebRTCConnection(peerId: string, connection: WebRTCConnection) {
  chatState.webrtcConnections.set(peerId, connection);
}

export function getWebRTCConnection(peerId: string): WebRTCConnection | undefined {
  return chatState.webrtcConnections.get(peerId);
}

export function updateConnectionState(peerId: string, state: RTCPeerConnectionState) {
  const connection = chatState.webrtcConnections.get(peerId);
  if (connection) {
    connection.connectionState = state;
    connection.lastActivity = new Date();
  }
}

// Typing indicators
export function setTyping(torrentId: string, peerId: string, typing: boolean) {
  const currentTyping = chatState.isTyping.get(torrentId) || [];
  
  if (typing) {
    if (!currentTyping.includes(peerId)) {
      currentTyping.push(peerId);
      chatState.isTyping.set(torrentId, currentTyping);
    }
  } else {
    const index = currentTyping.indexOf(peerId);
    if (index !== -1) {
      currentTyping.splice(index, 1);
      chatState.isTyping.set(torrentId, currentTyping);
    }
  }
}

export function getTypingPeers(torrentId: string): string[] {
  return chatState.isTyping.get(torrentId) || [];
}

// Cleanup
export function clearAllChats() {
  // Close all WebRTC connections
  for (const connection of chatState.webrtcConnections.values()) {
    if (connection.dataChannel) {
      connection.dataChannel.close();
    }
    if (connection.peerConnection) {
      connection.peerConnection.close();
    }
  }
  
  chatState.chatRooms.clear();
  chatState.webrtcConnections.clear();
  chatState.isTyping.clear();
  chatState.activeChatRoom = null;
} 