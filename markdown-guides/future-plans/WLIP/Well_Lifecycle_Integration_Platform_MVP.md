# Well Lifecycle Integration Platform (WLIP) — MVP Vision

### Purpose

To create a **future-proof, collaborative, and auditable environment** that streamlines **end-to-end planning and execution** of wells by connecting existing domain applications and data systems — not replacing them.

Inspired by **BIM (construction)** and **PLM (aerospace/manufacturing)**, the platform provides an **integration and data governance layer**, rather than a new monolithic application.

---

## 1. Problem Statement

Well delivery is an inherently **multidisciplinary and iterative** process — combining data and decisions from:

- Subsurface (geology, geophysics, petrophysics)
- Well engineering (trajectory, casing, drilling design)
- Operations (execution logs, NPT, performance)
- HSE and regulatory compliance

Today:

- Each domain uses **separate tools and schemas** (Petrel, Techlog, WellPlan, WellView, etc.)
- Data exchange is **manual or file-based**
- There is **no single traceable thread** linking assumptions → design → execution → outcomes

---

## 2. Vision

A **federated integration platform** that connects existing domain systems through:

1. A **shared ontology and data backbone**
2. **Version-controlled data storage** (immutable, queryable)
3. **Collaborative workflows** with access control and audit trails
4. **DAG-based lineage tracking** for reproducibility and data governance
5. **Interoperable APIs and adapters** for existing domain tools

The platform’s role is _to orchestrate and contextualize_ — not to replace existing specialized software.

---

## 3. Core MVP Capabilities

### 3.1 Data Integration Layer

- Connects to existing repositories (e.g., OSDU, WITSML, internal DBs)
- Harmonizes entities: Well, Wellbore, Trajectory, GeoModel, Log, Plan, ExecutionReport
- Uses a **Well Lifecycle Ontology (WLO)** to define relationships and provenance
- Stores data in open, future-proof formats (Parquet, JSON-LD, HDF5)

### 3.2 Collaboration Layer

- Web-based workspace for multidisciplinary teams
- Users can visualize, comment, and approve across domains
- Each data object is versioned and timestamped (Git-like commit model)
- Discussion threads, annotations, and change history linked to data entities

### 3.3 Provenance & DAG Engine

- Every transformation, computation, or data import creates a **DAG node**
- Lineage is recorded using the **W3C PROV-O** model (Entity–Activity–Agent)
- Allows users to trace the origin of any value or model (e.g., “this well plan used GeoModel v3.1 and PetrophysicalModel v2.0”)
- Enables reproducibility and accountability

### 3.4 Application Integration

- Lightweight connectors (adapters) for existing tools:
  - Petrel / Techlog (subsurface)
  - WellPlan / Compass (drilling)
  - Excel / Python (engineering calculations)
- API-first design: each adapter registers data provenance in the WLIP backend
- Enables hybrid workflows where legacy systems feed into modern web apps

### 3.5 Governance & Audit

- Every data change, upload, or decision is logged immutably
- User and role-based permissions define who can approve, share, or modify data
- Configurable workflows (review → approve → publish) mimic ISO 19650’s CDE process states
- Optional blockchain-based signature for regulatory-grade data integrity

---

## 4. High-Level Architecture

```
+---------------------------------------------------------------+
|                  Well Lifecycle Integration Platform          |
|---------------------------------------------------------------|
|  1. Presentation Layer                                        |
|     - Collaborative web UI (SvelteKit / React)                |
|     - 3D/2D visualization of well trajectories, logs, maps    |
|     - Commenting, version diff, and timeline view             |
|---------------------------------------------------------------|
|  2. Integration Layer                                         |
|     - Connectors to OSDU, WITSML, Petrel, Techlog, Excel      |
|     - REST/GraphQL API Gateway                                |
|     - Message bus (Kafka / NATS) for data sync                |
|---------------------------------------------------------------|
|  3. Data & Provenance Layer                                   |
|     - Object store (Parquet, Zarr, HDF5)                      |
|     - Metadata in RDF/JSON-LD using Well Lifecycle Ontology   |
|     - DAG engine for provenance (PROV-O compliant)            |
|---------------------------------------------------------------|
|  4. Collaboration & Audit Layer                               |
|     - Auth (Supabase / Keycloak)                              |
|     - Role-based permissions, project spaces                  |
|     - Immutable activity log and version diff engine          |
|---------------------------------------------------------------|
|  5. Extensibility Layer                                       |
|     - SDK for adapters and domain plug-ins                    |
|     - Computation nodes (Python / Rust / C++ UDFs)            |
|---------------------------------------------------------------|
+---------------------------------------------------------------+
```

