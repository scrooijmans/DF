# ParaView Multi-View Synchronization: Observer/Event Mechanisms and Pipeline-to-Render Propagation

## Executive Summary

ParaView synchronizes multiple views displaying the same pipeline data through a sophisticated **observer/event mechanism** built on VTK's command/observer pattern. When pipeline execution completes, events propagate through the Server Manager proxy system to **representations** (which map pipeline outputs to visualizations) and then to **render views** (which display the visualizations). This event-driven architecture ensures all views stay synchronized with pipeline state changes, providing consistent visualization across multiple render windows, charts, and other view types.

**Key Components**:
- **VTK Command/Observer Pattern**: Base event mechanism for notifications
- **Server Manager Events**: Proxy-level events for pipeline changes
- **Representations**: Bridge between pipeline outputs and visualizations
- **View Observers**: Views observe representations and pipeline sources
- **Render Synchronization**: Multiple views render the same data consistently

**Event Flow**:
```
Pipeline Execution Complete
    ↓
vtkSMProxy::UpdatePipeline() emits events
    ↓
Representations observe proxy events
    ↓
Representations update mappers/actors
    ↓
Views observe representation events
    ↓
Views trigger render updates
    ↓
All views synchronized
```

---

## 1. VTK Command/Observer Pattern

### Base Event Mechanism

VTK uses a **command/observer pattern** for event notifications:

```cpp
// VTK Command/Observer Pattern
class vtkObject : public vtkObjectBase
{
public:
    // Add observer for event
    unsigned long AddObserver(unsigned long event, 
                              vtkCommand* command,
                              float priority = 0.0f);
    
    // Remove observer
    void RemoveObserver(unsigned long tag);
    
    // Invoke event (notify all observers)
    void InvokeEvent(unsigned long event, void* callData = nullptr);
    
    // Invoke event with specific command
    void InvokeEvent(unsigned long event, void* callData, vtkCommand* command);
};
```

### Standard VTK Events

**Common Events**:

```cpp
// vtkCommand event IDs
vtkCommand::ModifiedEvent          // Object was modified
vtkCommand::StartEvent             // Operation started
vtkCommand::EndEvent                // Operation ended
vtkCommand::ProgressEvent          // Progress update
vtkCommand::ErrorEvent              // Error occurred
vtkCommand::WarningEvent            // Warning occurred
vtkCommand::UpdateEvent              // Update requested
vtkCommand::UpdateInformationEvent  // Information updated
vtkCommand::UpdateDataEvent          // Data updated
```

### Observer Implementation

**Creating an Observer**:

```cpp
// Custom observer class
class MyObserver : public vtkCommand
{
public:
    static MyObserver* New() { return new MyObserver; }
    
    void Execute(vtkObject* caller, 
                 unsigned long eventId,
                 void* callData) override
    {
        if (eventId == vtkCommand::ModifiedEvent)
        {
            // Handle modification
            vtkAlgorithm* algorithm = vtkAlgorithm::SafeDownCast(caller);
            if (algorithm)
            {
                // Algorithm was modified
                this->HandleAlgorithmModified(algorithm);
            }
        }
        else if (eventId == vtkCommand::UpdateDataEvent)
        {
            // Data was updated
            this->HandleDataUpdate(caller, callData);
        }
    }
    
private:
    void HandleAlgorithmModified(vtkAlgorithm* algorithm);
    void HandleDataUpdate(vtkObject* caller, void* callData);
};

// Usage
vtkAlgorithm* algorithm = ...;
MyObserver* observer = MyObserver::New();
algorithm->AddObserver(vtkCommand::ModifiedEvent, observer);
observer->Delete();  // Observer will be deleted when algorithm is deleted
```

---

## 2. Server Manager Event System

### Proxy Events

**Server Manager proxies emit events for pipeline changes**:

```cpp
// In vtkSMProxy
class vtkSMProxy : public vtkSMObject
{
public:
    // Update pipeline and emit events
    void UpdatePipeline(double time = -1.0)
    {
        // Update VTK objects
        this->UpdateVTKObjects();
        
        // Execute pipeline
        vtkAlgorithm* algorithm = vtkAlgorithm::SafeDownCast(
            this->GetClientSideObject());
        if (algorithm)
        {
            algorithm->Update();
        }
        
        // Emit update event
        this->InvokeEvent(vtkCommand::UpdateEvent);
        
        // Emit data update event
        this->InvokeEvent(vtkSMProxy::UpdateDataEvent);
    }
    
    // Property modified event
    void MarkModified(vtkSMProxy* modifiedProxy)
    {
        // Mark as modified
        this->Modified();
        
        // Emit modification event
        this->InvokeEvent(vtkCommand::ModifiedEvent, modifiedProxy);
        
        // Notify downstream proxies
        this->NotifyDownstreamProxies();
    }
};
```

