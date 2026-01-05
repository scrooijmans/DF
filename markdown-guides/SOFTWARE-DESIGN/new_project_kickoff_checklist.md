# New Project Kickoff Checklist (Principal Engineer)

This checklist is designed for **planning and bootstrapping a new software project**.  
It focuses on **irreversible decisions**, **change safety**, and **early leverage**, not polish or premature optimization.

Use this during:
- Project inception
- Architecture reviews
- Technical kickoffs
- Early design phases (weeks 0–4)

---

## Tier 1 — Must Be Correct from Day One

### 1. Problem Definition & Constraints

- [ ] Clear problem statement written in plain language
- [ ] Explicit non-goals documented
- [ ] Known constraints identified (scale, latency, cost, regulation, data size)
- [ ] Core business invariant(s) defined
- [ ] Success criteria measurable and observable

**Artifacts**
- Problem statement doc
- Invariants list
- Success metrics

---

### 2. Architecture & Boundaries

- [ ] Clear separation between domain logic and infrastructure
- [ ] Dependency direction is explicit and enforced
- [ ] Ownership of business rules is centralized
- [ ] No shared “common” dumping ground
- [ ] State ownership is clear
- [ ] External systems isolated behind interfaces

**Artifacts**
- Architecture diagram
- Module/package structure
- Boundary enforcement (linting, build rules)

---

### 3. Change Safety & Design for Refactoring

- [ ] System designed for localized change
- [ ] Core domain logic testable without infrastructure
- [ ] Tests encode behavior and invariants
- [ ] No flaky tests tolerated
- [ ] Refactoring expected and planned

**Artifacts**
- Domain test suite
- Example refactor PR
- Testing strategy doc

---

### 4. Delivery Pipeline & Reproducibility

- [ ] One command to build and test locally
- [ ] CI mirrors local build process
- [ ] Main branch always releasable
- [ ] Automated deployment path exists
- [ ] Rollback strategy defined

**Artifacts**
- CI configuration
- Build scripts
- Deployment docs

---

## Tier 2 — Establish Early, Iterate Safely

### 5. Observability & Operability

- [ ] Structured logging implemented
- [ ] Correlation IDs for requests/jobs
- [ ] Basic metrics for critical paths
- [ ] Errors surfaced clearly
- [ ] Initial dashboards created

**Artifacts**
- Logging format spec
- Metrics list
- Dashboard screenshots

---

### 6. Security & Data Integrity

- [ ] Authentication strategy defined
- [ ] Authorization model explicit
- [ ] Secrets managed outside code
- [ ] Input validation centralized
- [ ] Data invariants enforced at boundaries
- [ ] Migration strategy defined

**Artifacts**
- Auth flow diagram
- Secrets management setup
- Schema and migration plan

---

## Tier 3 — Intentionally Deferred (But Acknowledged)

### 7. Performance & Scalability

- [ ] Performance budgets written down
- [ ] Known hot paths identified
- [ ] Load test skeleton exists
- [ ] Backpressure strategy sketched

**Artifacts**
- Performance assumptions doc
- Load test placeholder

---

### 8. Codebase Consistency & Quality

- [ ] Formatting automated
- [ ] Linting focused on correctness
- [ ] Error-handling conventions documented
- [ ] Naming conventions agreed

**Artifacts**
- Lint config
- Style guide

---

## Team & Ownership

### 9. Ownership & Responsibilities

- [ ] Clear ownership of modules/services
- [ ] On-call expectations defined (even if future)
- [ ] Decision-making process explicit
- [ ] Architectural decisions recorded

**Artifacts**
- Ownership map
- ADR template

---

## Risk & Reality Checks

### 10. Explicit Tradeoffs

- [ ] What are we *not* solving yet?
- [ ] What assumptions are most likely wrong?
- [ ] What decisions would be hardest to undo?
- [ ] What would cause us to stop or pivot?

---

## Final Readiness Check

- [ ] New engineer can run the system in < 30 minutes
- [ ] System behavior understandable without tribal knowledge
- [ ] Failure modes are observable
- [ ] Change paths are clear

---

**Project Name:**  
**Assessment Date:**  
**Principal Engineer:**  

---

### Confidence Rating (Optional)

- Architecture Readiness: ⭐⭐⭐⭐⭐
- Change Safety: ⭐⭐⭐⭐⭐
- Delivery Readiness: ⭐⭐⭐⭐⭐
- Operational Readiness: ⭐⭐⭐⭐⭐
- Overall Confidence: ⭐⭐⭐⭐⭐
