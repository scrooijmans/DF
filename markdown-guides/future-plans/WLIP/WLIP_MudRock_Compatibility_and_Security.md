# WLIP × MudRock Compatibility & Security Plan

## TL;DR
- **Compatible & complementary**: MudRock’s stack (Supabase + Kong + PowerSync + DuckDB on Hetzner/Dokploy) fits naturally as the **Collaboration**, **Auth**, and **Deployment** substrate for the WLIP architecture.
- **Not replaced**: WLIP is an **integration/gov layer** (ontology + provenance + adapters). MudRock can implement/host that layer today with minimal refactors.
- **Security-first path**: Adopt **tenant isolation + RLS + VPN-only ingress + JWT/OIDC**, and formalize **provenance + immutable audit** to reach WLIP’s audit goals.

---

## 1) Where MudRock Fits in WLIP

| WLIP Layer | What’s Needed | What MudRock Already Provides | Action to Align |
|---|---|---|---|
| **Presentation** | Web UI for collaboration, review, version diff | Tauri/Svelte UI, real-time presence via PowerSync | Add graph views for **DAG lineage** + review/approve states |
| **Integration** | Adapters to domain tools (Petrel/Techlog/WellPlan) | REST gateway via Kong; Supabase APIs; offline sync | Ship **adapter SDK**; register lineage on ingest/export |
| **Data/Provenance** | RDF/JSON‑LD metadata; PROV‑O DAG; Parquet/HDF5 at rest | Parquet at rest; lineage tracker scaffolding; DuckDB/DataFusion | Implement **PROV‑O model** + immutable event log; map OSDU/WITSML ids |
| **Collaboration & Audit** | Comments, approvals, immutable history | Auth, RLS, PowerSync real-time; audit logs | Add **CDE-like states** (WIP → Shared → Published → Archived) and WORM logs |
| **Extensibility** | UDFs/operators; sandboxed compute | Rust crates + planned WASM; custom operator registry | Finalize **operator versioning** and sandbox execution policy |

---

## 2) Security & Confidentiality Model (WLIP-ready)

### 2.1 Data-at-Rest & In-Transit
- **At rest**: Parquet/HDF5/Objects encrypted (AES-256) on Supabase Storage or enterprise object store.
- **In transit**: TLS everywhere (gateway → services; client → gateway).

### 2.2 Identity, AuthN/Z
- **Identity**: Supabase Auth (JWT/OIDC) for app users; service accounts for adapters.
- **AuthZ**: **Row Level Security (RLS)** for tables; **dataset/per‑well ACLs** at the app layer; **workspace tenancy** enforced via schema prefixes or row scoping.
- **Sessions**: Short‑lived JWTs, refresh tokens; rotate keys; optional MFA/SAML for enterprise.

### 2.3 Network Posture
- **Ingress**: Single **Kong API Gateway**; **VPN‑only** or allow‑list for operators; optional private connectivity (WireGuard/ExpressRoute).  
- **Egress**: Deny by default; explicit destinations for adapters; audit outbound calls.
- **Isolation**: Per‑tenant buckets/namespaces; separate DB schemas; optional per‑tenant deployments for high‑sensitivity assets.

### 2.4 Governance & Audit
- **Immutable audit log** (append‑only/WORM) for: access, changes, approvals, pipeline runs.
- **Provenance graph** (**W3C PROV‑O**) for every transformation (Entity–Activity–Agent).
- **CDE‑style lifecycle**: WIP → Shared → Published → Archived with role‑gated transitions.
- **Backups/DR**: Automated snapshots + tested restore; PITR; retention policies by project.

### 2.5 Secrets & Compliance
- **Secrets**: Never in code or docs; store in **Vault/SOPS**; rotate on schedule.
- **Compliance accelerators**: Audit exports, data deletion workflow, legal hold, and per‑region residency (optional per‑region deployments).

---

## 3) Deployment Patterns

