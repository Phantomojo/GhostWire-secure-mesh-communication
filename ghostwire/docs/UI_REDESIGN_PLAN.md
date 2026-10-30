# GhostWire UI Redesign Plan

## Design Philosophy

Per user requirements:
- **Automatic by default**: Most operations work without manual intervention
- **Not a hassle**: Login friction, manual token entry, daemon restarts should be eliminated
- **Power user differentiation**: Regular users get automatic defaults; power users get manual controls
- **PC-friendly distribution**: QR codes for phones; copy-paste tokens for PC users
- **Live testing visibility**: UI updates with every new wiring or change
- **Progressive disclosure**: Simple UI for regular users, advanced panel for power users
- **Dark theme by default**: Dark navy ground, one accent (user preference)

## Current Pain Points

1. **Login friction**: Manual passphrase entry, profile selection, daemon restarts
2. **Token entry**: X-Ghost-Token manually entered (now fixed with dev-mode bypass)
3. **Peer display**: Shows "rust-libp2p/0.47.0" instead of nicknames (now fixed)
4. **Message ordering**: Newest-first (now fixed to oldest-first)
5. **Private vs broadcast**: "All Nodes" showed both (now fixed)
6. **Unread tracking**: Shows total count, not per-conversation
7. **Delivery receipts**: Field + UI icons exist, backend not wired
8. **Typing indicators**: Not implemented
9. **Message search**: Not implemented
10. **Attachments**: Not implemented
11. **Offline queue**: Partially implemented

## Redesign Priorities

### Phase 1: Automatic UX (eliminate friction)
- Auto-unlock on startup (already works via refreshIdentity)
- Auto-connect to known peers (mDNS + Kademlia)
- Auto-select transport (transport selector already runs)
- Auto-escalate privacy tier based on conditions
- Session persistence (remember unlocked state across page refreshes)
- Password recovery (Shamir secret sharing - already implemented, needs UI)

### Phase 2: Conversation UX
- Per-conversation unread badges (not total count)
- Delivery receipts (sent → delivered → read)
- Typing indicators
- Auto-scroll to latest message
- Message grouping by sender
- Date separators
- Reply threading
- Message reactions

### Phase 3: Advanced Features
- Message search (full-text)
- Attachments (file transfer)
- Offline queue (store-and-forward with UI indicator)
- Privacy tier selector (L0-L3 toggle)
- Transport mode selector (auto/libp2p-only/reticulum-only/hybrid)
- Power user panel (advanced settings)

### Phase 4: Power User Features
- Manual transport selection
- Privacy tier override
- Cover traffic toggle
- Stealth mode toggle
- RNS announce interval tuning
- Sphinx hop count configuration
- Debug/telemetry panel

## Component Redesign

### Login Screen
- **Current**: Profile selection + passphrase entry
- **New**: 
  - Auto-detect existing profiles
  - One-click unlock for returning users
  - "Forgot password" → Shamir recovery flow
  - Create new profile (guided wizard)
  - QR code / copy-paste invite token for joining networks
  - Dark theme by default

### Main Chat View
- **Current**: Broadcast/DM tabs, message list, send box
- **New**:
  - Conversation list (left sidebar) with unread badges
  - Active conversation (center) with message grouping
  - Peer info panel (right sidebar, collapsible)
  - Transport status indicator (subtle, not intrusive)
  - Privacy tier indicator (color-coded)
  - Typing indicator
  - Delivery receipt icons

### Settings Panel
- **Current**: Basic settings
- **New**:
  - Simple/Advanced toggle
  - Simple: privacy tier (L0-L3 slider), transport mode (auto/manual), notifications
  - Advanced: all transport parameters, crypto settings, debug options
  - Profile management (create, delete, backup, recover)
  - Network settings (bootstrap nodes, IFAC seed, invite tokens)

### Peer Discovery
- **Current**: Peer list with status
- **New**:
  - Visual peer map (optional, privacy-aware)
  - Connection quality indicators
  - Transport used per peer
  - Trust level display
  - Block/unblock controls
  - Invite new peers (copy-paste token, QR code)

## Design Tokens

### Colors (dark theme)
- Background: #0a0e1a (dark navy)
- Surface: #111827 (slightly lighter)
- Border: #1f2937
- Text: #e5e7eb
- Muted: #9ca3af
- Accent: #3b82f6 (blue)
- Success: #10b981 (green)
- Warning: #f59e0b (amber)
- Error: #ef4444 (red)
- Privacy L0: #6b7280 (gray)
- Privacy L1: #3b82f6 (blue)
- Privacy L2: #8b5cf6 (purple)
- Privacy L3: #ef4444 (red)

### Typography
- Font: Inter or system-ui
- Monospace: JetBrains Mono or system monospace
- Base size: 14px
- Line height: 1.5

### Spacing
- Base unit: 4px
- Component padding: 8px, 12px, 16px
- Section gaps: 16px, 24px, 32px

### Border Radius
- Small: 4px
- Medium: 8px
- Large: 12px
- Full: 9999px

## Implementation Strategy

1. **Update incrementally**: Each new wiring gets a UI update
2. **Feature flags**: Hide advanced features behind power user toggle
3. **Progressive enhancement**: Basic functionality first, polish later
4. **Accessibility**: Keyboard navigation, screen reader support, high contrast mode
5. **Responsive**: Works on desktop and mobile (Tauri)

## Files to Modify

- `webui/src/components/panels/Login.tsx` — auto-unlock, recovery flow
- `webui/src/components/panels/Communication.tsx` — conversation UX, delivery receipts
- `webui/src/components/layout/Sidebar.tsx` — per-conversation unread badges
- `webui/src/components/panels/Settings.tsx` — simple/advanced toggle
- `webui/src/context/AppContext.tsx` — session persistence, auto-connect
- `webui/src/services/api.ts` — new API endpoints for power user features

## Not Started

This is a planning document. Implementation should begin after:
1. RNS address exchange fix verified
2. Sealed sender wired
3. Message delivery receipts wired
EOF
echo "created"