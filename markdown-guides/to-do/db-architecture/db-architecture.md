# Database Architecture: Project-Pipeline System

## Overview

This document outlines the database architecture for MudRock's collaborative data analysis platform, implementing a two-tier system: Projects → Pipelines.

## Architecture Goals

1. **Collaboration** - Multiple users can work on the same project
2. **Organization** - Direct pipeline management within projects
3. **Flexibility** - Pipelines contain nodes that may or may not be connected
4. **Security** - Row-level security for data isolation
5. **Scalability** - Support for large datasets and many users

## Database Schema

### Core Tables

#### 1. Projects Table

- **Purpose**: Top-level project containers
- **Key Features**:
  - Project metadata (name, description, settings)
  - Owner management
  - Timestamps for tracking
  - JSONB settings for extensibility

#### 2. Pipelines Table

- **Purpose**: DAG (Directed Acyclic Graph) definitions for data processing
- **Key Features**:
  - Pipeline metadata (name, description, version)
  - DAG definition storage (JSONB with nodes and edges)
  - Node configurations (ingestion, operator, output nodes)
  - Execution state tracking
  - Data lineage tracking

#### 3. Project Permissions Table

- **Purpose**: User access control
- **Key Features**:
  - Role-based permissions (owner, editor, viewer)
  - User-project associations
  - Granular access control

## Storage Strategy

### MinIO Bucket Organization

```
project-{project-id}/
├── shared/                           # Project-wide shared data
├── pipelines/                        # Pipeline execution artifacts and outputs
│   ├── {pipeline-id}/
│   │   ├── runs/                    # Execution runs
│   │   │   ├── {run-id}/
│   │   │   │   ├── metadata.json   # Run metadata
│   │   │   │   ├── lineage.json     # Lineage edges
│   │   │   │   └── outputs/        # Execution outputs
│   │   │   │       └── {dataframe-id}/
│   │   │   │           ├── metadata.json
│   │   │   │           ├── schema.json
│   │   │   │           └── data.parquet
│   │   └── cache/                   # Execution cache
│   │       └── {content_hash}/
│   │           ├── metadata.json
│   │           └── data.parquet
├── temp/                           # Temporary processing data
└── exports/                        # Project-level exports
```

### Data Flow

1. **User creates project** → Database entry + MinIO bucket
2. **User creates pipeline** → Database with nodes stored in database
3. **Pipeline execution** → Results stored in pipeline folder

## Security Model

### Row Level Security (RLS)

- **Projects**: Users can only access projects they own or have permissions for
- **Pipelines**: Access controlled by project permissions
- **Permissions**: Only project owners can manage permissions

### Permission Levels

- **Owner**: Full control (create, read, update, delete, manage permissions)
- **Editor**: Can create, read, update (no delete, no permission management)
- **Viewer**: Read-only access

## API Design

### Project Management

```typescript
interface Project {
  id: string;
  name: string;
  description: string;
  ownerId: string;
  createdAt: Date;
  updatedAt: Date;
  settings: ProjectSettings;
  pipelines: Pipeline[];
  permissions: ProjectPermission[];
}

interface ProjectSettings {
  storageQuota?: number;
  allowedFileTypes?: string[];
  autoSave?: boolean;
  [key: string]: any;
}
```

### Pipeline Management

```typescript
interface Pipeline {
  id: string;
  projectId: string;
  name: string;
  description?: string;
  version: string;
  dagDefinition: DagDefinition;
  nodes: DagNode[];
  edges: DagEdge[];
  createdAt: Date;
  updatedAt: Date;
  tags: string[];
}

interface DagNode {
  id: string;
  name: string;
  nodeType: 'ingestion' | 'operator' | 'output';
  config: NodeConfig;
  dependencies: string[]; // Array of node IDs
  metadata: NodeMetadata;
}

interface DagEdge {
  id: string;
  sourceNodeId: string;
  targetNodeId: string;
  edgeType: 'data_flow' | 'conditional' | 'error';
}
```

## Implementation Phases

### Phase 1: Database Setup ✅

- [x] Create database schema
- [x] Set up RLS policies
- [x] Create initialization scripts

### Phase 2: API Development

- [ ] Create Supabase API functions
- [ ] Implement CRUD operations
- [ ] Add permission management
- [ ] Create storage management functions

### Phase 3: Frontend Integration

- [ ] Update project management UI
- [ ] Create pipeline management
- [ ] Build DAG editor
- [ ] Add collaboration features

### Phase 4: Storage Integration

- [ ] Implement MinIO bucket management
- [ ] Create data upload/download APIs
- [ ] Add file management features
- [ ] Implement pipeline execution

### Phase 5: Advanced Features

- [ ] Real-time pipeline collaboration
- [ ] Version control for pipelines
- [ ] Data lineage tracking
- [ ] Performance optimization

## Migration Strategy

### Current State

- Using MinIO buckets directly as projects
- No collaboration features
- Limited metadata storage

### Target State

- Database-driven project management
- Full collaboration support
- Rich metadata and settings
- Organized storage structure

### Migration Approach

1. **Parallel Development**: Build new system alongside existing
2. **Gradual Migration**: Move users to new system incrementally
3. **Data Preservation**: Ensure no data loss during migration
4. **Feature Parity**: Maintain all existing functionality

## Benefits

### For Users

- **Collaboration**: Work together on projects
- **Organization**: Better project structure
- **Flexibility**: Multiple pipelines per project with flexible node connections
- **Security**: Data isolation and access control

### For Developers

- **Maintainability**: Clean database schema
- **Extensibility**: Easy to add new features
- **Performance**: Optimized queries and storage
- **Scalability**: Support for growth

## Future Enhancements

### Short Term

- Real-time pipeline collaboration
- Advanced permission management
- Data versioning
- Export/import functionality

### Long Term

- AI-powered data analysis
- Automated data pipelines
- Integration with external tools
- Advanced visualization features

## Monitoring & Analytics

### Key Metrics

- Project creation rate
- User engagement
- Storage usage
- Performance metrics

### Logging

- User actions
- Data access patterns
- Error tracking
- Performance monitoring

## Security Considerations

### Data Protection

- Encryption at rest
- Secure data transmission
- Access logging
- Regular security audits

### Compliance

- GDPR compliance
- Data retention policies
- User consent management
- Audit trails

---

_Last Updated: [Current Date]_
_Version: 2.0_