### Server Manager Event Types

**ParaView-Specific Events**:

```cpp
// vtkSMProxy events
vtkSMProxy::UpdateDataEvent         // Pipeline data updated
vtkSMProxy::PropertyModifiedEvent   // Property changed
vtkSMProxy::ConnectionCreatedEvent  // Input connection created
vtkSMProxy::ConnectionRemovedEvent  // Input connection removed
vtkSMProxy::VisibilityChangedEvent  // Visibility changed
```

### Event Propagation

**Downstream Notification**:

```cpp
// In vtkSMProxy
void vtkSMProxy::NotifyDownstreamProxies()
{
    // Get all proxies that use this proxy as input
    vtkSMProxyIterator* iter = vtkSMProxyIterator::New();
    iter->SetModeToActiveOnly();
    
    for (iter->Begin(); !iter->IsAtEnd(); iter->Next())
    {
        vtkSMProxy* proxy = iter->GetProxy();
        
        // Check if this proxy is an input
        if (proxy->UsesProxyAsInput(this))
        {
            // Notify downstream proxy
            proxy->MarkModified(this);
        }
    }
    
    iter->Delete();
}
```

---

## 3. Representation System

### What are Representations?

**Representations** bridge pipeline outputs to visualizations:

```cpp
// Representation hierarchy
vtkSMRepresentationProxy (base)
    ├── vtkSMGeometryRepresentationProxy (3D geometry)
    ├── vtkSMChartRepresentationProxy (charts)
    ├── vtkSMTextSourceRepresentationProxy (text)
    └── vtkSMImageRepresentationProxy (images)
```

**Representation Responsibilities**:
1. **Map pipeline output to visualization**: Connect proxy output to mapper/actor
2. **Observe pipeline changes**: Listen for proxy update events
3. **Update visualization**: Update mappers/actors when data changes
4. **Manage visibility**: Control what is shown in views

### Representation Creation

**Creating a Representation**:

```cpp
// In ParaView UI (pqRepresentation)
pqRepresentation* representation = pqRepresentation::create(
    "GeometryRepresentation",  // Representation type
    sourceProxy,               // Pipeline source
    viewProxy                  // View to add to
);

// Representation internally:
// 1. Creates vtkSMRepresentationProxy
// 2. Sets input to source proxy
// 3. Adds to view
// 4. Observes source proxy for updates
```

### Representation Observer Setup

**Representations Observe Pipeline Sources**:

```cpp
// In vtkSMRepresentationProxy
void vtkSMRepresentationProxy::SetupPipeline(vtkSMProxy* input)
{
    // Set input
    this->SetInputProxy(input);
    
    // Observe input proxy for updates
    input->AddObserver(vtkCommand::UpdateEvent, 
                       this->GetObserver());
    input->AddObserver(vtkSMProxy::UpdateDataEvent,
                       this->GetObserver());
    input->AddObserver(vtkCommand::ModifiedEvent,
                       this->GetObserver());
}

// Observer callback
void vtkSMRepresentationProxy::OnInputUpdate(
    vtkObject* caller,
    unsigned long eventId,
    void* callData)
{
    if (eventId == vtkSMProxy::UpdateDataEvent)
    {
        // Input data was updated
        this->UpdateRepresentation();
    }
    else if (eventId == vtkCommand::ModifiedEvent)
    {
        // Input was modified
        this->MarkDirty();
    }
}
```

### Representation Update

**Updating Visualization**:

```cpp
// In vtkSMRepresentationProxy
void vtkSMRepresentationProxy::UpdateRepresentation()
{
    // Get input data
    vtkSMProxy* input = this->GetInputProxy();
    vtkDataObject* data = input->GetOutputDataObject(0);
    
    // Update mapper with new data
    vtkSMProperty* mapperProp = this->GetProperty("Mapper");
    vtkSMProxy* mapperProxy = vtkSMProxy::SafeDownCast(
        mapperProp->GetProxy());
    
    if (mapperProxy)
    {
        // Set input data
        vtkAlgorithm* mapper = vtkAlgorithm::SafeDownCast(
            mapperProxy->GetClientSideObject());
        if (mapper)
        {
            mapper->SetInputData(data);
            mapper->Update();
        }
    }
    
    // Emit representation update event
    this->InvokeEvent(vtkSMRepresentationProxy::RepresentationUpdatedEvent);
}
```

