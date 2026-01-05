# MudRock Storage-to-Database Mapping System

## Overview

This document outlines a modular, extensible mapping system that bridges storage layer identifiers with PostgreSQL database entities, following the separation of concerns established in the storage layer architecture.

## 🎯 Design Principles

### 1. **Unified Interface Pattern**

- Single API for all mapping operations regardless of source/target
- Consistent error handling and caching strategies
- Type-safe mapping with compile-time validation

### 2. **Modular Architecture**

- Pluggable mappers for different entity types
- Centralized configuration and registry
- Easy extension for new mapping types

### 3. **Performance Optimization**

- Intelligent caching with TTL and invalidation
- Batch operations for multiple mappings
- Lazy loading and background refresh

### 4. **Storage Layer Integration**

- Seamless integration with existing storage layer components
- Leverages `project-data-layout` for path management
- Uses `object-store-adapter` for storage operations

## 🏗️ Proposed Architecture

### Core Mapping System

```
┌─────────────────────────────────────────────────────────────┐
│                    MUDROCK MAPPING SYSTEM                    │
├─────────────────────────────────────────────────────────────┤
│  ┌─────────────────┐  ┌─────────────────┐  ┌─────────────┐  │
│  │  mapping-core   │  │ mapping-cache   │  │mapping-registry││
│  │ (unified API)   │  │ (intelligent)   │  │ (pluggable)  │  │
│  └─────────────────┘  └─────────────────┘  └─────────────┘  │
├─────────────────────────────────────────────────────────────┤
│  ┌─────────────────────────────────────────────────────────┐ │
│  │              mapping-adapters                          │ │
│  │        (database & storage adapters)                   │ │
│  └─────────────────────────────────────────────────────────┘ │
├─────────────────────────────────────────────────────────────┤
│  ┌─────────────────┐  ┌─────────────────┐  ┌─────────────┐  │
│  │project-mapper   │  │curve-mapper     │  │well-mapper  │  │
│  │ (project names) │  │ (curve metadata)│  │ (well info) │  │
│  └─────────────────┘  └─────────────────┘  └─────────────┘  │
└─────────────────────────────────────────────────────────────┘
```

## 📦 Component Design

### 1. **`mapping-core` - Unified Mapping Interface**

**Purpose**: Central mapping API with pluggable mappers and intelligent caching

**Key Features**:

- Unified `MappingService` trait for all mapping operations
- Generic `Mapper<T, U>` trait for specific entity mappings
- Centralized configuration and error handling
- Batch operations and async support

```rust
// Core mapping traits
pub trait MappingService {
    async fn map_batch<T, U>(
        &self,
        mapper_type: &str,
        inputs: Vec<T>,
    ) -> Result<Vec<MappingResult<U>>, MappingError>;

    async fn map_single<T, U>(
        &self,
        mapper_type: &str,
        input: T,
    ) -> Result<MappingResult<U>, MappingError>;

    async fn invalidate_cache(&self, mapper_type: &str, keys: Vec<String>);
    async fn refresh_cache(&self, mapper_type: &str);
}

pub trait Mapper<T, U> {
    type Error;

    async fn map(&self, input: T) -> Result<U, Self::Error>;
    async fn map_batch(&self, inputs: Vec<T>) -> Result<Vec<U>, Self::Error>;
    async fn get_cache_key(&self, input: &T) -> String;
    async fn validate_input(&self, input: &T) -> Result<(), ValidationError>;
}

// Generic mapping result
pub struct MappingResult<T> {
    pub original: String,
    pub mapped: T,
    pub metadata: MappingMetadata,
    pub cached: bool,
}

pub struct MappingMetadata {
    pub mapper_type: String,
    pub timestamp: DateTime<Utc>,
    pub source: String,
    pub confidence: f32,
}
```

### 2. **`mapping-cache` - Intelligent Caching System**

**Purpose**: Multi-tier caching with TTL, invalidation, and background refresh

**Key Features**:

- Memory cache (L1) for hot data
- Persistent cache (L2) for session data
- Background refresh for stale data
- Cache warming and preloading

```rust
pub struct MappingCache {
    memory_cache: Arc<RwLock<HashMap<String, CachedValue>>>,
    persistent_cache: Option<Box<dyn PersistentCache>>,
    ttl_config: TtlConfig,
    refresh_strategy: RefreshStrategy,
}

pub struct CachedValue<T> {
    pub value: T,
    pub created_at: DateTime<Utc>,
    pub expires_at: DateTime<Utc>,
    pub access_count: u64,
    pub last_accessed: DateTime<Utc>,
}

pub enum RefreshStrategy {
    Lazy,           // Refresh on access
    Proactive,      // Background refresh
    Hybrid,         // Hot data proactive, cold data lazy
}
```

