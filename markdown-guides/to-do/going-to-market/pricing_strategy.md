# Pricing Strategy

## Overview
Our product sits in a specialized geoscience / petrophysics niche, where customers are used to paying high license fees for tools like Petrel, Kingdom, or Geolog. We want to:
- Lower the barrier to entry with affordable plans for individuals and small teams.
- Still provide enterprise-friendly pricing that scales fairly with larger organizations.
- Be transparent and predictable, avoiding FME-style backlash over pricing hikes.

---

## Pricing Tiers

### 1. Starter Plan (Individuals, Consultants, Academics)
- **Price:** $99 / month per user
- **Limits:** 
  - 1 project
  - Up to 5 wells
  - Limited compute resources (1 execution engine slot)
- **Target Audience:** Independent consultants, freelancers, academic users.
- **Goal:** Reduce friction for adoption, provide low-commitment entry.

### 2. Professional Plan (Small/Medium Companies)
- **Price:** $199–399 / month per user (or ~$2,000–$5,000 annually)
- **Limits:**
  - 10 projects, up to 50 wells
  - More compute capacity (2–4 execution slots)
  - Access to more advanced operators/UDFs
- **Target Audience:** Small geoscience consulting firms, startups.

### 3. Enterprise Plan
- **Price:** $50k–150k annually (flat-rate, unlimited users)
- **Benefits:**
  - Unlimited projects and wells
  - Unlimited execution capacity (configurable engines)
  - Priority support and training
  - Site license / floating users
- **Target Audience:** Oil & gas companies, large service providers.
- **Goal:** Predictable cost for enterprises, while still capturing large value.

---

## Lessons from FME Pricing

- **Adopt:** Modular operators, visual DAG builder, scalable engines (add capacity as needed).
- **Avoid:** Overly complex licensing and opaque cost structures.
- **Key Difference:** Provide lower entry tiers and transparent enterprise flat-rate options.

---

## Enforcement Model

- **License keys** encode user limits (number of users, projects, or execution engines).
- **Named users** preferred over concurrent user licensing for simplicity.
- For large enterprises, provide **site licenses** or **license server** for flexibility.

---

## Pricing Strategy Summary

1. Keep barrier-to-entry low with a $99/month Starter tier.
2. Anchor serious users at $2k–5k/year per seat.
3. Offer predictable enterprise flat-rate at $50k–150k/year.
4. Remain transparent and avoid sudden hikes (differentiate from FME).
5. Always tie pricing back to **value delivered** (time saved, workflows replaced).