| Pattern | When to Use | Notes |
|---|---|---|
| **Single‑tenant VPS** (Hetzner/Dokploy) | Small teams, pilots | Fast to run; VPN + Kong gateway; low cost. |
| **Multi‑tenant VPS** | Central platform for multiple teams | Strong tenancy boundaries (schemas/buckets); per‑tenant RBAC & quotas. |
| **On‑prem / air‑gapped** | High‑security operators | Same containers behind corporate ingress; offline PowerSync still works. |
| **Hybrid** | Field work offline + central hub | PowerSync handles offline → central sync; conflict policy by table. |

**Operational add‑ons**: monitoring (Prometheus/Grafana), SIEM export, SSO, automated backups, IaC for repeatable installs.

---

## 4) Concrete Gaps to Close (High Impact)

1) **Per‑well ACLs & workspace tenancy**  
   – Define ACL tables + RLS policies; enforce in UI and API.  
2) **Immutable audit + PROV‑O lineage**  
   – Append‑only audit table; hash code/params/data for each run; materialize PROV‑O as JSON‑LD.  
3) **CDE lifecycle & approvals**  
   – Add states + transitions; require approvals/signatures; capture rationale/attachments.  
4) **Adapter SDK + registration hooks**  
   – Adapters emit “ingest/export” events with source refs, checksums, and schema mappings.  
5) **Secret hygiene & zero‑trust**  
   – Move all keys to a secret store; rotate; least‑privilege service roles.

---

## 5) Mapping: MudRock Components → WLIP Controls

- **Supabase Auth + JWT** → identity/session; add SAML/MFA for enterprise.  
- **RLS policies** → fine‑grained table access; extend to dataset/per‑well scopes.  
- **Kong API Gateway** → single choke‑point: authN/Z, rate‑limit, IP allow‑list, WAF.  
- **PowerSync** → offline‑first collaboration, conflict resolution; works with VPN‑only ingress.  
- **DuckDB/DataFusion** → fast local/remote analytics; register results into provenance graph.  
- **Rust DAG + operator registry** → execution engine; emit PROV‑O + WORM audit on each node.  

---

## 6) Will WLIP “wipe out” MudRock? (No.)

WLIP is a **reference architecture** and governance layer. MudRock is an **implementation** that already delivers: Auth, gateway, storage, offline sync, analytics, and a DAG plan. With the gaps above closed, MudRock **becomes the WLIP runtime** rather than being replaced.

---

## 7) Immediate To‑Dos (Secure Collaboration by Design)

- **Tenancy & ACLs**: Implement workspace + per‑well ACL and extend RLS.  
- **Provenance**: Add PROV‑O entities/activities/agents + hash of inputs/outputs; UI lineage graph.  
- **Audit**: Append‑only audit log (write‑once); export to SIEM.  
- **Ingress**: Enforce VPN‑only for admin/studio; HTTPS everywhere; rotate all leaked sample creds.  
- **Secrets**: Migrate env secrets to Vault/SOPS; automate rotation.  
- **Backups/DR**: Daily encrypted backups + restore runbook + DR test cadence.  

---

## 8) Minimal Architecture Diagram (Text)

Client (Tauri/Svelte) → **Kong** → Supabase Auth / PostgREST / Storage / Realtime  
           ↘ **PowerSync** (offline sync) ↔ PostgreSQL/Mongo  
Rust DAG Engine → provenance emitter → **Audit + PROV‑O store** → WLIP Graph UI

---

## 9) Decision: Compatible, and recommended to integrate
Use MudRock as the **secure control plane** and **execution runtime** while WLIP standardizes the **ontology, lineage, and governance**. Together they deliver BIM‑like traceability without replacing existing domain tools.


---

## Appendix A — Plain‑Language Explanation of Core WLIP Concepts

To make the architecture easier to discuss across disciplines, here’s an analogy‑based explanation of key features.

### 🧱 1. Per‑Well ACLs — “Who can open which door?”
**ACLs** are *Access Control Lists.*  
Think of your data platform as a building, where every **well** is a separate **room** with its own lock.  
- Each user has a **keycard** that only opens the rooms (wells) they’re allowed to enter.  
- A petrophysicist might access one set of wells, a drilling engineer another.  
This keeps sensitive data separated, while still allowing collaboration in shared spaces.

