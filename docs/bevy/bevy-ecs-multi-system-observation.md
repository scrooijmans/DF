# Bevy ECS: Multi-System Observation Without Tight Coupling

## Executive Summary

Bevy's Entity Component System (ECS) enables multiple systems and render pipelines to observe and react to the same underlying data through a sophisticated architecture that eliminates tight coupling and direct ownership. This is achieved through:

1. **Query-Based Access** - Systems declare what data they need, not what they own
2. **Archetype-Based Storage** - Column-oriented storage enables efficient parallel access
3. **Access Pattern Analysis** - The scheduler analyzes read/write patterns to enable safe parallelism
4. **Separate Render World** - Pipelined rendering uses a parallel render world synchronized via entity mapping
5. **No Direct Ownership** - Systems query data on-demand; no system owns component data

---

## 1. Core Architecture: Query-Based Observation

### 1.1 Systems Don't Own Data

In Bevy, systems never own component data. Instead, they **query** for what they need:

```rust
// System 1: Physics system reads Position and Velocity, writes Velocity
fn physics_system(mut query: Query<(&Position, &mut Velocity)>) {
    for (position, mut velocity) in &mut query {
        // Apply physics - modifies velocity
        velocity.y -= 9.8 * DELTA;
    }
}

// System 2: Rendering system reads Position (same data, different system)
fn render_system(query: Query<&Position>) {
    for position in &query {
        // Render at position - only reads, doesn't modify
    }
}
```

**Key Insight**: Both systems access the same `Position` component, but neither owns it. The ECS world owns all data, and systems request access through queries.

### 1.2 Query Types Determine Access Patterns

Bevy's query system uses Rust's type system to encode access patterns:

- **`&T`** - Immutable read access (can be shared)
- **`&mut T`** - Mutable write access (must be exclusive)
- **`Entity`** - Entity identifier (no data access)
- **`Option<&T>`** - Optional component (expands query space)

The scheduler analyzes these types to determine which systems can run in parallel:

> "Two systems cannot be executed in parallel if both access the same component type where at least one of the accesses is mutable. Because of this, it is recommended for queries to only fetch mutable access to components when necessary, since immutable access can be parallelized."

### 1.3 Multiple Read-Only Observers

Multiple systems can observe the same data simultaneously if they only need read access:

```rust
// System A: AI system observes positions
fn ai_system(query: Query<&Position>) {
    for position in &query {
        // AI logic based on positions
    }
}

// System B: Audio system observes positions (runs in parallel with AI)
fn audio_system(query: Query<&Position>) {
    for position in &query {
        // Spatial audio based on positions
    }
}

// System C: Debug system observes positions (also runs in parallel)
fn debug_system(query: Query<&Position>) {
    for position in &query {
        // Debug visualization
    }
}
```

All three systems can run **in parallel** because they only read `Position` - no conflicts exist.

---

## 2. Storage Architecture: Archetypes and Tables

### 2.1 Column-Oriented Storage

Bevy stores components in a **column-oriented structure-of-arrays** format:

- **Archetypes**: Group entities with the same component combination
- **Tables**: Store table components (most common) in columnar format
- **Sparse Sets**: Store sparse components (less common) for fast add/remove

This architecture enables:

1. **Cache-Friendly Iteration**: Systems iterate over columns, not individual entities
2. **Parallel Access**: Different systems can access different columns simultaneously
3. **Efficient Filtering**: Archetypes naturally filter entities by component combination

### 2.2 How Multiple Systems Access the Same Data

When multiple systems query the same components:

1. **Query Compilation**: Each system's query is compiled into an archetype filter
2. **Archetype Matching**: The scheduler matches queries to archetypes containing the required components
3. **Parallel Execution**: Systems with non-conflicting access patterns are scheduled in parallel
4. **Shared Data Access**: All systems read from the same underlying storage

The data is stored **once** in the archetype, but **multiple systems can observe it simultaneously** through their queries.

---

## 3. Scheduler: Conflict Detection and Parallel Execution

### 3.1 Access Pattern Analysis

Bevy's scheduler analyzes each system's access patterns at compile time:

