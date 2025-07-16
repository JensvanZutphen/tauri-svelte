# TorChat Implementation Plan

## Project Overview
Building a lightweight desktop torrent client with integrated P2P chat using **Hybrid Architecture**:
- **Backend**: Rust BitTorrent engine (rqbit) for native TCP/UDP torrenting via Tauri
- **Frontend**: SvelteKit + WebRTC data channels for P2P chat
- **Result**: Full BitTorrent compatibility + innovative WebRTC chat features

## Phase 1: Foundation & Core Setup ⚡
**Goal**: Establish hybrid architecture with Rust BitTorrent backend + Svelte frontend

### 1.1 Dependencies & Setup
- [x] Research Rust BitTorrent crates (`rqbit`, `torrent-rs`) using Context7 MCP for up-to-date docs
- [x] Add chosen Rust BitTorrent crate to `src-tauri/Cargo.toml`
- [x] Install frontend dependencies: `idb` (IndexedDB for chat history) with TypeScript types
- [x] Install and configure `shadcn-svelte` for UI components (use Context7 MCP for docs)
- [x] Set up proper folder structure with TypeScript configuration
- [x] Configure TypeScript for strict type checking and Svelte 5 compatibility
- [x] Create state management using Svelte 5 runes ($state, $derived, $effect)
  > ✅ **COMPLETED**: Fixed Svelte 5 runes by renaming state files to `.svelte.ts` format

### 1.2 Rust BitTorrent Backend
- [x] Implement Tauri commands for torrent operations (`add_torrent`, `remove_torrent`, etc.)
- [x] Create Rust torrent manager using `rqbit` session
- [x] Set up event emission from Rust to frontend (progress, peers, completion)
- [x] Handle torrent lifecycle management (add, remove, pause, resume)
  > ✅ **COMPLETED**: Rust backend compiles successfully with librqbit 8.1.1
- [ ] Test with sample magnet links via TCP/UDP connections
  > 🔄 **NEXT**: End-to-end testing with real magnet links needed

### 1.3 Frontend Integration
- [x] Create TypeScript API layer to call Tauri commands with strict typing
- [x] Design main application layout with TailwindCSS + shadcn-svelte components
- [x] Create torrent list component using shadcn-svelte data tables with Svelte 5 $state/$derived
- [x] Implement add torrent functionality with TypeScript forms and shadcn-svelte components
- [x] Show real-time progress using $effect runes to listen to Rust backend events
  > ✅ **COMPLETED**: Modern UI with shadcn-svelte components and Svelte 5 state management
  > 🔄 **NEXT**: Test the complete application with `npm run tauri dev`

**Deliverable**: Working native BitTorrent client with full TCP/UDP compatibility

---

## Phase 2: Hybrid P2P Chat Foundation 💬
**Goal**: Bridge Rust BitTorrent peers with WebRTC chat channels

### 2.1 Chat Architecture Design
- [ ] Get peer information from Rust backend (IP addresses, peer IDs)
- [ ] Design WebRTC signaling protocol for chat initialization
- [ ] Create chat message protocols (join, leave, message, typing)
- [ ] Plan peer-to-peer WebRTC connection establishment
- [ ] Design fallback mechanisms for non-WebRTC peers

### 2.2 WebRTC Chat Bridge
- [ ] Implement Tauri commands to get active peers from `rqbit`
- [ ] Create WebRTC data channel establishment with discovered peers  
- [ ] Handle WebRTC signaling via DHT or tracker announcements
- [ ] Implement peer discovery for WebRTC-capable clients
- [ ] Bridge BitTorrent peer events to chat system

### 2.3 Chat Core Features
- [ ] Implement message sending/receiving over WebRTC data channels with TypeScript types
- [ ] Handle peer join/leave events using Svelte 5 $effect runes
- [ ] Create message validation and sanitization with TypeScript interfaces
- [ ] Basic chat history storage (IndexedDB with typed schemas)
- [ ] Per-torrent chat room management using $state and $derived runes