---

## 5. MVP Roadmap (Phased)

### Phase 1 — Foundation (0–6 months)

- Implement Well Lifecycle Ontology (WLO) with OSDU + WITSML mapping
- Minimal API to store and query Well, Wellbore, Log, Plan, ExecutionReport
- Basic web interface with authentication, version history, and commenting
- Integrate simple DAG lineage for imported datasets

### Phase 2 — Collaboration & Visualization (6–12 months)

- Add 3D trajectory and log viewers (WebGL + LightningChart / deck.gl)
- Implement approval workflows and annotation tools
- Create connectors for Excel, Techlog, and WellPlan

### Phase 3 — Advanced Provenance & Workflow (12–24 months)

- Full DAG execution tracing (inputs → outputs → approvals)
- Introduce data validation and SHACL rules
- Extend ontology to cover drilling and operational domains
- Deploy project templates (Exploration Well, Appraisal, Development)

---

## 6. Integration Philosophy

- **Open-first**: build on open standards (OSDU, Energistics, W3C, QUDT)
- **Composable**: every component (storage, auth, provenance) can be swapped
- **Transparent**: every computation and change is traceable
- **Collaborative**: enable multidisciplinary dialogue, not just data sharing
- **Future-proof**: data stored in open formats; metadata in RDF for long-term reuse

---

## 7. Current Gaps & Opportunities

Despite strong existing tools, **no integrated layer** connects the end-to-end lifecycle. Key gaps:

### 7.1 Lack of Cross-Discipline Traceability

- Petrel, Techlog, and WellPlan operate as silos; no shared metadata or lineage.
- Even within OSDU, provenance of transformations is weakly defined.
  **→ Opportunity:** Build the DAG + PROV-O-based provenance layer as the connective tissue.

### 7.2 Absence of Collaborative Governance

- Well planning is file-based, with data emailed or stored on shared drives.
- No structured review → approve → publish cycle across domains.
  **→ Opportunity:** Introduce lightweight governance modeled after ISO 19650 (CDE lifecycle).

### 7.3 Schema Fragmentation

- OSDU, WITSML, and Energistics standards overlap but are not unified.
- Well data can’t easily reference subsurface models or execution logs.
  **→ Opportunity:** Develop a “Well Lifecycle Ontology” (WLO) to harmonize these schemas.

### 7.4 Missing Mid-Tier Integration Tools

- There are connectors for file exchange (WITSML servers) but not **semantic APIs** for live collaboration.
  **→ Opportunity:** Create an adapter SDK that lets domain tools register data lineage automatically.

### 7.5 Poor Auditability and Version Control

- Existing tools store local project files without immutable version history.
- No global timeline of what changed, when, or why.
  **→ Opportunity:** Git-like semantic versioning for all well lifecycle data.

### 7.6 Human Collaboration Layer

- Engineers and geoscientists still communicate via email and PowerPoint.
  **→ Opportunity:** Build a shared collaborative UI with comments, annotations, and contextual discussions tied to data entities.

---

## 8. Why This Matters

By learning from BIM and PLM, the WLIP approach:

- Reduces rework and handover loss between domains
- Creates a single auditable thread from concept → execution → results
- Enhances trust and reproducibility
- Enables automation and analytics across lifecycle data
- Lays the foundation for a true **digital twin of the well** — with traceable context

---

## 9. Next Steps

1. Prototype ontology (Well, Wellbore, GeoModel, Log, Plan, ExecutionReport)
2. Build metadata + DAG proof of concept using JSON-LD + Neo4j
3. Design API and UI mockups for collaborative review and lineage visualization
4. Test with historical well data to evaluate usability and scalability

---

### 🧭 Summary

The goal is **not** to replace Petrel, Techlog, or WellPlan —  
but to **connect them in a governed, transparent, and reproducible way**.

Like BIM or PLM, this approach builds an **ecosystem**, not an application —  
enabling teams to work from a single digital backbone with full traceability.
