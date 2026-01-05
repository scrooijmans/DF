# Local-First Software Design Guidelines

This document provides guidelines for implementing local-first architecture in this project. All backend and data layer design decisions should adhere to these principles.

## What is Local-First?

Local-first software prioritizes the use of local storage (the disk on the user's device) and local networks over servers in remote data centers. The copy of data on your local device is treated as the **primary copy**, not subordinate to any server. Servers exist to assist with synchronization across devices, but are not required for core functionality.

**Core Philosophy**: "In local-first software, the availability of another computer should never prevent you from working."

## The Seven Ideals of Local-First Software

### 1. Fast - No Spinners
- All operations should be handled by reading/writing to local storage
- Never require network round-trips for basic operations
- User input should result in near-instantaneous response
- Data synchronization happens quietly in the background

**Implementation Guidelines:**
- Design all read operations to work from local data first
- Queue write operations locally before syncing
- Use optimistic UI patterns where changes appear immediately
- Never block the UI waiting for network responses

### 2. Multi-Device Support
- Users work across multiple devices (phone, tablet, laptop)
- Data must synchronize seamlessly across all devices
- Each device maintains a complete, functional copy of the data

**Implementation Guidelines:**
- Design data schemas that support merging from multiple sources
- Implement sync protocols that handle intermittent connectivity
- Store device-specific state separately from shared data
- Plan for conflict resolution from the start

### 3. Works Offline
- Full functionality must be available without network connectivity
- The app should be usable in airplanes, elevators, tunnels, rural areas
- Network is optional, not required

**Implementation Guidelines:**
- Never require authentication checks that depend on network
- Cache all necessary application code and data locally
- Design for "offline-first" - assume no network, then add sync as enhancement
- Test all features with network disabled

### 4. Seamless Collaboration
- Support real-time collaboration equal to or better than cloud apps
- Multiple users can edit simultaneously without coordination overhead
- Support various collaboration workflows (real-time editing, suggestions, review)

**Implementation Guidelines:**
- Use CRDTs (Conflict-free Replicated Data Types) for shared data structures
- Design data models that support automatic merging
- Provide visibility into collaborator presence and changes
- Support both synchronous and asynchronous collaboration patterns

### 5. Long-Term Preservation (The Long Now)
- Data should remain accessible indefinitely
- Software should continue working even if the company shuts down
- Avoid dependency on external services for core functionality

**Implementation Guidelines:**
- Use open, documented file formats (JSON, SQLite, plain text)
- Avoid proprietary binary formats without escape hatches
- Store data in locations accessible to users
- Document data formats for future interoperability
- Consider: "Will this data be readable in 20 years?"

### 6. Security and Privacy by Default
- Local devices store only the user's own data
- Avoid centralized databases holding everyone's data
- Use end-to-end encryption for any data that leaves the device

**Implementation Guidelines:**
- Encrypt data before transmission to any server
- Never store unencrypted user data on servers
- Design so servers cannot read user data
- Minimize data collection - only store what's necessary
- Implement zero-knowledge sync where possible

### 7. User Control and Ownership
- Users retain full ownership of their data
- No artificial restrictions on data access
- Users can copy, export, backup, and delete their data freely

**Implementation Guidelines:**
- Provide comprehensive data export functionality
- Allow users to store data in locations they control
- Never lock users into proprietary ecosystems
- Support data portability between applications
- Avoid "pay or we delete your data" models

## What IS NOT Local-First

Understanding what local-first is NOT helps clarify the architecture:

- **Local-only is NOT local-first** - Local-first implies multiplayer or at least multi-device sync
- **Doesn't work offline? NOT local-first** - Offline support is mandatory
- **Stops working if vendor shuts down? NOT local-first** - Software must be "incredible-journey-proof"
- **Pure peer-to-peer without persistence? Problematic** - What if all peers are offline?

## Architecture Patterns

### Data Storage Hierarchy

```
Primary:    Local Device Storage (SQLite, IndexedDB, file system)
Secondary:  Sync Server (encrypted, holds copies for sync)
Optional:   Peer-to-peer (for real-time, same-network sync)
```

### Sync Architecture Options (Best to Worst)

1. **Multiple Interoperable Sync Providers** (Best)
   - Open standard sync protocol
   - Users can choose or switch providers
   - Maximum resilience and user control

2. **Sync via Cloud Storage** (Good)
   - Dropbox/Google Drive/iCloud as sync layer
   - Less likely to disappear
   - OK for files, not great for real-time

3. **Peer-to-Peer**
   - Direct device-to-device sync
   - Challenges: signaling, NAT traversal, offline peers

4. **Open Source Self-Hostable Server**
   - Better than proprietary, but requires technical expertise

5. **Proprietary Server** (Worst)
   - Not incredible-journey-proof
   - Avoid if possible

### Conflict Resolution Strategy

Use CRDTs for automatic conflict resolution:

```
Traditional Approach (Problematic):
1. User A edits document
2. User B edits same document offline
3. Both sync -> CONFLICT -> manual resolution required

CRDT Approach (Preferred):
1. User A edits document (operations recorded)
2. User B edits same document (operations recorded)
3. Both sync -> operations merge automatically -> consistent state
```

**Recommended CRDT Libraries:**
- Automerge (JavaScript/Rust)
- Yjs (JavaScript)
- Loro (Rust)
- Electric SQL (for relational data)

## Implementation Checklist

### Data Layer
- [ ] Primary data storage is on-device
- [ ] All operations work without network
- [ ] Data format is documented and open
- [ ] Export functionality available
- [ ] Sync is additive, not required

### Sync Layer
- [ ] End-to-end encryption for all synced data
- [ ] Conflict resolution is automatic (CRDTs)
- [ ] Sync failures are handled gracefully
- [ ] Progress/status is visible to users
- [ ] Works across multiple devices

### User Experience
- [ ] No loading spinners for local operations
- [ ] Clear offline/online status indicators
- [ ] Immediate response to user actions
- [ ] Collaboration presence awareness
- [ ] Data ownership is clear to users

### Resilience
- [ ] App works if company server is unreachable
- [ ] Data survives device loss (if sync enabled)
- [ ] No single point of failure
- [ ] Graceful degradation under network issues

## Benefits to Consider

### For Users
- Fast, responsive applications
- Work anywhere, anytime
- True data ownership
- Privacy by default
- Long-term data preservation

### For Developers
- No backend engineering team required for many apps
- No 24/7 on-call rotation for server infrastructure
- No complex network error handling code
- Reduced cloud hosting costs
- Simpler overall architecture

## Technology Recommendations

### For Document/Rich Data
- **Automerge**: Production-ready CRDT library (Rust core, JS/Swift bindings)
- **Yjs**: High-performance CRDT for collaborative editing

### For Relational Data
- **SQLite**: Local database with excellent durability
- **Electric SQL**: PostgreSQL sync to local SQLite
- **PowerSync**: Sync layer for SQLite

### For File Storage
- Local file system with cloud storage sync (Dropbox/iCloud)
- Content-addressed storage for deduplication

### For Sync Infrastructure
- WebSocket for real-time sync
- HTTP for batch sync
- Local network discovery for same-network sync

## References

- Original Paper: "Local-First Software: You Own Your Data, in spite of the Cloud" (Kleppmann et al., 2019)
- Ink & Switch: https://www.inkandswitch.com/local-first/
- Automerge: https://automerge.org/
- Martin Kleppmann's work: https://martin.kleppmann.com/

---

**Remember**: Local-first is a set of principles and values. It's a better way of building software, and that is more important than any particular technology, tool, or product.