---

## 4. View Observer System

### View Architecture

**View Types**:

```cpp
// View hierarchy
vtkSMViewProxy (base)
    ├── vtkSMRenderViewProxy (3D rendering)
    ├── vtkSMChartViewProxy (charts)
    ├── vtkSMTableViewProxy (tables)
    └── vtkSMSpreadSheetViewProxy (spreadsheets)
```

**View Responsibilities**:
1. **Display representations**: Show visualizations
2. **Observe representations**: Listen for representation updates
3. **Coordinate rendering**: Manage render pipeline
4. **Synchronize with other views**: Keep multiple views in sync

### View Observer Setup

**Views Observe Representations**:

```cpp
// In vtkSMRenderViewProxy
void vtkSMRenderViewProxy::AddRepresentation(vtkSMRepresentationProxy* repr)
{
    // Add representation to view
    this->Representations.push_back(repr);
    
    // Observe representation for updates
    repr->AddObserver(vtkSMRepresentationProxy::RepresentationUpdatedEvent,
                      this->GetRepresentationObserver());
    repr->AddObserver(vtkCommand::ModifiedEvent,
                      this->GetRepresentationObserver());
    
    // Add to renderer
    this->AddRepresentationToRenderer(repr);
}

// Observer callback
void vtkSMRenderViewProxy::OnRepresentationUpdate(
    vtkObject* caller,
    unsigned long eventId,
    void* callData)
{
    if (eventId == vtkSMRepresentationProxy::RepresentationUpdatedEvent)
    {
        // Representation was updated
        this->MarkDirty();
        this->StillRender();  // Trigger render
    }
}
```

### Multiple Views Observing Same Representation

**Synchronization Pattern**:

```cpp
// Representation can be in multiple views
vtkSMRepresentationProxy* representation = ...;

// Add to first view
vtkSMRenderViewProxy* view1 = ...;
view1->AddRepresentation(representation);

// Add to second view
vtkSMRenderViewProxy* view2 = ...;
view2->AddRepresentation(representation);

// Both views observe the same representation
// When representation updates, both views are notified
```

**Event Propagation**:

```
Pipeline Update
    ↓
Representation observes proxy → RepresentationUpdatedEvent
    ↓
View 1 observes representation → MarkDirty() → Render()
    ↓
View 2 observes representation → MarkDirty() → Render()
    ↓
Both views synchronized
```

---

## 5. Complete Event Flow: Pipeline to Render

### Step-by-Step Flow

**Step 1: Pipeline Execution**:

```cpp
// User clicks Apply or pipeline updates
vtkSMProxy* sourceProxy = ...;
sourceProxy->UpdatePipeline();

// Internally:
void vtkSMProxy::UpdatePipeline()
{
    // Execute VTK pipeline
    vtkAlgorithm* algorithm = ...;
    algorithm->Update();
    
    // Emit events
    this->InvokeEvent(vtkCommand::UpdateEvent);
    this->InvokeEvent(vtkSMProxy::UpdateDataEvent);
}
```

**Step 2: Representation Observes Update**:

```cpp
// Representation observer callback
void vtkSMRepresentationProxy::OnInputUpdate(
    vtkObject* caller,
    unsigned long eventId,
    void* callData)
{
    if (eventId == vtkSMProxy::UpdateDataEvent)
    {
        // Get new data
        vtkSMProxy* input = vtkSMProxy::SafeDownCast(caller);
        vtkDataObject* newData = input->GetOutputDataObject(0);
        
        // Update mapper
        this->UpdateMapper(newData);
        
        // Emit representation update
        this->InvokeEvent(vtkSMRepresentationProxy::RepresentationUpdatedEvent);
    }
}
```

**Step 3: View Observes Representation**:

```cpp
// View observer callback
void vtkSMRenderViewProxy::OnRepresentationUpdate(
    vtkObject* caller,
    unsigned long eventId,
    void* callData)
{
    if (eventId == vtkSMRepresentationProxy::RepresentationUpdatedEvent)
    {
        // Mark view as needing update
        this->MarkDirty();
        
        // Trigger render
        this->StillRender();
    }
}
```

**Step 4: Render Update**:

