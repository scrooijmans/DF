# DataForge Compute — MVP Specification
## A Local-First System of Computation Built on DataForge

---

## 1. Purpose of the Compute App (MVP)

**DataForge Compute** is a **local-first computation client** that:

- Reads authoritative data from **DataForge**
- Runs **explicit, intentional computations**
- Writes **derived results back to DataForge** as first-class, versioned artifacts
- Demonstrates how **systems of computation** can safely coexist with a **system of record**

This MVP is **not** a parametric modeling system, and **not** a replacement for Petrel, nTopology, or future tools.

> The MVP exists to **prove the boundary**, not to win on features.

---

## 2. What This MVP Is (and Is Not)

### ✅ This MVP *is*
- A **reference compute client**
- A **trust-building extension** to DataForge
- Proof that computation can be decoupled from storage
- Proof that derived data can be reproducible and auditable

### ❌ This MVP is *not*
- A parametric graph editor
- A real-time reactive system
- A visual modeling tool
- A general scripting environment
- A second “core product”

If something **silently recomputes**, it is out of scope.

---

## 3. Core Design Principles (Non-Negotiable)

1. **Explicit computation** — users intentionally trigger runs.
2. **Clear provenance** — inputs, parameters, and compute version recorded.
3. **Immutability of results** — append-only derived artifacts.
4. **Separation of concerns** — DataForge remains the system of record.
5. **Local-first** — fully usable offline.

---

## 4. MVP User Story

> “As an engineer, I want to load existing well log data from DataForge, run a simple computation on one or more curves, and store the result back in DataForge with full traceability.”

---

## 5. MVP Scope

### 5.1 Data Access
- Read-only access to DataForge data
- Select workspace, well, and curves
- Load Parquet data locally

### 5.2 Supported Computations (2–3 only)
- Moving average smoothing
- Linear scaling / normalization
- Simple depth resampling
- Null / gap flagging
- Basic statistics

All computations must be deterministic and easy to verify.

### 5.3 Parameters
- Small explicit parameter forms
- Parameters immutable once run

### 5.4 Execution Model
1. Select data
2. Configure parameters
3. Click **Run**
4. Execute locally
5. Write derived artifact to DataForge

If execution fails, nothing is written.

### 5.5 Output Artifacts
Each run produces:
- A derived curve or dataset
- Stored as Parquet
- Registered in DataForge with provenance

Derived data is clearly labeled as **Derived**.

---

## 6. Minimal Data Contract with DataForge

### Read Contract
- Query metadata via DataForge APIs
- Load Parquet via shared local storage
- No write access to raw entities

### Write Contract
- Single explicit API to register derived artifacts
- Append-only semantics
- Full provenance metadata

---

## 7. UI Expectations

- Minimal desktop UI or CLI
- No graph editor
- No live recompute
- Optional read-only visualization

---

## 8. Success Criteria

The MVP is successful if users:
- Clearly distinguish raw vs derived data
- Trust that original data is never modified
- Can explain how a result was produced
- Perceive increased value in DataForge

---

## 9. Explicit Non-Goals

- No DAG editor
- No scripting language
- No optimization loops
- No cloud execution
- No interactive parametric modeling

---

## 10. Strategic Role

This MVP exists to:
- Validate the DataForge / Compute boundary
- Build trust in local-first computation
- Demonstrate ecosystem extensibility

It should be presented as:
> “A reference compute client demonstrating safe computation on DataForge data.”

---

## 11. Recommended Timeline

- Built in **2–4 weeks**
- Implemented after initial DataForge validation
- Before committing to parametric modeling roadmaps

---

## 12. Guiding Principle

> **If DataForge is where truth lives,  
> DataForge Compute is where ideas are tested.**

---