### 2.4 Chat UI Components
- [ ] Design chat sidebar/panel for each torrent using shadcn-svelte sheet/sidebar components
- [ ] Create message components (sent/received styling) with shadcn-svelte cards/avatars
- [ ] Implement peer list with online indicators using shadcn-svelte badge/status components
- [ ] Add typing indicators over WebRTC with shadcn-svelte loading animations
- [ ] Message input with send functionality using shadcn-svelte input/button components

**Deliverable**: Hybrid chat system working with BitTorrent peer discovery + WebRTC messaging

---

## Phase 3: Enhanced Chat Features 🚀
**Goal**: Add advanced chat functionality and user experience improvements

### 3.1 Advanced Chat Features
- [ ] Message encryption/signing for security
- [ ] Chat room persistence (reconnect to existing chats)
- [ ] File sharing within chat (small files via data channels)
- [ ] Emoji support and message formatting
- [ ] Chat notifications (system tray)

### 3.2 Peer Management
- [ ] Peer verification and trust system
- [ ] Block/unblock peers functionality
- [ ] Peer nicknames and avatars
- [ ] Connection quality indicators
- [ ] Automatic peer discovery optimization

### 3.3 Chat UX Polish
- [ ] Message search and filtering
- [ ] Chat themes and customization
- [ ] Sound notifications
- [ ] Unread message counters
- [ ] Chat export functionality

**Deliverable**: Full-featured chat system with security and UX polish

---

## Phase 4: Torrent Management Excellence 📁
**Goal**: Complete torrent client functionality with advanced features

### 4.1 Advanced Torrent Features
- [ ] File selection within multi-file torrents
- [ ] Bandwidth limiting controls per torrent
- [ ] Seeding management and ratio tracking
- [ ] Torrent categories and labels
- [ ] Download scheduling

### 4.2 Performance Optimization
- [ ] Implement torrent connection limits
- [ ] Memory usage optimization
- [ ] Large file list virtualization
- [ ] Background processing optimization
- [ ] Resource cleanup on component unmount

### 4.3 File Management
- [ ] Drag-and-drop torrent adding
- [ ] .torrent file support
- [ ] Download location management
- [ ] File preview capabilities
- [ ] Completed downloads organization

**Deliverable**: Production-ready torrent client with advanced management

---

## Phase 5: UI/UX & Polish ✨
**Goal**: Create beautiful, responsive interface with excellent user experience

### 5.1 Theme & Design System
- [ ] Dark/light theme toggle using shadcn-svelte theme provider
- [ ] Responsive design leveraging shadcn-svelte responsive utilities
- [ ] Consistent design system using shadcn-svelte component library
- [ ] Accessibility improvements (shadcn-svelte components are accessible by default)
- [ ] Custom icons and branding with lucide-react icons (part of shadcn-svelte)

### 5.2 Advanced UI Features
- [ ] Search and filter functionality using shadcn-svelte command/search components
- [ ] Sortable columns and views with shadcn-svelte data table components
- [ ] Context menus and shortcuts using shadcn-svelte dropdown/context menu components
- [ ] Settings panel with preferences using shadcn-svelte form/switch/select components
- [ ] System tray integration (Tauri native functionality)

### 5.3 Error Handling & Feedback
- [ ] Comprehensive error handling with shadcn-svelte alert/error components
- [ ] User-friendly error messages using shadcn-svelte toast/dialog components
- [ ] Loading states and skeleton screens with shadcn-svelte skeleton/spinner components
- [ ] Success/failure notifications using shadcn-svelte toast/alert components
- [ ] Connection status indicators with shadcn-svelte badge/status components

**Deliverable**: Polished desktop application ready for daily use

---

## Phase 6: Testing & Distribution 🧪
**Goal**: Ensure reliability and prepare for distribution

### 6.1 Testing Strategy
- [ ] Unit tests for utility functions
- [ ] Integration tests for WebTorrent functionality
- [ ] E2E tests for critical user flows
- [ ] Multi-instance P2P chat testing
- [ ] Performance testing with multiple torrents