### 3. **`mapping-adapters` - Database & Storage Adapters**

**Purpose**: Unified adapters for different data sources (PostgreSQL, MinIO, etc.)

**Key Features**:

- Generic `DataSourceAdapter` trait
- PostgreSQL adapter with connection pooling
- MinIO/S3 adapter for storage metadata
- Configurable query builders

```rust
pub trait DataSourceAdapter {
    type Connection;
    type QueryBuilder;

    async fn connect(&self) -> Result<Self::Connection, AdapterError>;
    async fn execute_query<T>(
        &self,
        query: &str,
        params: &[&dyn ToSql],
    ) -> Result<Vec<T>, AdapterError>;

    async fn execute_batch_query<T>(
        &self,
        queries: Vec<BatchQuery>,
    ) -> Result<Vec<Vec<T>>, AdapterError>;
}

pub struct PostgresAdapter {
    pool: Arc<Pool<Postgres>>,
    config: PostgresConfig,
}

pub struct MinIOAdapter {
    client: Arc<MinioClient>,
    config: MinIOConfig,
}
```

### 4. **Specific Mappers**

#### **`project-mapper` - Project Name Mapping**

**Purpose**: Map storage bucket IDs to project names and metadata

```rust
pub struct ProjectMapper {
    adapter: Arc<dyn DataSourceAdapter>,
    cache: Arc<MappingCache>,
    config: ProjectMapperConfig,
}

impl Mapper<StorageBucketId, ProjectInfo> for ProjectMapper {
    async fn map(&self, bucket_id: StorageBucketId) -> Result<ProjectInfo, ProjectMapperError> {
        // Extract project ID from bucket name
        let project_id = self.extract_project_id(&bucket_id)?;

        // Check cache first
        if let Some(cached) = self.cache.get(&project_id).await {
            return Ok(cached);
        }

        // Query database
        let project = self.adapter
            .execute_query::<ProjectInfo>(
                "SELECT id, name, description, created_at, updated_at FROM projects WHERE id = $1",
                &[&project_id]
            )
            .await?
            .into_iter()
            .next()
            .ok_or(ProjectMapperError::ProjectNotFound(project_id))?;

        // Cache result
        self.cache.set(&project_id, &project, Duration::hours(1)).await;

        Ok(project)
    }
}
```

#### **`curve-mapper` - Curve Metadata Mapping**

**Purpose**: Map curve identifiers to metadata from `curve_metadata_view`

```rust
pub struct CurveMapper {
    adapter: Arc<dyn DataSourceAdapter>,
    cache: Arc<MappingCache>,
    config: CurveMapperConfig,
}

impl Mapper<CurveId, CurveMetadata> for CurveMapper {
    async fn map(&self, curve_id: CurveId) -> Result<CurveMetadata, CurveMapperError> {
        // Similar pattern to ProjectMapper but for curve metadata
        let curve_metadata = self.adapter
            .execute_query::<CurveMetadata>(
                "SELECT * FROM curve_metadata_view WHERE curve_id = $1",
                &[&curve_id]
            )
            .await?
            .into_iter()
            .next()
            .ok_or(CurveMapperError::CurveNotFound(curve_id))?;

        Ok(curve_metadata)
    }
}
```

#### **`well-mapper` - Well Information Mapping**

**Purpose**: Map well identifiers to comprehensive well information

```rust
pub struct WellMapper {
    adapter: Arc<dyn DataSourceAdapter>,
    cache: Arc<MappingCache>,
    config: WellMapperConfig,
}

impl Mapper<WellId, WellInfo> for WellMapper {
    async fn map(&self, well_id: WellId) -> Result<WellInfo, WellMapperError> {
        // Map well ID to comprehensive well information
        // Including metadata, location, status, etc.
    }
}
```

## 🔧 Configuration System

### **Unified Configuration**

```rust
#[derive(Debug, Serialize, Deserialize)]
pub struct MappingConfig {
    pub cache: CacheConfig,
    pub adapters: AdapterConfig,
    pub mappers: MapperConfig,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CacheConfig {
    pub memory_size: usize,
    pub ttl_default: Duration,
    pub refresh_strategy: RefreshStrategy,
    pub persistent_cache: Option<PersistentCacheConfig>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AdapterConfig {
    pub postgres: PostgresConfig,
    pub minio: MinIOConfig,
    pub connection_pool_size: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MapperConfig {
    pub project: ProjectMapperConfig,
    pub curve: CurveMapperConfig,
    pub well: WellMapperConfig,
}
```