```rust
// System declares its needs
fn movement_system(
    mut query: Query<(&mut Position, &Velocity)>,  // Writes Position, reads Velocity
    time: Res<Time>,                                 // Reads Time resource
) {
    // ...
}
```

The scheduler tracks:

- **Component Access**: Which components are read/written
- **Resource Access**: Which resources are read/written
- **World Access**: Whether the entire world is accessed

### 3.2 Conflict Detection

The scheduler builds a **dependency graph** that identifies conflicts:

- **No Conflict**: Both systems only read the same component → **Parallel execution**
- **Conflict**: One system writes, another reads/writes → **Sequential execution**
- **Ambiguity**: Conflicting access but no explicit ordering → **Warning/Error**

Example of conflict detection:

```rust
// System 1: Writes Position
fn physics_system(mut query: Query<&mut Position>) { }

// System 2: Reads Position (CONFLICT with System 1)
fn render_system(query: Query<&Position>) { }

// System 3: Reads Position (CONFLICT with System 1, but can run with System 2)
fn debug_system(query: Query<&Position>) { }
```

**Result**:

- `physics_system` and `render_system` cannot run in parallel
- `render_system` and `debug_system` **can** run in parallel (both read-only)
- The scheduler ensures `physics_system` runs before the read-only systems

### 3.3 Explicit Ordering

Systems can declare explicit ordering to resolve ambiguities:

```rust
app.add_systems(Update, (
    physics_system,
    render_system.after(physics_system),  // Explicit: render after physics
    debug_system.after(physics_system),   // Explicit: debug after physics
));
```

This ensures deterministic execution while still allowing `render_system` and `debug_system` to run in parallel with each other.

---

## 4. Pipelined Rendering: Separate Render World

### 4.1 Dual-World Architecture

Bevy's renderer operates in a **separate ECS World** that runs in parallel with the main world:

> "Bevy's renderer is architected independently from the main app. It operates in its own separate ECS World, so the renderer logic can run in parallel with the main world logic. This is called 'Pipelined Rendering'."

**Benefits**:

- Main world systems can run while rendering the previous frame
- Render world can process rendering data without blocking main world
- True parallelism between game logic and rendering

### 4.2 Entity Synchronization

The `SyncWorldPlugin` maintains entity-to-entity mapping between worlds:

1. **Entity Mapping**: Each synced main world entity gets a corresponding render world entity
2. **Component Tracking**: `RenderEntity` (main world) and `MainEntity` (render world) track the mapping
3. **Selective Sync**: Only entities with `SyncToRenderWorld` component are synchronized
4. **Automatic Cleanup**: When a main world entity is despawned, its render world counterpart is automatically despawned

```rust
// Main world entity
entity.add(SyncToRenderWorld);  // Marks entity for sync

// SyncWorldPlugin creates corresponding render world entity
// Render world entity gets MainEntity component pointing back
```

### 4.3 Data Extraction (Not Copying)

Crucially, **component data is not copied** during sync:

> "The sync step does not copy any of component data between worlds, since its often not necessary to transfer over all the components of a main world entity. The render world probably cares about a `Position` component, but not a `Velocity` component. The extraction happens in its own step, independently from, and after synchronization."

**Extraction Process**:

1. **Sync Phase**: Creates render world entities (structure only)
2. **Extract Phase**: Systems extract only the data they need from main world to render world
3. **Render Phase**: Render world systems process extracted data

This means:

- Main world systems can modify `Position` while render world reads the previous frame's position
- Render world doesn't need all components (e.g., doesn't need `Velocity`)
- Extraction is selective and efficient

### 4.4 Render Pipeline Observation

Render pipelines observe the same underlying data through:

1. **Extraction Systems**: Query main world, extract data to render world
2. **Render Systems**: Query render world, process extracted data
3. **No Direct Coupling**: Render systems don't directly access main world

Example extraction system:

```rust
fn extract_positions(
    mut commands: Commands,
    query: Query<(Entity, &Position), With<SyncToRenderWorld>>,
) {
    for (entity, position) in &query {
        // Extract position to render world
        commands.get_or_spawn(entity).insert(*position);
    }
}
```

Multiple render systems can then observe the extracted data:

```rust
// Render system 1: Mesh rendering
fn render_meshes(query: Query<(&Position, &Mesh)>) { }

// Render system 2: Sprite rendering
fn render_sprites(query: Query<(&Position, &Sprite)>) { }

// Render system 3: UI rendering
fn render_ui(query: Query<(&Position, &UiElement)>) { }
```

All three render systems observe the same `Position` data (extracted from main world), but they process different rendering aspects in parallel.

---

## 5. Avoiding Tight Coupling

### 5.1 No Direct References

Systems never hold direct references to entities or components:

- **Queries are ephemeral**: Created each frame, destroyed after use
- **No ownership**: Systems don't own the data they access
- **No lifetime coupling**: Systems don't need to know about other systems

### 5.2 Declarative Access

Systems **declare** what they need, not how to get it:

```rust
// System declares: "I need Position and Velocity"
fn movement_system(query: Query<(&Position, &Velocity)>) {
    // System doesn't know:
    // - Where Position is stored
    // - How many entities have Position
    // - What other systems are accessing Position
}
```

The scheduler handles:

- Finding matching entities
- Providing efficient access
- Ensuring safe parallel execution

### 5.3 Change Detection Without Coupling

Systems can detect changes without knowing what caused them:

```rust
fn render_system(query: Query<&Position, Changed<Position>>) {
    // Only processes entities where Position changed
    // Doesn't know what system changed it
}
```

The `Changed<T>` filter tracks modifications at the component level, not the system level. This enables:

- **Decoupled change detection**: Systems react to changes without knowing the source
- **Efficient updates**: Only changed components trigger system execution
- **Multiple observers**: Many systems can observe the same changes independently

### 5.4 Resource Sharing

Resources (global data) are shared similarly:

```rust
// System 1: Reads Time resource
fn physics_system(time: Res<Time>) { }

// System 2: Reads Time resource (can run in parallel)
fn audio_system(time: Res<Time>) { }

// System 3: Writes Time resource (conflicts with readers)
fn time_manager_system(mut time: ResMut<Time>) { }
```

Multiple systems can read a resource simultaneously, but writers get exclusive access. The scheduler ensures proper ordering.

---

## 6. Practical Example: Multiple Systems Observing Transform Data

Consider a game entity with a `Transform` component (position, rotation, scale):

```rust
#[derive(Component)]
struct Transform {
    translation: Vec3,
    rotation: Quat,
    scale: Vec3,
}
```

**Multiple systems observe this same data**:

```rust
// System 1: Physics - reads Transform, writes Velocity
fn physics_system(
    mut query: Query<(&Transform, &mut Velocity)>,
) {
    // Calculate forces based on position
}

// System 2: Rendering - reads Transform (runs in parallel with others)
fn render_system(query: Query<&Transform>) {
    // Render at transform position
}

// System 3: Audio - reads Transform (runs in parallel)
fn spatial_audio_system(query: Query<&Transform>) {
    // Calculate audio based on position
}

// System 4: Collision - reads Transform (runs in parallel)
fn collision_system(query: Query<&Transform>) {
    // Check collisions based on position
}

// System 5: AI - reads Transform (runs in parallel)
fn ai_system(query: Query<&Transform>) {
    // AI decisions based on position
}
```

**Execution Pattern**:

1. `physics_system` runs first (writes Velocity, reads Transform)
2. All other systems run **in parallel** (all read-only Transform access)
3. No system knows about the others
4. All observe the same underlying `Transform` data

**In the Render World**:

```rust
// Extract system: Copies Transform from main to render world
fn extract_transforms(
    mut commands: Commands,
    query: Query<(Entity, &Transform), With<SyncToRenderWorld>>,
) {
    for (entity, transform) in &query {
        commands.get_or_spawn(entity).insert(*transform);
    }
}

// Render systems observe extracted Transform
fn render_meshes(query: Query<&Transform>) { }
fn render_sprites(query: Query<&Transform>) { }
fn render_particles(query: Query<&Transform>) { }
```

All render systems observe the same extracted `Transform` data, processed in parallel.

---

## 7. Key Architectural Principles

### 7.1 Data Ownership Model

