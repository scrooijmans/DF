# DataForge — Security & Data Residency

## Overview

DataForge is designed as an **offline-first, local-first geoscience platform**.  
It does **not require cloud connectivity**, external services, or third-party SaaS to function.

All data ownership, storage location, and network exposure are **explicitly controlled by the customer**.

---

## Core Security Principles

### 1. Customer-Controlled Data Location

- Data is stored **locally on user machines** and/or **on customer-managed servers**
- No mandatory cloud services
- No vendor-controlled storage

### 2. Offline-First by Design

- Full functionality without network access
- Networking is an optional capability, not a requirement
- Sync is **explicitly triggered**, never automatic or hidden

### 3. No Vendor “Phone Home”

- No telemetry
- No analytics
- No background connections
- No license validation over the internet (offline licenses supported)

### 4. Defense-in-Depth

- Encryption in transit (TLS)
- Optional encryption at rest
- Role-based access control
- Immutable content-addressed storage

---

## Data Residency Guarantees

DataForge **never moves data across borders unless the customer configures it to do so**.

### What This Means:

- If your server is deployed in Germany, data stays in Germany
- If your server is deployed on-premise, data never leaves your site
- If no server is deployed, data remains only on user machines

DataForge does **not**:

- Proxy data through vendor servers
- Replicate data to external regions
- Require global control planes

---

## Deployment Models

### 1. Fully Offline / Air-Gapped

- No internet access
- No external network access
- Server and clients operate on isolated LAN or standalone machines

Use cases:

- Classified environments
- National oil companies
- Government or defense-related projects

### 2. On-Premise (Private Network)

- Server hosted inside company network
- Access via LAN or VPN
- No public endpoints

### 3. Private Cloud (Customer-Managed)

- Server hosted in approved region (AWS/Azure/Hetzner/etc.)
- Region selection controlled by customer
- Vendor has no access to infrastructure or data

### 4. Hybrid

- On-premise primary system
- Encrypted cloud backup for disaster recovery only

---

## Client Security (Desktop Application)

- Runs as a native desktop application (Tauri)
- Local data stored in:
  - SQLite (metadata)
  - Parquet files (large datasets)
- Credentials stored in OS-provided secure storage
- Treated as an untrusted client by design

---

## Server Security

- Single-binary Rust server (optional Docker)
- SQLite or PostgreSQL (customer choice)
- Blob storage on filesystem or S3-compatible service
- No inbound access required beyond configured endpoints

---

## Sync & Collaboration Model

- Git-like, pull-based synchronization
- No real-time push channels
- No peer-to-peer networking
- All data transfers are explicit and auditable

---

## Compliance Alignment

DataForge architecture aligns with:

- Data sovereignty requirements
- ISO 27001 principles
- NIST Zero Trust concepts
- Enterprise security review expectations

---

## Summary

- You own the data
- You choose where it runs
- You control when it syncs
- DataForge works without internet
- Nothing leaves your environment unless you allow it