## 🚀 Usage Examples

### **1. Project Name Mapping (Current Use Case)**

```rust
// Initialize mapping service
let mapping_service = MappingService::new(config).await?;

// Map single project
let project_info = mapping_service
    .map_single::<StorageBucketId, ProjectInfo>(
        "project",
        "project-3bd95caa-6360-4d91-8d40-5ef6c659c93a".into()
    )
    .await?;

// Map multiple projects
let bucket_ids = vec![
    "project-3bd95caa-6360-4d91-8d40-5ef6c659c93a".into(),
    "project-8fac629b-7485-44fe-a2b0-2c3eb22c5bf6".into(),
];
let project_infos = mapping_service
    .map_batch::<StorageBucketId, ProjectInfo>("project", bucket_ids)
    .await?;
```

### **2. Curve Metadata Mapping**

```rust
// Map curve IDs to metadata
let curve_ids = vec!["GR", "RHOB", "NPHI", "DT"];
let curve_metadata = mapping_service
    .map_batch::<CurveId, CurveMetadata>("curve", curve_ids)
    .await?;

// Use in data processing
for curve in curve_metadata {
    println!("Curve: {} - Unit: {} - Description: {}",
        curve.name, curve.unit, curve.description);
}
```

### **3. Well Information Mapping**

```rust
// Map well IDs to comprehensive information
let well_ids = vec!["F02-1", "F03-4", "F06-1"];
let well_infos = mapping_service
    .map_batch::<WellId, WellInfo>("well", well_ids)
    .await?;

// Use for spatial analysis
for well in well_infos {
    println!("Well: {} - Location: ({}, {}) - Status: {}",
        well.name, well.x, well.y, well.status);
}
```

## 🔄 Integration with Storage Layer

### **Enhanced Storage Manager Integration**

```rust
// Extend existing storage manager with mapping capabilities
pub struct EnhancedStorageManager {
    storage_manager: MudRockStorageManager,
    mapping_service: Arc<MappingService>,
}

impl EnhancedStorageManager {
    pub async fn get_bucket_with_mapped_name(
        &self,
        bucket_name: &str,
    ) -> Result<StorageBucketWithMappedName, StorageError> {
        // Use mapping service to get project info
        let project_info = self.mapping_service
            .map_single::<StorageBucketId, ProjectInfo>("project", bucket_name.into())
            .await?;

        // Combine with storage metadata
        Ok(StorageBucketWithMappedName {
            bucket_name: bucket_name.to_string(),
            project_id: project_info.id,
            project_name: project_info.name,
            project_description: project_info.description,
            file_count: self.get_file_count(bucket_name).await?,
        })
    }
}
```

### **Frontend Integration**

```typescript
// Enhanced frontend service
export class EnhancedMappingService {
  private mappingService: MappingService;

  async mapStorageBucketsToProjects(
    bucketNames: string[],
  ): Promise<StorageBucketWithMappedName[]> {
    // Use unified mapping API
    const results = await this.mappingService.mapBatch("project", bucketNames);
    return results.map((result) => ({
      bucket_name: result.original,
      project_name: result.mapped.name,
      project_id: result.mapped.id,
      project_description: result.mapped.description,
      cached: result.cached,
    }));
  }

  async mapCurveIdsToMetadata(curveIds: string[]): Promise<CurveMetadata[]> {
    return await this.mappingService.mapBatch("curve", curveIds);
  }

  async mapWellIdsToInfo(wellIds: string[]): Promise<WellInfo[]> {
    return await this.mappingService.mapBatch("well", wellIds);
  }
}
```

## 📊 Performance Optimizations

### **1. Intelligent Caching Strategy**

```rust
pub struct CacheStrategy {
    // Hot data: Keep in memory, refresh proactively
    hot_data_ttl: Duration,
    hot_data_refresh_threshold: f32,

    // Warm data: Keep in memory, refresh on access
    warm_data_ttl: Duration,

    // Cold data: Persist to disk, refresh on access
    cold_data_ttl: Duration,
    cold_data_persistent: bool,
}

impl CacheStrategy {
    pub fn determine_strategy(&self, access_pattern: &AccessPattern) -> RefreshStrategy {
        match access_pattern {
            AccessPattern::Hot => RefreshStrategy::Proactive,
            AccessPattern::Warm => RefreshStrategy::Hybrid,
            AccessPattern::Cold => RefreshStrategy::Lazy,
        }
    }
}
```

### **2. Batch Operations**

