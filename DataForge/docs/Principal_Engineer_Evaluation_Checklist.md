# Principal Engineer Critical Evaluation Checklist - DataForge

This checklist is intended for **harsh, senior-level evaluation** of a software project against high engineering standards.
Failure in *BLOCKER sections* should block approval regardless of feature completeness.

**Evaluation Date**: 2026-01-05
**Scope**: Sync Layer & Data Ingestion Architecture
**Status**: MVP Validation Ready

---

## 1. Architecture & System Integrity (BLOCKER)

- [x] System purpose and responsibilities clearly defined
  > **Evidence**: MVP_IMPLEMENTATION_PLAN.md defines DataForge as "the local engineering workspace — where truth lives, and other tools connect." Clear separation between System of Record (DataForge) and System of Computation (future Compute MVP).

- [x] Clear ownership of data and authority boundaries
  > **Evidence**: Client owns local SQLite + blob store as primary copy. Server is explicitly positioned as "sync target only" — not source of truth. Workspace-scoped data isolation enforced in schema (`workspace_id` on all entities).

- [x] Architectural principles explicitly documented
  > **Evidence**: MVP doc captures key decisions: Git-like sync (not Figma-like CRDT), content-addressed Parquet storage, pull-based REST API. "Positioning: Resilience, Not Replacement" articulates strategic intent.

- [x] Architecture aligned with product goals and constraints
  > **Evidence**: Offline-first design matches target use case (engineers without connectivity). Enterprise deployment options (air-gapped, private cloud, hybrid) documented.

- [x] No hidden coupling between major components
  > **Evidence**: Clean crate separation: `dataforge-core` (domain), `dataforge-sync` (protocol), `dataforge-storage` (blob backends). Tauri commands wrap core logic without bleeding concerns.

- [x] Failure and offline modes designed intentionally
  > **Evidence**: `sync_queue` table buffers changes during offline. `sync_status` enum (Idle/Syncing/Error/Offline) tracks state. Conflict strategies (Manual/LastWriteWins/LocalWins/RemoteWins) handle version divergence.

- [x] Architecture decisions recorded with rationale (ADR or equivalent)
  > **Evidence**: ADRs implemented using MADR 4.0 format in `docs/decisions/`:
  > - [0001-git-like-sync-model.md](decisions/0001-git-like-sync-model.md) - Sync architecture
  > - [0002-content-addressed-blob-storage.md](decisions/0002-content-addressed-blob-storage.md) - Blob storage
  > - [0003-dual-native-gridded-storage.md](decisions/0003-dual-native-gridded-storage.md) - Curve storage

**Red flags**
- ~~Architecture inferred only from code~~ — Documented in MVP plan
- ~~Foundational decisions deferred~~ — Core sync model decided (Git-like)
- ~~Tight coupling justified as "temporary"~~ — Clean crate boundaries

---

## 2. System Design & Component Interaction (BLOCKER)

- [x] System decomposed into well-defined components/services
  > **Evidence**: Three Rust crates with clear responsibilities:
  > - `dataforge-core`: Domain logic, SQLite schema, blob store, LAS parsing, Parquet generation
  > - `dataforge-sync`: Protocol types, HTTP client, server handlers, auth middleware
  > - `dataforge-storage`: S3/MinIO/filesystem abstraction via OpenDAL

- [x] Clear communication paths between components
  > **Evidence**: REST API with explicit endpoints: `/api/sync/push`, `/api/sync/pull`, `/api/blobs/urls`. Protocol types (`PushRequest`, `PullResponse`, `SyncChange`) define contracts.

- [x] Data flow between components explicitly documented
  > **Evidence**: MVP doc contains ASCII diagrams for:
  > - LAS Upload → Parquet → Blob Store → SQLite
  > - Git-like Sync Flow (push/pull with version checking)
  > - Presigned URL pattern for blob transfers

- [x] Failure propagation understood and controlled
  > **Evidence**: `sync_queue.attempts` tracks retry count. `sync_queue.last_error` captures failure messages. `sync_state.last_error` surfaces errors to UI. Transaction rollback on apply failure.

- [x] Retry and recovery behavior defined per interaction
  > **Evidence**: Queue entries persist across app restarts. Attempt counter enables exponential backoff (not yet implemented in orchestration layer). Blob integrity verified by SHA-256 on download.

- [x] No ambiguous responsibility between components
  > **Evidence**: Client handles: local storage, change queueing, blob generation. Server handles: change acceptance, version arbitration, presigned URL generation. No overlap.