### 6.2 Security & Validation
- [ ] Torrent file validation
- [ ] Magnet link verification
- [ ] XSS prevention in chat messages
- [ ] Peer verification improvements
- [ ] Security audit of data channels

### 6.3 Build & Distribution
- [ ] Optimize production builds
- [ ] Cross-platform testing (Linux, Windows, macOS)
- [ ] Create installation packages
- [ ] Documentation and user guides
- [ ] Release preparation

**Deliverable**: Tested, secure, and distributable desktop application

---

## Technical Architecture

### Hybrid Architecture Overview
**Two-Layer P2P System:**
1. **Rust Backend (Tauri)**: Native BitTorrent with TCP/UDP for maximum compatibility
2. **Frontend (SvelteKit)**: WebRTC data channels for innovative chat features
3. **Bridge**: Rust provides peer discovery, frontend establishes WebRTC chat

### Key Components Structure
```
src-tauri/                  # Rust backend (BitTorrent engine)
├── src/
│   ├── main.rs            # Tauri app entry point
│   ├── torrent/           # BitTorrent functionality
│   │   ├── manager.rs     # rqbit session management
│   │   ├── commands.rs    # Tauri commands (add_torrent, etc.)
│   │   └── events.rs      # Progress/peer events to frontend
│   └── chat/              # Chat support functions
│       ├── peers.rs       # Peer discovery for WebRTC
│       └── signaling.rs   # WebRTC signaling helpers
└── Cargo.toml             # Rust dependencies (rqbit, etc.)

src/                        # SvelteKit frontend
├── lib/
│   ├── components/          # Reusable Svelte components
│   │   ├── ui/             # shadcn-svelte components (auto-generated)
│   │   ├── torrent/        # Torrent UI components (using shadcn-svelte)
│   │   ├── chat/           # Chat UI components (using shadcn-svelte)
│   │   └── layout/         # Layout components (headers, sidebars, etc.)
│   ├── state/              # Svelte 5 runes state management
│   │   ├── torrents.ts     # Torrent state using $state/$derived (from Rust events)
│   │   ├── chat.ts         # Chat state management with runes
│   │   └── settings.ts     # App settings state with runes
│   ├── api/                # Tauri command wrappers (TypeScript)
│   │   ├── torrent.ts      # Frontend -> Rust torrent API
│   │   └── peers.ts        # Frontend -> Rust peer API
│   ├── chat/               # WebRTC P2P chat functionality (TypeScript)
│   │   ├── webrtc.ts       # RTCDataChannel management
│   │   ├── protocol.ts     # Message protocols with types
│   │   ├── signaling.ts    # WebRTC connection establishment
│   │   ├── encryption.ts   # Message security
│   │   └── storage.ts      # Chat persistence (IndexedDB with types)
│   ├── types/              # TypeScript type definitions
│   │   ├── torrent.ts      # Torrent-related interfaces
│   │   ├── chat.ts         # Chat message/peer interfaces
│   │   └── api.ts          # Tauri API response types
│   └── utils/              # Helper functions (TypeScript)
├── routes/                 # SvelteKit routes
│   └── +page.svelte       # Main application page
└── static/                 # Static assets
```

### Hybrid Architecture Benefits
- ✅ **Maximum BitTorrent compatibility** - Connect to ALL BitTorrent clients via TCP/UDP
- ✅ **Innovative chat features** - WebRTC data channels for real-time messaging
- ✅ **Native performance** - Single Rust binary, no external dependencies
- ✅ **Gradual WebRTC adoption** - Chat works as more clients support WebRTC
- ✅ **Best of both worlds** - Traditional torrenting + modern P2P chat

### State Management Pattern (Svelte 5 Runes)
- **Rust backend events** - Torrent progress/peers emitted to frontend via Tauri
- **Svelte 5 runes ($state, $derived, $effect)** - Reactive state management for real-time UI updates
- **TypeScript throughout** - Strict typing for all state, API calls, and components
- **WebRTC state with runes** - Chat connection tracking using $state and $derived
- **Event-driven architecture** - BitTorrent peer discovery triggers WebRTC chat via $effect
- **Local persistence** - Settings in Rust, chat history in IndexedDB with TypeScript types