### 🔒 2. Immutable Audit + PROV‑O Lineage — “The Black Box Recorder for Data”
An **immutable audit log** is like an airplane’s **black box**—it records every control input and event.  
You can’t erase history, only append new events.  
**PROV‑O lineage** adds the **map** that links those events:  
- **Entities** (datasets, models)  
- **Activities** (computations, imports, edits)  
- **Agents** (users, tools, scripts)  
Together, they show *exactly* how a dataset was created, enabling reproducibility and trust.

### 🧾 3. CDE‑Style Review/Approve States — “Document Control for Data”
Borrowed from the **Common Data Environment (CDE)** used in construction:  
data moves through clear lifecycle states — **Work‑in‑Progress → Shared → Published → Archived.**  
Each transition is gated by permissions and sometimes approvals.  
It ensures only validated designs or interpretations drive downstream decisions.

### ⚙️ 4. Adapter SDK Hooks — “Smart Plugs for Tools”
Just like a **smart plug** makes an old lamp controllable by your smart home app,  
**adapter SDKs** make legacy domain tools (Petrel, Techlog, WellPlan) part of the WLIP ecosystem.  
Every import/export automatically:
- Logs the event in the audit trail  
- Captures metadata (timestamp, user, checksum)  
- Registers lineage links in PROV‑O form  
This builds a seamless provenance record without user overhead.

### 🧩 Summary Table

| Concept | Analogy | Role in WLIP |
|----------|----------|--------------|
| **Per‑Well ACLs** | Locks and keycards for each well’s data room | Enforces secure access and partner boundaries |
| **Immutable Audit + PROV‑O Lineage** | Black box flight recorder + event map | Provides traceability and reproducibility |
| **CDE‑Style Review/Approve States** | Engineering document control lifecycle | Guarantees data quality and approval history |
| **Adapter SDK Hooks** | Smart plugs connecting existing tools | Automates provenance and context capture |


---

## Appendix B — Implementation Strategy: WLIP vs MudRock

### 1. Relationship Between WLIP and MudRock

**WLIP (Well Lifecycle Integration Platform)** and **MudRock** are distinct yet complementary.

- **WLIP** provides the *specification and integration layer* — the ontology, provenance model (PROV‑O), governance rules, and adapter SDKs that define how data and processes should interoperate securely and reproducibly across the well lifecycle.
- **MudRock** is an *implementation and runtime* — offering a desktop/web app experience, offline sync, authentication, deployment tooling, and an execution engine that brings WLIP’s ideas to life.

**Analogy:**  
Think of WLIP as the “IFC standard + CDE rules” in BIM, while MudRock is “Revit + BIM360” — the platform that implements and extends those rules.

---

### 2. Why They Should Be Separated

| Goal | Why Separation Helps |
|------|-----------------------|
| **Interoperability** | WLIP defines open standards so other vendors and tools can connect. |
| **Stability** | WLIP evolves slowly; MudRock iterates quickly. |
| **Governance** | Operators and regulators can adopt WLIP without being tied to a vendor. |
| **Innovation** | MudRock can innovate (UI, DAGs, offline sync) while staying compliant with WLIP. |

Recommended separation:
```
wlip-spec/          # Ontology, SHACL, RFCs
wlip-services/      # Provenance, audit, lifecycle, auth proxy
wlip-adapters-sdk/  # SDKs for Rust, Python
mudrock-app/        # UI, DAG runtime, collab engine
```

---

### 3. Implementation Strategy

#### Architecture Overview

```
+-------------------------------+         +------------------+
| MudRock (Desktop/Web UI)      |  gRPC   |  WLIP Services   |
| - Tauri + SvelteKit           +--------->  (stateless)     |
| - Visual DAG, operators       |         |  - Provenance    |
| - Team collab, offline sync   |         |  - Audit (WORM)  |
+-------------------------------+         |  - Lifecycle CDE |
          ^      |                        |  - Auth proxy    |
    Plugins/UDFs |                        +---------+--------+
          |      |                                  |
          |   File/Stream                         Storage
          |                                        (Postgres + RLS, Object Store,
+---------+---------+                              RDF/Graph for lineage)
|  Runtime Engines  |
| - Rust DAG Core   |
| - Python UDFs     |
| - WASM sandbox    |
+-------------------+
```