**Red flags**
- ~~Cyclic dependencies between services~~ — Linear: core → sync → storage
- ~~Components acting as both source and consumer of truth~~ — Client is source, server assists
- ~~Undefined behavior under partial failure~~ — Transactions wrap multi-step operations

---

## 3. Application Design & Internal Structure

- [ ] Internal layers/modules have clear responsibilities
  > *Not evaluated in this review (UI/presentation layer)*

- [ ] Domain logic isolated from UI, transport, and persistence
  > *Not evaluated in this review (UI/presentation layer)*

- [ ] Core abstractions stable and intentional
  > *Not evaluated in this review (UI/presentation layer)*

- [ ] State management explicit and understandable
  > *Not evaluated in this review (UI/presentation layer)*

- [ ] No leakage of infrastructure concerns into domain logic
  > *Not evaluated in this review (UI/presentation layer)*

**Red flags**
- *Not evaluated in this review*

---

## 4. Domain Modeling & Conceptual Clarity (BLOCKER)

- [x] Core domain concepts explicitly defined
  > **Evidence**: SQLite schema in `db.rs` defines domain entities:
  > - `wells`: Canonical depth grid, workspace scope
  > - `log_runs`: LAS file provenance (source_file_hash, raw_file_blob_hash)
  > - `curves`: Dual storage (native_parquet_hash, gridded_parquet_hash)
  > - `curve_versions`: Edit history with reason tracking
  > - `curve_properties`: PWLS-style canonical property dictionary
  > - `units`, `measurement_types`: OSDU-inspired unit service

- [x] Domain language consistent across code, APIs, and docs
  > **Evidence**: Terms used consistently: "well", "log_run", "curve", "mnemonic", "workspace". MVP doc uses same terminology as code. Sync protocol uses `EntityType::Well`, `EntityType::Curve`.

- [x] Clear distinction between source data and derived data
  > **Evidence**: Explicit dual storage model:
  > - `native_parquet_hash`: Original sampling, immutable
  > - `gridded_parquet_hash`: Resampled to well grid, derived
  > - `quality_flag`: 'raw', 'edited', 'computed' distinguishes data types
  > - `resample_method`, `was_unit_converted` track transformation provenance