### Development Guidelines
**TypeScript & Svelte 5 Requirements:**
- 🔥 **TypeScript everywhere** - All frontend code must use TypeScript (.ts files)
- ⚡ **Svelte 5 runes only** - Use $state, $derived, $effect - never use stores
- 🚫 **No JavaScript files** - Strict TypeScript enforcement for type safety
- 📝 **Type definitions** - Define interfaces for all data structures (torrents, peers, messages)

**Context7 MCP Documentation Strategy:**
- 🔍 **Always use Context7 MCP** for up-to-date library documentation before installing/implementing
- 📚 **Research first** - Get latest docs for `rqbit`, `shadcn-svelte`, `idb`, TypeScript configs
- 🔄 **Stay current** - Use Context7 to check for breaking changes and new features
- 📖 **Implementation reference** - Use Context7 docs as the primary source during development

**UI Component Library:**
- 🎨 **shadcn-svelte as primary** - Use for all UI components (forms, tables, dialogs, etc.)
- ⚡ **TailwindCSS for styling** - Leverage shadcn-svelte's TailwindCSS integration
- 🌙 **Built-in theming** - Dark/light modes via shadcn-svelte theme provider
- ♿ **Accessibility first** - shadcn-svelte components include ARIA attributes by default

### Development Priorities
1. **Rust BitTorrent backend first** - Core torrenting with maximum compatibility
2. **TypeScript + Svelte 5 foundation** - Type-safe frontend with modern runes
3. **shadcn-svelte UI foundation** - Beautiful, accessible components from the start
4. **WebRTC chat bridging** - Innovative feature that differentiates TorChat
5. **Real-time updates** - Reactive UI using $effect runes for live activity
6. **Security considerations** - Type-safe validation, peer verification, Rust safety
7. **Performance optimization** - Native Rust performance + efficient WebRTC
8. **Cross-platform compatibility** - Single binary deployment

### Risk Mitigation
- **TypeScript configuration** - Ensure proper SvelteKit + Svelte 5 + TypeScript compatibility
- **Svelte 5 migration** - Use Context7 MCP to get latest runes documentation and best practices
- **WebRTC adoption** - Chat gracefully degrades for non-WebRTC peers (torrenting still works)
- **Rust crate stability** - Evaluate `rqbit` vs alternatives, ensure active maintenance
- **WebRTC signaling** - Design robust peer discovery and connection establishment
- **Resource management** - Rust's memory safety + careful WebRTC connection cleanup
- **Cross-platform compatibility** - Test Rust crate on Linux/Windows/macOS

---

## Success Metrics
- [ ] Successfully download torrents with real-time progress
- [ ] Chat messages delivered reliably between peers
- [ ] Multiple simultaneous torrents with chat rooms
- [ ] Responsive UI with <100ms interaction feedback
- [ ] Stable connections with 10+ peers per torrent
- [ ] Cross-platform builds working correctly

---

## Next Steps Decision Points
1. **Research with Context7** - Use Context7 MCP to get latest docs for `rqbit`, `shadcn-svelte` (recommended first step)
2. **Start with Phase 1** - Build Rust BitTorrent backend + shadcn-svelte UI foundation
3. **Prototype WebRTC bridging** - Validate peer discovery -> WebRTC connection flow
4. **Minimal viable product** - Basic Rust torrenting + WebRTC chat, then iterate

**Recommended Path**: 
1. **Context7 Research** → Get latest docs for TypeScript, Svelte 5 runes, `rqbit`, `shadcn-svelte`
2. **Phase 1 Implementation** → Rust backend + TypeScript frontend with Svelte 5 runes
3. **WebRTC Chat Layer** → Bridge BitTorrent peers with type-safe WebRTC messaging 