```cpp
// In vtkSMRenderViewProxy
void vtkSMRenderViewProxy::StillRender()
{
    // Update all representations
    for (auto& repr : this->Representations)
    {
        repr->Update();
    }
    
    // Render
    vtkRenderWindow* renderWindow = this->GetRenderWindow();
    renderWindow->Render();
}
```

### Visual Flow Diagram

```
┌─────────────────────────────────────────────────────────────┐
│ Pipeline Execution                                           │
│  vtkSMProxy::UpdatePipeline()                                │
│    ├─> algorithm->Update()                                  │
│    └─> InvokeEvent(UpdateDataEvent)                         │
└────────────────────┬────────────────────────────────────────┘
                     │
                     ▼
┌─────────────────────────────────────────────────────────────┐
│ Representation Layer                                         │
│  vtkSMRepresentationProxy observes proxy                    │
│    ├─> OnInputUpdate() callback                             │
│    ├─> UpdateMapper(newData)                                │
│    └─> InvokeEvent(RepresentationUpdatedEvent)             │
└────────────────────┬────────────────────────────────────────┘
                     │
                     ▼
┌─────────────────────────────────────────────────────────────┐
│ View Layer (Multiple Views)                                  │
│  View 1: vtkSMRenderViewProxy                                │
│    ├─> Observes representation                               │
│    ├─> OnRepresentationUpdate() callback                     │
│    ├─> MarkDirty()                                           │
│    └─> StillRender()                                        │
│                                                              │
│  View 2: vtkSMRenderViewProxy                                │
│    ├─> Observes same representation                           │
│    ├─> OnRepresentationUpdate() callback                     │
│    ├─> MarkDirty()                                           │
│    └─> StillRender()                                        │
└─────────────────────────────────────────────────────────────┘
```

---

## 6. Synchronization Patterns

### Pattern 1: Single Source, Multiple Views

**Scenario**: One pipeline source displayed in multiple views.

```cpp
// Create source
vtkSMProxy* source = CreateProxy("sources", "Sphere");

// Create representation
vtkSMRepresentationProxy* repr = CreateRepresentation(source);

// Add to view 1
vtkSMRenderViewProxy* view1 = CreateView("RenderView");
view1->AddRepresentation(repr);

// Add to view 2
vtkSMRenderViewProxy* view2 = CreateView("RenderView");
view2->AddRepresentation(repr);

// Update source
source->UpdatePipeline();

// Event flow:
// 1. source emits UpdateDataEvent
// 2. repr observes → updates mapper → emits RepresentationUpdatedEvent
// 3. view1 observes → renders
// 4. view2 observes → renders
// Both views synchronized
```

### Pattern 2: Multiple Sources, Single View

**Scenario**: Multiple pipeline sources in one view.

```cpp
// Create sources
vtkSMProxy* source1 = CreateProxy("sources", "Sphere");
vtkSMProxy* source2 = CreateProxy("sources", "Cone");

// Create representations
vtkSMRepresentationProxy* repr1 = CreateRepresentation(source1);
vtkSMRepresentationProxy* repr2 = CreateRepresentation(source2);

// Add both to view
vtkSMRenderViewProxy* view = CreateView("RenderView");
view->AddRepresentation(repr1);
view->AddRepresentation(repr2);

// Update source1
source1->UpdatePipeline();

// Event flow:
// 1. source1 emits UpdateDataEvent
// 2. repr1 observes → updates → emits RepresentationUpdatedEvent
// 3. view observes repr1 → renders
// repr2 unchanged (no update)
```

### Pattern 3: Cascading Updates

**Scenario**: Filter chain with multiple views.

```cpp
// Pipeline: Source -> Filter -> (repr1, repr2)
vtkSMProxy* source = CreateProxy("sources", "Sphere");
vtkSMProxy* filter = CreateProxy("filters", "Shrink");
filter->SetInputConnection(source->GetOutputPort());

// Representations
vtkSMRepresentationProxy* repr1 = CreateRepresentation(filter);
vtkSMRepresentationProxy* repr2 = CreateRepresentation(filter);

// Views
vtkSMRenderViewProxy* view1 = CreateView("RenderView");
vtkSMRenderViewProxy* view2 = CreateView("RenderView");
view1->AddRepresentation(repr1);
view2->AddRepresentation(repr2);

// Update source
source->UpdatePipeline();

// Event flow:
// 1. source emits UpdateDataEvent
// 2. filter observes source → updates → emits UpdateDataEvent
// 3. repr1 observes filter → updates → emits RepresentationUpdatedEvent
// 4. repr2 observes filter → updates → emits RepresentationUpdatedEvent
// 5. view1 observes repr1 → renders
// 6. view2 observes repr2 → renders
// All synchronized
```

