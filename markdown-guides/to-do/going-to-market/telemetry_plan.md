# Telemetry Plan

## Purpose
Telemetry provides insights into how users interact with the product, what features are adopted, and where failures occur. Since our product is **self-hosted and on-premise**, telemetry must:
- Work offline and locally by default.
- Be optional for customers to share with us (export reports).
- Provide valuable metrics for measuring adoption, performance, and user experience.

---

## Goals of Telemetry

1. **Measure adoption**
   - How many users/projects/wells are being created?
   - How often are DAGs being built and executed?
   - Which features/operators are most commonly used?

2. **Measure success**
   - Execution success rate of DAGs (% of runs that complete without errors).
   - Time-to-first-result (how quickly a new user gets results).
   - Number of repeated active sessions per user.

3. **Measure failure**
   - Execution errors (operator not found, schema mismatch, I/O errors).
   - Abandoned DAGs (incomplete workflows never executed).
   - Long idle times after installation (potential churn).

4. **Measure user tendencies**
   - Which query types are most common (depth range vs. value cutoff vs. UDFs)?
   - Which data types are loaded most (logs, markers, polygons)?
   - Typical DAG size (average nodes per workflow).

---

## What to Track in Source Code

### Success Signals
- `users_created`
- `projects_created`
- `wells_loaded`
- `dags_executed`
- `execution_success_count`
- `time_to_first_result`

### Failure Signals
- `execution_failure_count` (with error categories)
- `abandoned_workflows`
- `retry_attempts`
- `idle_installations`

### User Tendencies
- `operator_usage_frequency`
- `data_type_loaded`
- `dag_size_distribution`
- `query_filter_types` (depth vs value vs UDF-heavy)

---

## Implementation Options

### 1. Local Metrics Logging
- Store usage data in a local SQLite/Parquet file inside the install directory.
- Customer can export/share logs manually for audits or support.

### 2. OpenTelemetry + Tracing
- Use **OpenTelemetry** to instrument function calls, DAG execution, and errors.
- Integrate with Rust `tracing` crate to collect structured events.
- Optionally allow exporting telemetry in JSON/Prometheus format.

### 3. In-App Dashboard
- Provide a local dashboard that visualizes usage metrics (like Grafana).
- Useful for customers to measure their own adoption and ROI.

---

## Privacy & Control
- **Default behavior:** metrics are local-only (no internet “phone-home”).
- **Optional sharing:** customers can opt-in to send anonymized metrics to us for product improvement.
- **Transparency:** document exactly what is collected (no sensitive data).

---

## Example Telemetry Flow

1. User executes a DAG.
2. Code logs:
   - DAG ID, size, operators used.
   - Execution time and outcome (success/failure).
   - Resource usage (CPU/memory, if allowed).
3. Telemetry writes to a local Parquet/JSON file.
4. Optional: Customer exports usage report for support.

---

## Telemetry Plan Summary
- Collect **adoption, success, failure, and usage tendency metrics**.
- Store telemetry **locally first** (self-hosted requirement).
- Use **OpenTelemetry + tracing** for structured instrumentation.
- Provide **optional sharing** and **in-app dashboards**.
- Keep telemetry transparent and lightweight to build customer trust.
