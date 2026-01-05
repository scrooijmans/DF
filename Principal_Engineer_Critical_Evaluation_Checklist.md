# Principal Engineer Critical Evaluation Checklist

This checklist is intended for **harsh, senior-level evaluation** of a software project against high engineering standards.
Failure in *BLOCKER sections* should block approval regardless of feature completeness.

This checklist **explicitly covers**:
- Architecture
- System Design
- Application Design
- API Design
- API Specification
- Implementation quality

---

## 1. Architecture & System Integrity (BLOCKER)

- [ ] System purpose and responsibilities clearly defined
- [ ] Clear ownership of data and authority boundaries
- [ ] Architectural principles explicitly documented
- [ ] Architecture aligned with product goals and constraints
- [ ] No hidden coupling between major components
- [ ] Failure and offline modes designed intentionally
- [ ] Architecture decisions recorded with rationale (ADR or equivalent)

**Red flags**
- Architecture inferred only from code
- Foundational decisions deferred
- Tight coupling justified as “temporary”

---

## 2. System Design & Component Interaction (BLOCKER)

- [ ] System decomposed into well-defined components/services
- [ ] Clear communication paths between components
- [ ] Data flow between components explicitly documented
- [ ] Failure propagation understood and controlled
- [ ] Retry and recovery behavior defined per interaction
- [ ] No ambiguous responsibility between components

**Red flags**
- Cyclic dependencies between services
- Components acting as both source and consumer of truth
- Undefined behavior under partial failure

---

## 3. Application Design & Internal Structure

- [ ] Internal layers/modules have clear responsibilities
- [ ] Domain logic isolated from UI, transport, and persistence
- [ ] Core abstractions stable and intentional
- [ ] State management explicit and understandable
- [ ] No leakage of infrastructure concerns into domain logic

**Red flags**
- Business logic spread across UI and handlers
- Excessive conditional logic instead of modeled behavior
- Tight coupling to frameworks

---

## 4. Domain Modeling & Conceptual Clarity (BLOCKER)

- [ ] Core domain concepts explicitly defined
- [ ] Domain language consistent across code, APIs, and docs
- [ ] Clear distinction between source data and derived data
- [ ] Domain invariants enforced in code
- [ ] No anemic domain model for non-trivial logic

**Red flags**
- Primitive obsession
- Domain rules duplicated across layers
- Inability to explain domain behavior clearly

---

## 5. API Design & Interaction Contracts (BLOCKER)

- [ ] APIs designed intentionally before implementation
- [ ] Clear ownership of each API boundary
- [ ] Operations are explicit and predictable
- [ ] Idempotency defined where retries are possible
- [ ] Error semantics consistent and documented
- [ ] Backward compatibility rules defined

**Red flags**
- Breaking API changes without versioning
- Clients rely on undocumented behavior
- Error handling via ad-hoc strings

---

## 6. API Specification & Schema Governance

- [ ] API specifications exist (OpenAPI / Protobuf / equivalent)
- [ ] Specs treated as source of truth
- [ ] Schemas versioned and evolution rules defined
- [ ] Required vs optional fields clearly defined
- [ ] Compatibility validated via tooling or CI

**Red flags**
- Specs lag behind implementation
- Schemas changed without compatibility review
- Consumers manually reverse-engineer APIs

---

## 7. Data Integrity, Storage & Lifecycle (BLOCKER)

- [ ] Data ownership clearly defined
- [ ] Immutable vs mutable data explicitly documented
- [ ] Data lifecycle documented end-to-end
- [ ] Derived data provenance tracked
- [ ] No silent mutation or loss of data

**Red flags**
- Data overwritten without audit
- No recovery or reconstruction strategy
- Implicit assumptions about persistence

---

## 8. Failure Handling, Resilience & Correctness

- [ ] Failure modes enumerated and validated
- [ ] Network, disk, and dependency failures handled safely
- [ ] Retries are bounded and safe
- [ ] Partial failures do not corrupt system state
- [ ] System behavior under stress predictable

**Red flags**
- Crashes as control flow
- Infinite or silent retries
- Inconsistent recovery behavior

---

## 9. Implementation Quality & Maintainability

- [ ] Code structure consistent and navigable
- [ ] Complexity is localized and justified
- [ ] Code prioritizes clarity over cleverness
- [ ] No unexplained “magic” behavior
- [ ] Dead code and technical debt actively managed

**Red flags**
- Fear of modifying core code
- Large functions with unclear responsibility
- Copy-paste driven development

---

## 10. Testing, Verification & Confidence

- [ ] Testing strategy defined per layer
- [ ] Critical paths covered by automated tests
- [ ] Tests validate behavior, not implementation details
- [ ] Data integrity and failure cases tested
- [ ] CI enforces quality gates

**Red flags**
- Tests exist but are ignored
- Manual testing required for confidence
- No tests for data correctness

---

## 11. Evolution, Change & Long-Term Viability (BLOCKER)

- [ ] System can evolve without large rewrites
- [ ] Core abstractions resilient to new requirements
- [ ] Migration paths defined for breaking changes
- [ ] Versioning and deprecation strategies documented
- [ ] System does not encode current assumptions permanently

**Red flags**
- Small changes require system-wide refactors
- No plan for breaking changes
- Tight coupling to current requirements

---

## Final Assessment Criteria

**Approve only if:**
- No BLOCKER section fails
- Red flags are acknowledged with mitigation plans
- Architecture, APIs, and data flow can be clearly explained

**Reject if:**
- Architecture is accidental
- Correctness relies on tribal knowledge
- Future change is visibly painful

---

*This checklist intentionally focuses on technical excellence and long-term system integrity.*