---

## 7. Observer Management

### Observer Lifecycle

**Adding Observers**:

```cpp
// In representation setup
void vtkSMRepresentationProxy::SetupPipeline(vtkSMProxy* input)
{
    // Create observer
    vtkSmartPointer<vtkCommand> observer = vtkSmartPointer<vtkCommand>::New();
    observer->SetClientData(this);
    observer->SetCallback(OnInputUpdate);
    
    // Add observer
    unsigned long tag = input->AddObserver(
        vtkSMProxy::UpdateDataEvent, observer);
    
    // Store tag for later removal
    this->ObserverTags[input] = tag;
}
```

**Removing Observers**:

```cpp
// In representation cleanup
void vtkSMRepresentationProxy::CleanupPipeline()
{
    // Remove all observers
    for (auto& pair : this->ObserverTags)
    {
        vtkSMProxy* proxy = pair.first;
        unsigned long tag = pair.second;
        proxy->RemoveObserver(tag);
    }
    this->ObserverTags.clear();
}
```

### Observer Priority

**Priority-Based Observation**:

```cpp
// High priority observer (executes first)
source->AddObserver(vtkCommand::UpdateEvent, 
                    highPriorityObserver, 
                    1.0f);  // High priority

// Normal priority observer
source->AddObserver(vtkCommand::UpdateEvent, 
                    normalObserver, 
                    0.0f);  // Normal priority

// Low priority observer (executes last)
source->AddObserver(vtkCommand::UpdateEvent, 
                    lowPriorityObserver, 
                    -1.0f);  // Low priority
```

---

## 8. Performance Optimizations

### Batch Updates

**Batching Multiple Updates**:

```cpp
// In view
void vtkSMRenderViewProxy::OnRepresentationUpdate(...)
{
    // Mark dirty but don't render immediately
    this->MarkDirty();
    
    // Schedule render for next event loop iteration
    if (!this->RenderScheduled)
    {
        QTimer::singleShot(0, this, SLOT(StillRender()));
        this->RenderScheduled = true;
    }
}

// Multiple representation updates in one frame
// Only one render call
```

### Selective Updates

**Updating Only Changed Representations**:

```cpp
// In view
void vtkSMRenderViewProxy::OnRepresentationUpdate(
    vtkObject* caller,
    unsigned long eventId,
    void* callData)
{
    vtkSMRepresentationProxy* repr = 
        vtkSMRepresentationProxy::SafeDownCast(caller);
    
    // Update only this representation
    repr->Update();
    
    // Mark view dirty
    this->MarkDirty();
    
    // Render (only changed representation updated)
    this->StillRender();
}
```

### Lazy Rendering

**Deferring Renders**:

```cpp
// In view
void vtkSMRenderViewProxy::MarkDirty()
{
    this->Dirty = true;
    
    // Don't render immediately if view is not visible
    if (!this->IsVisible())
    {
        return;  // Defer render until view becomes visible
    }
    
    // Render if visible
    this->StillRender();
}
```

---

## 9. Summary: Synchronization Architecture

### Key Components

1. **VTK Command/Observer Pattern**:
   - Base event mechanism
   - `AddObserver()`, `InvokeEvent()`, `RemoveObserver()`
   - Standard and custom events

2. **Server Manager Events**:
   - Proxy-level events (`UpdateDataEvent`, `ModifiedEvent`)
   - Downstream propagation
   - Property change notifications

3. **Representations**:
   - Bridge pipeline to visualization
   - Observe pipeline sources
   - Update mappers/actors
   - Emit representation events

4. **Views**:
   - Display representations
   - Observe representations
   - Coordinate rendering
   - Synchronize multiple views

### Event Flow Summary

```
Pipeline Execution
    ↓
Proxy Events (UpdateDataEvent)
    ↓
Representation Observers
    ↓
Representation Updates (RepresentationUpdatedEvent)
    ↓
View Observers (Multiple Views)
    ↓
View Renders
    ↓
All Views Synchronized
```

### Benefits

- **Automatic Synchronization**: Views stay in sync without manual coordination
- **Event-Driven**: Efficient, only updates what changed
- **Scalable**: Works with any number of views and representations
- **Flexible**: Supports different view types and representations
- **Performance**: Batch updates and lazy rendering optimize performance

This observer/event architecture ensures that **all views displaying the same pipeline data stay synchronized** automatically, providing a consistent visualization experience across multiple render windows, charts, and other view types.

