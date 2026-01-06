# Use Git-Like Pull-Based Sync Instead of Real-Time CRDT

- **Status**: accepted
- **Date**: 2024-12-14
- **Decision-makers**: Engineering team
- **Consulted**: Domain experts (subsurface engineering workflows)
- **Informed**: Stakeholders

## Context and Problem Statement

DataForge needs to synchronize well log data between desktop clients and a central server. Users work offline (on rigs, traveling, remote locations) and need to share data with team members when connectivity is available. How should we design the sync model?

## Decision Drivers

- **Offline-first**: Full functionality without network connectivity
- **Enterprise deployment**: Air-gapped environments, strict security requirements
- **Data integrity**: Well log data is mission-critical; no silent data loss
- **Simplicity**: Maintainable with a small team
- **Mental model**: Engineers understand version control (Git, SVN)
- **Conflict frequency**: Low - users typically work on different wells

## Considered Options

1. **Git-like pull-based sync** (version vectors, manual conflict resolution)
2. **Real-time CRDT sync** (Yjs/Automerge, automatic merging, WebSocket)
3. **Cloud-native sync** (Supabase Realtime, PowerSync)

## Decision Outcome

**Chosen option**: "Git-like pull-based sync", because it provides the simplest architecture that meets our requirements, aligns with users' mental model of version control, and avoids the complexity of real-time collaboration that our use case doesn't require.

### Consequences

**Positive:**
- Simpler architecture (no WebSocket, no CRDT complexity)
- Works naturally offline (queue changes, sync when online)
- Clear conflict model users understand (like Git merge conflicts)
- Lower infrastructure requirements (REST API, SQLite)
- Deployable in air-gapped environments
- Server doesn't need to track client state

**Negative:**
- No real-time collaboration (users don't see each other's changes immediately)
- Conflicts require manual resolution (acceptable given low conflict frequency)
- Less "magical" user experience compared to Figma-like collaboration

## Confirmation

- Architecture review confirms pull-based sync throughout
- Sync protocol tests verify offline queue → push → pull → apply flow
- Conflict resolution UI tested with simulated version conflicts

## Pros and Cons of Options

### Git-like Pull-Based Sync

Version-based conflict detection, REST API, client-initiated sync.

- **Good**: Simple architecture, minimal infrastructure
- **Good**: Natural offline support (changes queue locally)
- **Good**: Familiar mental model for engineers
- **Good**: Server is stateless regarding client presence
- **Good**: Works in air-gapped environments
- **Bad**: No real-time collaboration
- **Bad**: Conflicts require user intervention
- **Neutral**: Sync is explicit (user triggers or periodic)

### Real-Time CRDT Sync (Yjs/Automerge)

Automatic merging via CRDT data structures, WebSocket for push updates.

- **Good**: Real-time collaboration like Figma/Notion
- **Good**: Automatic conflict resolution (no user intervention)
- **Good**: Seamless multi-user editing
- **Bad**: Significant complexity (CRDT semantics, WebSocket infrastructure)
- **Bad**: Harder to deploy in restricted networks
- **Bad**: Server must track client connections
- **Bad**: Overkill for our use case (users rarely edit same data)
- **Bad**: Learning curve for team

### Cloud-Native Sync (Supabase Realtime)

Managed sync service with real-time subscriptions.

- **Good**: Less infrastructure to manage
- **Good**: Real-time features built-in
- **Bad**: Requires constant connectivity to cloud
- **Bad**: Cannot deploy air-gapped
- **Bad**: Vendor dependency
- **Bad**: Complex offline story (PowerSync adds complexity)
- **Bad**: Previous implementation had reliability issues

## More Information

**Related documents:**
- [MVP Implementation Plan](../../MVP_IMPLEMENTATION_PLAN.md) - Section "Key Architectural Decisions"
- [Local-First Guidelines](../../../LOCAL_FIRST_GUIDELINES.md) - Principles this decision follows
- [Principal Engineer Evaluation](../Principal_Engineer_Evaluation_Checklist.md) - Architecture review

**Inspirations:**
- **Pijul**: Push/pull protocol design
- **Harbor**: Presigned URL pattern for blob transfers
- **ColaNode**: Account-workspace-member model

**Implementation files:**
- `crates/dataforge-sync/src/protocol.rs` - Sync protocol types
- `crates/dataforge-core/src/sync/` - Client-side sync state
- `src-tauri/src/sync_commands.rs` - Tauri command interface
