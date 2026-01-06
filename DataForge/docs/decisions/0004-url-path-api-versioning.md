# Use URL Path Versioning for REST API

- **Status**: accepted
- **Date**: 2026-01-05
- **Decision-makers**: Engineering team
- **Consulted**: API design best practices research
- **Informed**: Stakeholders

## Context and Problem Statement

DataForge's sync server exposes REST endpoints that clients depend on. As the API evolves, we need a strategy to introduce changes without breaking existing clients. How should we version the API?

## Decision Drivers

- **Backward compatibility**: Existing clients must continue working
- **Discoverability**: Version should be obvious to developers
- **Cache efficiency**: Versioned requests should cache properly
- **Simplicity**: Easy to implement and maintain
- **Testing**: Easy to test different versions

## Considered Options

1. **URL Path Versioning** (`/api/v1/sync/push`)
2. **Header Versioning** (`Accept-Version: v1`)
3. **Query Parameter Versioning** (`/api/sync/push?version=1`)

## Decision Outcome

**Chosen option**: "URL Path Versioning", because it's the most widely adopted pattern (used by Stripe, GitHub, Twitter), naturally cache-friendly, and trivially implemented with Axum's nested router pattern.

### Consequences

**Positive:**
- Clear version visibility in URLs (easy debugging)
- Cache-friendly (different URLs = different cache entries)
- Native Axum support via `Router::nest()`
- Easy to maintain parallel versions during migration
- Industry standard pattern

**Negative:**
- URL changes between versions (clients must update base URL)
- Larger URL footprint
- Must maintain routing for each version

## Confirmation

- Sync server routes organized under `/api/v1/`
- Integration tests verify both versioned endpoints work
- Documentation reflects versioned URLs

## Pros and Cons of Options

### URL Path Versioning

Version in URL path: `/api/v1/sync/push`, `/api/v2/sync/push`

- **Good**: Most widely adopted (Stripe, GitHub, Twitter, Airbnb)
- **Good**: Cache-friendly (URL is cache key)
- **Good**: Easy to implement in Axum with `Router::nest()`
- **Good**: Version visible in logs, debugging, documentation
- **Good**: Easy to test in browser/curl
- **Bad**: URL changes require client updates
- **Bad**: May lead to code duplication across versions

### Header Versioning

Version in HTTP header: `Accept-Version: v1` or `X-API-Version: 1`

- **Good**: Clean URLs (no version in path)
- **Good**: Follows HTTP content negotiation patterns
- **Bad**: Version hidden from casual inspection
- **Bad**: Harder to test in browser
- **Bad**: Some proxies/caches don't handle custom headers well
- **Bad**: Requires middleware to extract and route

### Query Parameter Versioning

Version in query string: `/api/sync/push?version=1`

- **Good**: Easy to add to existing URLs
- **Good**: Visible in URL
- **Bad**: Clutters query string
- **Bad**: Less conventional
- **Bad**: Can conflict with other query parameters
- **Bad**: Semantically odd (version isn't a filter)

## More Information

### Implementation Pattern (Axum)

```rust
// Version-specific routers
let v1_routes = Router::new()
    .route("/sync/push", post(push_changes_v1))
    .route("/sync/pull", post(pull_changes_v1))
    .route("/blobs/urls", post(get_blob_urls_v1));

let v2_routes = Router::new()
    .route("/sync/push", post(push_changes_v2))
    .route("/sync/pull", post(pull_changes_v2));

// Compose with version prefixes
let app = Router::new()
    .nest("/api/v1", v1_routes)
    .nest("/api/v2", v2_routes)
    .route("/health", get(health_check));
```

### Versioning Policy

1. **Major versions** (v1 → v2): Breaking changes, new URL prefix
2. **Minor changes**: Additive, backward-compatible within same version
3. **Deprecation**: Announce 6 months before removing old version
4. **Support**: Maintain at least N-1 version alongside current

### Current Endpoints (v1)

| Endpoint | Method | Purpose |
|----------|--------|---------|
| `/api/v1/sync/push` | POST | Push local changes |
| `/api/v1/sync/pull` | POST | Pull remote changes |
| `/api/v1/blobs/urls` | POST | Get download URLs |
| `/api/v1/blobs/upload-urls` | POST | Get upload URLs |
| `/api/v1/blobs/:hash/uploaded` | POST | Confirm upload |

### References

- [REST API Versioning Best Practices](https://restfulapi.net/versioning/)
- [Axum Router::nest documentation](https://docs.rs/axum/latest/axum/routing/struct.Router.html)
- [Navigating API Versioning in Rust with Axum](https://leapcell.io/blog/navigating-api-versioning-in-rust-with-axum-and-actix-web)