```rust
// Optimized batch mapping with connection pooling
impl MappingService {
    pub async fn map_batch_optimized<T, U>(
        &self,
        mapper_type: &str,
        inputs: Vec<T>,
    ) -> Result<Vec<MappingResult<U>>, MappingError> {
        // Group by cache status
        let (cached, uncached) = self.group_by_cache_status(inputs).await;

        // Return cached results immediately
        let mut results = cached;

        // Batch query for uncached items
        if !uncached.is_empty() {
            let batch_results = self.batch_query_uncached(mapper_type, uncached).await?;
            results.extend(batch_results);
        }

        Ok(results)
    }
}
```

### **3. Background Refresh**

```rust
// Background cache refresh for hot data
pub struct BackgroundRefreshService {
    mapping_service: Arc<MappingService>,
    refresh_interval: Duration,
    hot_data_threshold: f32,
}

impl BackgroundRefreshService {
    pub async fn start(&self) {
        let mut interval = tokio::time::interval(self.refresh_interval);

        loop {
            interval.tick().await;

            // Refresh hot data proactively
            let hot_keys = self.get_hot_data_keys().await;
            for key in hot_keys {
                self.mapping_service.refresh_cache("project", vec![key]).await;
            }
        }
    }
}
```

## 🔧 Implementation Plan

### **Phase 1: Core Infrastructure (Week 1-2)**

1. **Create `mapping-core` crate**
   - Define core traits and interfaces
   - Implement basic `MappingService`
   - Add error handling and validation

2. **Create `mapping-cache` crate**
   - Implement memory cache with TTL
   - Add cache invalidation and refresh logic
   - Create cache configuration system

3. **Create `mapping-adapters` crate**
   - Implement `PostgresAdapter`
   - Implement `MinIOAdapter`
   - Add connection pooling and query optimization

### **Phase 2: Specific Mappers (Week 3-4)**

1. **Migrate `project-mapper`**
   - Move existing project mapping logic to new system
   - Add enhanced caching and error handling
   - Maintain backward compatibility

2. **Create `curve-mapper`**
   - Implement curve metadata mapping
   - Add curve-specific validation and caching
   - Integrate with `curve_metadata_view`

3. **Create `well-mapper`**
   - Implement well information mapping
   - Add spatial data support
   - Integrate with well-related tables

### **Phase 3: Integration & Optimization (Week 5-6)**

1. **Update Storage Layer Integration**
   - Enhance `MudRockStorageManager` with mapping capabilities
   - Update `project-data-layout` for mapping integration
   - Add mapping configuration to storage layer

2. **Frontend Integration**
   - Create unified frontend mapping service
   - Update existing components to use new mapping system
   - Add mapping configuration UI

3. **Performance Optimization**
   - Implement intelligent caching strategies
   - Add background refresh services
   - Optimize batch operations

### **Phase 4: Advanced Features (Week 7-8)**

1. **Advanced Caching**
   - Add persistent cache support
   - Implement cache warming strategies
   - Add cache analytics and monitoring

2. **Extensibility Features**
   - Add plugin system for custom mappers
   - Create mapper configuration UI
   - Add mapping validation and testing tools

3. **Documentation & Testing**
   - Complete API documentation
   - Add comprehensive test suite
   - Create migration guides

## 🎯 Benefits

### **1. Unified API**

- Single interface for all mapping operations
- Consistent error handling and caching
- Easy to use and maintain

### **2. Performance**

- Intelligent caching with multiple tiers
- Batch operations for efficiency
- Background refresh for hot data

### **3. Extensibility**

- Easy to add new mappers
- Pluggable architecture
- Configuration-driven behavior

### **4. Integration**

- Seamless integration with storage layer
- Maintains existing functionality
- Future-proof architecture

### **5. Maintainability**

- Clear separation of concerns
- Centralized configuration
- Comprehensive error handling

## 🔮 Future Extensions

### **1. Advanced Mapping Types**

- Spatial mapping (coordinates to regions)
- Temporal mapping (timestamps to periods)
- Hierarchical mapping (parent-child relationships)

### **2. Machine Learning Integration**

- Predictive caching based on usage patterns
- Intelligent refresh strategies
- Anomaly detection for mapping errors

### **3. Real-time Updates**

- WebSocket integration for live updates
- Event-driven cache invalidation
- Real-time mapping synchronization

### **4. Multi-tenant Support**

- Tenant-specific mapping configurations
- Isolated caching strategies
- Cross-tenant mapping capabilities

This modular mapping system will provide a solid foundation for all storage-to-database mapping needs while maintaining the clean architecture and separation of concerns established in your storage layer.