---

### 4. Language and Tech Choices

| Layer | Language | Rationale |
|--------|-----------|-----------|
| **DAG Core & Adapters** | **Rust** | Safe, performant, strong Arrow/Parquet/DataFusion ecosystem, cross-platform. |
| **UDFs / Data Science Layer** | **Python (sandboxed)** | Access to scientific libraries; executes under version control. |
| **APIs** | Rust (Axum/Tonic) + REST/GraphQL | Type-safe contracts and easy SDK generation. |
| **Lineage Store** | RDF/JSON-LD over Postgres or Neo4j | Supports PROV‑O lineage graph queries. |
| **UI** | Tauri + SvelteKit | Offline-first desktop; optional web mode for reviews. |

---

### 5. Key Modules

- **wlip-provenance-svc (Rust)** — Records PROV‑O entities, activities, and agents.
- **wlip-audit-svc (Rust)** — Append-only audit ledger (WORM), tamper-evident.
- **wlip-lifecycle-svc (Rust)** — Implements CDE-like states (WIP → Shared → Published → Archived).
- **wlip-auth-proxy (Rust)** — AuthN/Z proxy enforcing JWT and Row Level Security decisions.
- **wlip-adapters-sdk (Rust/Python)** — Simplifies registering ingest/export events with provenance.
- **mudrock-dag-engine (Rust)** — Executes pipelines; emits lineage data to wlip-provenance-svc.
- **mudrock-udf-runner (Python)** — Runs sandboxed UDFs; hashes inputs/outputs for traceability.

---

### 6. Data, Security & Confidentiality Model

- **Artifacts at rest**: Parquet, HDF5, Zarr (AES‑256 encrypted).
- **Metadata**: JSON‑LD + PROV‑O for full lineage.
- **AuthN/Z**: OIDC/SAML; short-lived JWTs; per-well ACLs + Row Level Security (RLS).
- **Network**: Single ingress (Kong), VPN or allow-list; TLS everywhere.
- **Audit**: Append-only ledger, exportable to SIEM.
- **Provenance**: PROV‑O model with hash-chained entries.
- **Tenancy**: Workspaces/schemas per project or operator; single-tenant or air-gapped deploys.
- **Secrets**: Vault/SOPS, rotation enforced.
- **Backups/DR**: Encrypted, daily snapshots with tested restores.

---

### 7. Developer Experience

- **APIs**: Axum (REST) + tonic (gRPC) with OpenAPI spec.
- **SDKs**:
  - `wlip-adapters-sdk-rs`: `register_ingest()`, `register_export()`, `register_transform()`.
  - `wlip-adapters-sdk-py`: same surface for Python tools.
- **Continuous Integration**: schema tests, SHACL validation, reproducibility checks.

---

### 8. Deployment Approach

| Pattern | When to Use | Highlights |
|----------|-------------|------------|
| **Single-tenant VPS (Hetzner/Dokploy)** | Pilots, small teams | Secure, fast, low-cost deployment. |
| **Multi-tenant VPS** | Central operator deployment | Per-tenant isolation, quotas, ACLs. |
| **On-prem/Air-gapped** | High security or regulatory sites | No external dependencies, offline-first sync. |
| **Hybrid** | Field teams offline → central hub | PowerSync handles sync and conflicts. |

---

### 9. Implementation Roadmap

1. **Phase 1 — Provenance Hooks**: Emit PROV‑O events from the Rust DAG engine.  
2. **Phase 2 — ACL & Lifecycle**: Add per-well RLS + CDE lifecycle UI.  
3. **Phase 3 — Publish Spec**: Release WLIP ontology + APIs.  
4. **Phase 4 — SDKs & Adapters**: Deliver Rust/Python SDKs; sample integrations.  
5. **Phase 5 — Security Hardening**: VPN-only ingress, Vault secrets, SIEM export.  
6. **Phase 6 — Pilot**: End-to-end traceability demo with real well data.

---

### 10. Summary

- **WLIP** defines *how things connect and are governed.*  
- **MudRock** provides *where people work and compute.*  
Together, they form a future-proof ecosystem for multidisciplinary well planning and execution — secure, auditable, and collaborative.