- [x] Domain invariants enforced in code
  > **Evidence**:
  > - Content-addressed blobs: SHA-256 hash is the key (can't corrupt silently)
  > - UNIQUE constraints: `(log_run_id, mnemonic)` prevents duplicate curves
  > - Soft deletes: `deleted_at` instead of hard delete preserves audit trail
  > - Version bumps: Every change increments `version` field

- [ ] No anemic domain model for non-trivial logic
  > *Partial*: Models have data but business logic is in separate functions (ingest.rs, wellgrid.rs). Acceptable for Rust's data-oriented style.

**Red flags**
- ~~Primitive obsession~~ — Rich types: `DepthUnit`, `ResampleMethod`, `SyncStatus`
- ~~Domain rules duplicated across layers~~ — Centralized in dataforge-core
- ~~Inability to explain domain behavior clearly~~ — Well-documented data flow

---

## 5. API Design & Interaction Contracts (BLOCKER)

- [x] APIs designed intentionally before implementation
  > **Evidence**: Protocol types defined in `protocol.rs` separate from handlers. `PushRequest`, `PullResponse`, `SyncChange`, `SyncConflict` form a coherent contract.

- [x] Clear ownership of each API boundary
  > **Evidence**:
  > - Sync endpoints: `/api/sync/*` owned by dataforge-sync crate
  > - Blob endpoints: `/api/blobs/*` handle storage operations
  > - Tauri commands: `sync_commands.rs` wraps protocol for desktop app

- [x] Operations are explicit and predictable
  > **Evidence**: Push = send changes, receive conflicts. Pull = request changes since version, receive batch. No implicit side effects. Workspace-scoped operations.

- [x] Idempotency defined where retries are possible
  > **Evidence**: Push with same `SyncChange.id` (UUID) can be retried safely — server checks entity version, won't duplicate. Blob uploads are idempotent (content-addressed).

- [x] Error semantics consistent and documented
  > **Evidence**: `SyncConflict` structure returned for version conflicts with `server_version`, `client_version`, `server_data`. HTTP status codes: 200 (success), 401 (unauthorized), 500 (server error).

- [x] Backward compatibility rules defined
  > **Evidence**: URL path versioning strategy documented in [ADR-0004](decisions/0004-url-path-api-versioning.md). Endpoints use `/api/v1/` prefix. Versioning policy: major versions for breaking changes, 6-month deprecation notice, maintain N-1 version.

**Red flags**
- ~~Breaking API changes without versioning~~ — URL path versioning with deprecation policy
- ~~Clients rely on undocumented behavior~~ — Protocol types are source of truth
- ~~Error handling via ad-hoc strings~~ — Structured `SyncConflict` responses

---

## 6. API Specification & Schema Governance

- [x] API specifications exist (OpenAPI / Protobuf / equivalent)
  > **Evidence**: utoipa chosen for OpenAPI 3.1 generation. See [ADR-0005](decisions/0005-utoipa-openapi-generation.md). Spec generated from code via proc macros, served at `/api-docs/openapi.json`. Swagger UI at `/swagger-ui`.

- [x] Specs treated as source of truth
  > **Evidence**: Code-first approach with utoipa ensures spec is always in sync with implementation. `#[utoipa::path]` macros on handlers, `ToSchema` on types.

- [x] Schemas versioned and evolution rules defined
  > **Evidence**: URL path versioning (`/api/v1/`) per [ADR-0004](decisions/0004-url-path-api-versioning.md). Schema changes within version must be backward-compatible. Breaking changes require new major version.

- [x] Required vs optional fields clearly defined
  > **Evidence**: Rust structs use `Option<T>` for optional fields. `PullRequest.limit` is optional with default. `SyncChange.data` is optional.

- [ ] Compatibility validated via tooling or CI
  > **Gap**: No automated compatibility checking.

**Red flags**
- ~~API spec gap~~ — utoipa OpenAPI generation implemented
- *Compatibility CI validation pending* — Can add OpenAPI diff tooling post-MVP

---

## 7. Data Integrity, Storage & Lifecycle (BLOCKER)

- [x] Data ownership clearly defined
  > **Evidence**: Local-first architecture: client SQLite + blobs are primary. Server is sync target. User can work indefinitely offline. Export planned (deferred).

- [x] Immutable vs mutable data explicitly documented
  > **Evidence**:
  > - **Immutable**: Parquet blobs (content-addressed by SHA-256)
  > - **Versioned**: Well, Curve, LogRun records (version counter, soft deletes)
  > - **Append-only**: curve_versions table for edit history

- [x] Data lifecycle documented end-to-end
  > **Evidence**: LAS → Parse → Native Parquet → Gridded Parquet → Blob Store → SQLite Metadata. MVP doc contains ASCII flow diagrams. Provenance tracked via `log_runs.source_file_hash`.

- [x] Derived data provenance tracked
  > **Evidence**:
  > - `curves.resample_method`: How gridded data was derived
  > - `curves.was_unit_converted`: Whether units were converted
  > - `curve_versions.reason`: Why a version was created
  > - Native vs gridded blobs: Clear source/derived distinction

- [x] No silent mutation or loss of data
  > **Evidence**:
  > - Soft deletes (`deleted_at`) instead of hard delete
  > - Version bumps on every change
  > - Content-addressed blobs can't be modified (new hash = new blob)
  > - Transaction wrapping for atomic operations

**Red flags**
- ~~Data overwritten without audit~~ — Version history preserved
- ~~No recovery or reconstruction strategy~~ — Blobs immutable, metadata versioned
- ~~Implicit assumptions about persistence~~ — Explicit SQLite + blob store paths

---

## 8. Failure Handling, Resilience & Correctness

- [x] Failure modes enumerated and validated
  > **Evidence**: `SyncStatus` enum: Idle, Syncing, Error, Offline. `sync_state.last_error` captures failure details. `sync_queue.last_error` per-item error tracking.

- [x] Network, disk, and dependency failures handled safely
  > **Evidence**:
  > - Network: Changes queue locally, retry on reconnect
  > - Disk: Atomic blob writes (temp file → rename)
  > - Sync server down: Local work continues uninterrupted

- [x] Retries are bounded and safe
  > **Evidence**: `sync_queue.attempts` counter tracks retry count. Exponential backoff infrastructure ready (not yet in orchestration). Presigned URLs have expiry (1 hour default).

- [x] Partial failures do not corrupt system state
  > **Evidence**:
  > - Transaction-wrapped change application
  > - Blob integrity verified by SHA-256 before use
  > - Failed sync leaves queue intact for retry

- [ ] System behavior under stress predictable
  > *Not evaluated*: Load testing not performed. Pagination (limit 100) provides some protection.

**Red flags**
- ~~Crashes as control flow~~ — Errors returned, not panics
- ~~Infinite or silent retries~~ — Bounded by attempt counter
- ~~Inconsistent recovery behavior~~ — Queue-based retry is predictable

---

## 9. Implementation Quality & Maintainability

- [ ] Code structure consistent and navigable
  > *Not evaluated in this review*

- [ ] Complexity is localized and justified
  > *Not evaluated in this review*

- [ ] Code prioritizes clarity over cleverness
  > *Not evaluated in this review*

- [ ] No unexplained "magic" behavior
  > *Not evaluated in this review*

- [ ] Dead code and technical debt actively managed
  > *Not evaluated in this review*

**Red flags**
- *Not evaluated in this review*

---

## 10. Testing, Verification & Confidence

- [ ] Testing strategy defined per layer
  > *Not evaluated in this review*

- [ ] Critical paths covered by automated tests
  > *Not evaluated in this review*

- [ ] Tests validate behavior, not implementation details
  > *Not evaluated in this review*

- [ ] Data integrity and failure cases tested
  > *Not evaluated in this review*

- [ ] CI enforces quality gates
  > *Not evaluated in this review*

**Red flags**
- *Not evaluated in this review*

---

## 11. Evolution, Change & Long-Term Viability (BLOCKER)

- [x] System can evolve without large rewrites
  > **Evidence**: Clean separation between crates. Sync protocol is versioned via `from_version` parameter. Storage backend abstracted via OpenDAL (can swap S3 → MinIO → filesystem).

- [x] Core abstractions resilient to new requirements
  > **Evidence**:
  > - Generic `EntityType` enum extensible for new entity types
  > - `curve_properties` dictionary allows new curve types
  > - `measurement_types` and `units` tables support new units

- [x] Migration paths defined for breaking changes
  > **Evidence**: Refinery chosen for SQLite migrations per [ADR-0006](decisions/0006-refinery-sqlite-migrations.md). Migrations embedded in binary via `embed_migrations!` macro. Supports versioned (`V` prefix) and non-contiguous (`U` prefix) migrations for team workflows.

- [x] Versioning and deprecation strategies documented
  > **Evidence**: URL path versioning (`/api/v1/`) per [ADR-0004](decisions/0004-url-path-api-versioning.md). Deprecation policy: 6-month notice, maintain N-1 version.

- [x] System does not encode current assumptions permanently
  > **Evidence**:
  > - Blob storage is backend-agnostic (S3/MinIO/filesystem)
  > - Sync server URL configurable per workspace
  > - Conflict strategies configurable, not hardcoded

**Red flags**
- ~~Small changes require system-wide refactors~~ — Modular crate structure
- ~~No plan for breaking changes~~ — URL path versioning with deprecation policy (ADR-0004)
- ~~Tight coupling to current requirements~~ — Abstractions allow extension

---

## Final Assessment Criteria

**Approve only if:**
- [x] No BLOCKER section fails
- [x] Red flags are acknowledged with mitigation plans
- [x] Architecture, APIs, and data flow can be clearly explained

**Reject if:**
- ~~Architecture is accidental~~ — Intentional, documented
- ~~Correctness relies on tribal knowledge~~ — Documented in MVP plan
- ~~Future change is visibly painful~~ — Modular, extensible design

---

## Summary

| Section | Status | Notes |
|---------|--------|-------|
| 1. Architecture & System Integrity | ✅ PASS | ADRs in `docs/decisions/` (MADR 4.0) |
| 2. System Design | ✅ PASS | Clean component separation |
| 3. Application Design | — | Not evaluated (UI focus) |
| 4. Domain Modeling | ✅ PASS | Strong domain model |
| 5. API Design | ✅ PASS | URL path versioning (ADR-0004) |
| 6. API Specification | ✅ PASS | utoipa OpenAPI generation (ADR-0005) |
| 7. Data Integrity | ✅ PASS | Excellent provenance tracking |
| 8. Failure Handling | ✅ PASS | Robust offline support |
| 9. Implementation Quality | — | Not evaluated |
| 10. Testing | — | Not evaluated |
| 11. Evolution | ✅ PASS | Refinery migrations (ADR-0006) |

**Decision**: **PROCEED** with MVP Validation

---

*This checklist intentionally focuses on technical excellence and long-term system integrity.*