| Aspect          | Traditional OOP           | Bevy ECS                |
| --------------- | ------------------------- | ----------------------- |
| **Ownership**   | Objects own their data    | World owns all data     |
| **Access**      | Direct member access      | Query-based access      |
| **Coupling**    | Tight (object references) | Loose (queries)         |
| **Parallelism** | Manual synchronization    | Automatic via scheduler |

### 7.2 Observation Pattern

Bevy implements a **query-based observation pattern**:

1. **Systems declare interests**: Queries specify what components they need
2. **Scheduler matches**: Finds entities matching query criteria
3. **Parallel execution**: Non-conflicting systems run simultaneously
4. **No ownership**: Systems never own the data they observe

### 7.3 Separation of Concerns

- **Data Layer**: Components stored in archetypes (owned by World)
- **Logic Layer**: Systems query and process data (no ownership)
- **Rendering Layer**: Separate world with extracted data (no direct coupling)

Each layer operates independently, connected only through the query interface.

---

## 8. Comparison with Other Patterns

### 8.1 Observer Pattern

| Observer Pattern            | Bevy ECS                |
| --------------------------- | ----------------------- |
| Subjects notify observers   | Systems query on-demand |
| Explicit registration       | Implicit via queries    |
| Manual lifecycle management | Automatic via scheduler |
| Sequential updates          | Parallel execution      |

### 8.2 Event-Driven Architecture

| Event-Driven                  | Bevy ECS                              |
| ----------------------------- | ------------------------------------- |
| Events propagate changes      | Queries observe current state         |
| Decoupled producers/consumers | Decoupled systems via queries         |
| Event ordering matters        | Query access patterns determine order |
| Potential event loss          | Always sees current state             |

### 8.3 Data-Oriented Design

Bevy's ECS is a **data-oriented design** implementation:

- **Structure of Arrays**: Components stored in columns
- **Cache-Friendly**: Sequential memory access patterns
- **Parallel-Friendly**: Natural parallelization via queries
- **Scalable**: Handles thousands of entities efficiently

---

## 9. Benefits of This Architecture

### 9.1 Performance

1. **Parallel Execution**: Multiple systems run simultaneously
2. **Cache Efficiency**: Column-oriented storage improves cache locality
3. **Minimal Overhead**: Query compilation happens once, execution is fast
4. **Pipelined Rendering**: Main world and render world run in parallel

### 9.2 Maintainability

1. **Loose Coupling**: Systems don't know about each other
2. **Easy to Add Systems**: Just declare a query, scheduler handles the rest
3. **Clear Dependencies**: Access patterns are explicit in system signatures
4. **Testable**: Systems can be tested independently

### 9.3 Flexibility

1. **Dynamic Composition**: Entities can have any combination of components
2. **System Independence**: Systems can be added/removed without affecting others
3. **Selective Observation**: Systems only query what they need
4. **Extensible**: New components and systems integrate seamlessly

---

## 10. Conclusion

Bevy's ECS architecture enables multiple systems and render pipelines to observe and react to the same underlying data through:

1. **Query-Based Access**: Systems declare needs, don't own data
2. **Archetype Storage**: Efficient column-oriented storage enables parallel access
3. **Scheduler Analysis**: Automatic conflict detection enables safe parallelism
4. **Separate Render World**: Pipelined rendering with entity synchronization
5. **No Direct Ownership**: World owns data, systems observe via queries

This architecture provides:

- **Performance**: Parallel execution of non-conflicting systems
- **Decoupling**: Systems don't know about each other
- **Flexibility**: Easy to add/remove systems and components
- **Scalability**: Handles large numbers of entities efficiently

The key insight is that **systems observe data through queries, they never own it**. This fundamental design enables the loose coupling and parallel execution that makes Bevy's ECS so powerful.

---

## References

- Bevy ECS Documentation: https://docs.rs/bevy/latest/bevy/ecs/
- Bevy Render Documentation: https://docs.rs/bevy/latest/bevy/render/
- Pipelined Rendering: https://docs.rs/bevy/latest/bevy/render/pipelined_rendering/
- Sync World Plugin: https://docs.rs/bevy/latest/bevy/render/sync_world/
