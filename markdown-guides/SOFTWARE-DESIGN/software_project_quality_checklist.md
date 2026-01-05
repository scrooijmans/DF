# Software Project Quality Assessment Checklist

This checklist is intended for **Principal / Staff-level engineers** assessing whether a software project is *built well*, not merely *working*.  
It emphasizes **change safety, operational excellence, and long-term maintainability**.

---

## 1. Change Safety & Maintainability

- [ ] Medium-sized changes can be made without widespread breakage
- [ ] Refactoring is routine and low-risk
- [ ] Code changes have predictable impact
- [ ] Tests fail for meaningful reasons (low flakiness)
- [ ] Test suite validates behavior, not implementation details
- [ ] Clear boundaries between units/modules/components

**Evidence to review**
- Recent PRs
- Test failure history
- Refactor PRs

---

## 2. Architecture & System Design

- [ ] Architecture aligns with actual problem constraints
- [ ] Clear ownership of business logic
- [ ] Dependencies flow in one direction
- [ ] No hidden global state or god objects
- [ ] Complexity is isolated and named
- [ ] Clear separation of concerns (UI / domain / infra)

**Evidence to review**
- Dependency graph
- High-level diagrams or ADRs
- Core domain modules

---

## 3. Operational Excellence

- [ ] Issues are diagnosable within minutes
- [ ] Logs, metrics, and traces are correlated
- [ ] Alerts are actionable and low-noise
- [ ] SLIs/SLOs exist and are monitored
- [ ] Runbooks exist for common incidents
- [ ] Postmortems lead to structural improvements

**Evidence to review**
- Dashboards
- Alert definitions
- Incident reports / postmortems

---

## 4. Delivery Pipeline & Dev Experience

- [ ] Project builds from scratch consistently
- [ ] CI is fast, deterministic, and reliable
- [ ] Main branch is usually green
- [ ] Deployments are automated
- [ ] Rollbacks are easy and tested
- [ ] Environments do not drift

**Evidence to review**
- CI logs and duration metrics
- Deployment scripts
- Infra-as-code repositories

---

## 5. Security & Data Integrity

- [ ] Authentication and authorization are centralized
- [ ] Least-privilege access is enforced
- [ ] Secrets are managed securely
- [ ] Dependencies are pinned and reviewed
- [ ] Input validation is consistent
- [ ] Data invariants are enforced in one place
- [ ] Migrations are reversible and tested

**Evidence to review**
- Auth middleware
- Dependency manifests
- Migration history

---

## 6. Performance & Scalability

- [ ] Performance budgets exist (latency, memory, cost)
- [ ] Hot paths are measured and profiled
- [ ] Load testing exists for critical flows
- [ ] Backpressure and rate limits are present
- [ ] System degrades gracefully under load
- [ ] Caching strategy is explicit and correct

**Evidence to review**
- Load test results
- Profiling data
- Resource utilization metrics

---

## 7. Codebase Consistency & Quality

- [ ] Naming and structure are consistent
- [ ] Formatting and linting are enforced automatically
- [ ] Common problems have standard solutions
- [ ] Error handling is consistent
- [ ] PRs are small and readable
- [ ] Code reviews focus on correctness and design

**Evidence to review**
- Lint rules
- Style guides
- Code review comments

---

## 8. Ownership & Team Health

- [ ] Clear ownership of components/services
- [ ] On-call responsibilities are well defined
- [ ] Knowledge is documented, not tribal
- [ ] New engineers onboard smoothly
- [ ] Incidents do not rely on heroics

**Evidence to review**
- Ownership docs
- On-call schedules
- Onboarding guides

---

## 9. Risk & Smell Detection

- [ ] No "do not touch" areas
- [ ] No critical manual steps in production workflows
- [ ] Feature flags are cleaned up
- [ ] Deprecated paths are removed
- [ ] Metrics are trusted and used
- [ ] Tech debt is tracked and paid down

---

## Overall Assessment

**Strengths**
- 

**Key Risks**
- 

**Recommended Actions**
- 

**Assessment Date:**  
**Assessed By:**  

---

### Rating (Optional)

- Architecture: ⭐⭐⭐⭐⭐
- Operability: ⭐⭐⭐⭐⭐
- Delivery: ⭐⭐⭐⭐⭐
- Code Quality: ⭐⭐⭐⭐⭐
- Overall Health: ⭐⭐⭐⭐⭐
