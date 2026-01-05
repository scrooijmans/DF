# ParaView Pipeline View Updates

This document explains in detail how ParaView updates views when pipeline state changes: execution model (`UpdatePipeline`, time steps), render view refresh, data information updates, and the observer/event mechanism tying pipeline execution to UI refresh.

## Overview

ParaView uses a **demand-driven execution model** with **observer/event mechanisms** to synchronize views with pipeline state:

1. **Demand-Driven Execution**: Pipeline executes only when needed (rendering, explicit updates)
2. **UpdatePipeline**: Triggers pipeline execution at specific time steps
3. **UpdatePipelineInformation**: Updates metadata without full execution
4. **Observer/Event System**: VTK events → Qt signals → UI updates
5. **View Refresh**: Render views update when pipeline data changes

**Key Principles:**

- **Lazy Evaluation**: Pipeline doesn't execute until data is needed
- **Time-Aware**: Pipeline execution supports time-varying data
- **Event-Driven**: UI updates via observer/event mechanism
- **Efficient Updates**: Only visible pipelines are updated
- **Information Caching**: Metadata cached and updated separately

## Architecture Overview

```
┌─────────────────────────────────────────────────────────────┐
│         ParaView Pipeline Update System                     │
│                                                             │
│  ┌─────────────────────────────────────────────────────┐   │
│  │  User Action / View Render                          │   │
│  │  - Property change                                   │   │
│  │  - View render request                              │   │
│  │  - Time step change                                 │   │
│  └──────────────┬──────────────────────────────────────┘   │
│                 │                                            │
│                 ▼                                            │
│  ┌─────────────────────────────────────────────────────┐   │
│  │  UpdatePipeline(time) / UpdatePipelineInformation()│   │
│  │  - Triggers VTK pipeline execution                 │   │
│  │  - Updates data at specified time                  │   │
│  └──────────────┬──────────────────────────────────────┘   │
│                 │                                            │
│                 ▼                                            │
│  ┌─────────────────────────────────────────────────────┐   │
│  │  VTK Pipeline Execution                              │   │
│  │  - RequestData()                                     │   │
│  │  - Data produced                                     │   │
│  └──────────────┬──────────────────────────────────────┘   │
│                 │                                            │
│                 ▼                                            │
│  ┌─────────────────────────────────────────────────────┐   │
│  │  Observer/Event System                               │   │
│  │  - vtkCommand::DataUpdateEvent                      │   │
│  │  - pqPipelineSource::dataUpdated signal             │   │
│  └──────────────┬──────────────────────────────────────┘   │
│                 │                                            │
│                 ▼                                            │
│  ┌─────────────────────────────────────────────────────┐   │
│  │  UI Updates                                          │   │
│  │  - View refresh                                     │   │
│  │  - Data information update                          │   │
│  │  - Pipeline browser update                          │   │
│  └─────────────────────────────────────────────────────┘   │
└─────────────────────────────────────────────────────────────┘
```

## Execution Model

### Demand-Driven Pipeline

**Principle**: Pipeline executes only when data is needed for visualization or explicit update.

**Key Behaviors:**

1. **Property Changes**: Don't immediately trigger execution
2. **View Rendering**: Triggers pipeline update if data is visible
3. **Explicit Updates**: `UpdatePipeline()` forces execution
4. **Visibility-Based**: Only visible pipelines are updated

### UpdatePipeline Method

**Purpose**: Execute pipeline at specified time step.

**Method Signature:**

```cpp
// vtkSMSourceProxy: Server-side proxy
class vtkSMSourceProxy : public vtkSMProxy
{
public:
    // Update pipeline at current time
    void UpdatePipeline();

    // Update pipeline at specific time
    void UpdatePipeline(double time);
};
```

**Implementation:**

```cpp
void vtkSMSourceProxy::UpdatePipeline(double time)
{
    // 1. Set time on proxy
    this->SetTime(time);

    // 2. Mark pipeline as modified
    this->MarkDirty();

    // 3. Update VTK algorithm
    vtkAlgorithm* algorithm = this->GetClientSideObject();
    if (algorithm)
    {
        // 4. Trigger pipeline execution
        algorithm->Update();
    }

    // 5. Fire DataUpdateEvent
    this->InvokeEvent(vtkCommand::DataUpdateEvent);
}
```

**Python API:**

```python
from paraview.simple import *

# Create source
sphere = Sphere()

# Update pipeline at current time
UpdatePipeline()

# Update pipeline at specific time
UpdatePipeline(time=0.5)

# Update specific proxy
sphere.UpdatePipeline()
```

### UpdatePipelineInformation Method

**Purpose**: Update metadata without full pipeline execution.

**Method Signature:**

```cpp
class vtkSMSourceProxy : public vtkSMProxy
{
public:
    // Update information properties
    void UpdatePipelineInformation();
};
```

**Implementation:**

```cpp
void vtkSMSourceProxy::UpdatePipelineInformation()
{
    // 1. Update VTK algorithm information
    vtkAlgorithm* algorithm = this->GetClientSideObject();
    if (algorithm)
    {
        // 2. Call UpdateInformation() (doesn't execute RequestData)
        algorithm->UpdateInformation();
    }

    // 3. Update information properties
    this->UpdatePropertyInformation();

    // 4. Fire InformationUpdateEvent
    this->InvokeEvent(vtkCommand::UpdateInformationEvent);
}
```

**Usage:**

```python
# Update information without executing pipeline
sphere.UpdatePipelineInformation()

# Get data information (uses cached information)
info = sphere.GetDataInformation()
print(info.GetBounds())
```

## Time Steps

### Time-Aware Pipeline

**Time Step Handling:**

```cpp
class vtkSMSourceProxy : public vtkSMProxy
{
public:
    // Set time for pipeline execution
    void SetTime(double time);

    // Get current time
    double GetTime();

    // Update pipeline at specific time
    void UpdatePipeline(double time);
};
```

### Time Step Update Flow

**Time Step Change Sequence:**

```
1. User changes time slider
   ↓
2. pqTimeKeeper updates time
   ↓
3. All proxies notified of time change
   ↓
4. Visible proxies update pipeline at new time
   ↓
5. Views refresh with new time data
```

**Implementation:**

```cpp
// pqTimeKeeper: Manages time for all proxies
class pqTimeKeeper : public QObject
{
    Q_OBJECT

public slots:
    void setTime(double time)
    {
        this->Time = time;

        // Notify all proxies of time change
        emit timeChanged(time);

        // Update visible pipelines
        this->updateVisiblePipelines(time);
    }

signals:
    void timeChanged(double time);

private:
    void updateVisiblePipelines(double time)
    {
        // Get all visible proxies
        QList<vtkSMProxy*> visibleProxies = this->getVisibleProxies();

        foreach (vtkSMProxy* proxy, visibleProxies)
        {
            vtkSMSourceProxy* sourceProxy =
                vtkSMSourceProxy::SafeDownCast(proxy);

            if (sourceProxy)
            {
                // Update pipeline at new time
                sourceProxy->UpdatePipeline(time);
            }
        }
    }

    double Time = 0.0;
};
```

## Render View Refresh

### View Update Trigger

**View Rendering Flow:**

```
1. View needs to render
   ↓
2. Check if pipeline data is up-to-date
   ↓
3. If outdated: UpdatePipeline()
   ↓
4. Render with updated data
   ↓
5. Display rendered image
```

**Implementation:**

```cpp
// pqRenderView: Base class for render views
class pqRenderView : public pqView
{
public:
    void render() override
    {
        // 1. Update all visible pipelines
        this->updateVisiblePipelines();

        // 2. Render view
        this->getRenderView()->Render();
    }

private:
    void updateVisiblePipelines()
    {
        // Get all visible representations
        QList<pqRepresentation*> reps = this->getVisibleRepresentations();

        foreach (pqRepresentation* rep, reps)
        {
            // Get source proxy
            pqPipelineSource* source = rep->getInput();
            if (source)
            {
                // Check if pipeline needs update
                if (source->needsUpdate())
                {
                    // Update pipeline
                    source->getProxy()->UpdatePipeline();
                }
            }
        }
    }
};
```

### Automatic View Updates

**Property Change → View Update:**

```cpp
// pqPipelineSource: Client-side source representation
class pqPipelineSource : public pqProxy
{
    void onPropertyChanged()
    {
        // Mark pipeline as modified
        this->MarkModified();

        // Check if visible in any view
        if (this->isVisible())
        {
            // Trigger view update
            this->updateViews();
        }
    }

    void updateViews()
    {
        // Get all views showing this source
        QList<pqView*> views = this->getViews();

        foreach (pqView* view, views)
        {
            // Schedule view render
            view->render();
        }
    }
};
```

## Data Information Updates

### GetDataInformation Method

**Purpose**: Get metadata about pipeline output.

**Method Signature:**

```cpp
class vtkSMSourceProxy : public vtkSMProxy
{
public:
    // Get data information at current time
    vtkPVDataInformation* GetDataInformation();

    // Get data information at specific time
    vtkPVDataInformation* GetDataInformation(double time);
};
```

**Implementation:**

```cpp
vtkPVDataInformation* vtkSMSourceProxy::GetDataInformation(double time)
{
    // 1. Update pipeline information first
    this->UpdatePipelineInformation();

    // 2. Update pipeline at specified time if needed
    if (this->needsUpdate(time))
    {
        this->UpdatePipeline(time);
    }

    // 3. Get output data object
    vtkDataObject* dataObject = this->GetOutputDataObject(0);

    // 4. Create or update data information
    vtkPVDataInformation* info = this->DataInformation;
    if (!info)
    {
        info = vtkPVDataInformation::New();
        this->DataInformation = info;
        info->Delete();
    }

    // 5. Copy information from data object
    info->CopyFromDataObject(dataObject);

    return info;
}
```

### Data Information Caching

**Information Caching:**

```cpp
class vtkSMSourceProxy : public vtkSMProxy
{
protected:
    // Cached data information
    vtkPVDataInformation* DataInformation = nullptr;

    void InvalidateDataInformation()
    {
        // Clear cached information when pipeline changes
        if (this->DataInformation)
        {
            this->DataInformation->Initialize();
        }
    }

    void MarkDirty() override
    {
        // Invalidate cached information
        this->InvalidateDataInformation();

        // Call base class
        vtkSMProxy::MarkDirty();
    }
};
```

**Usage:**

```python
# Get data information
info = sphere.GetDataInformation()

# Access metadata
bounds = info.GetBounds()
numPoints = info.GetNumberOfPoints()
numCells = info.GetNumberOfCells()
dataType = info.GetDataSetType()
```

## Observer/Event Mechanism

### VTK Event System

**VTK Command Events:**

```cpp
// vtkCommand: VTK event types
class vtkCommand
{
public:
    enum EventIds
    {
        DataUpdateEvent = 1000,        // Data updated
        UpdateInformationEvent = 1001, // Information updated
        PropertyModifiedEvent = 1002,   // Property modified
        // ...
    };
};
```

**Observer Registration:**

```cpp
// vtkSMProxy: Server-side proxy
class vtkSMProxy : public vtkSMObject
{
public:
    // Add observer for event
    unsigned long AddObserver(unsigned long event,
                              vtkCommand* command,
                              float priority = 0.0f);

    // Invoke event
    void InvokeEvent(unsigned long event, void* callData = nullptr);
};
```

### Client-Side Signal System

**Qt Signals:**

```cpp
// pqPipelineSource: Client-side source
class pqPipelineSource : public pqProxy
{
    Q_OBJECT

signals:
    // Emitted when data is updated
    void dataUpdated();

    // Emitted when information is updated
    void informationUpdated();

    // Emitted when visibility changes
    void visibilityChanged(bool visible);

public:
    pqPipelineSource(vtkSMProxy* proxy, pqServer* server, QObject* parent)
        : pqProxy(proxy, server, parent)
    {
        // Connect VTK event to Qt signal
        this->connectVTKEventToQtSignal(
            vtkCommand::DataUpdateEvent,
            this, SIGNAL(dataUpdated())
        );
    }

private:
    void connectVTKEventToQtSignal(unsigned long vtkEvent,
                                   QObject* qtObject,
                                   const char* qtSignal)
    {
        // Create observer that emits Qt signal
        vtkEventQtSlotConnect* connector =
            vtkEventQtSlotConnect::New();

        connector->Connect(
            this->getProxy(),
            vtkEvent,
            qtObject,
            qtSignal
        );

        this->EventConnector = connector;
    }

    vtkEventQtSlotConnect* EventConnector = nullptr;
};
```

### Event Flow

**Complete Event Flow:**

```
1. Pipeline Execution
   vtkSMSourceProxy::UpdatePipeline()
       ↓
2. VTK Algorithm Updates
   vtkAlgorithm::Update()
       ↓
3. Data Produced
   vtkAlgorithm::RequestData()
       ↓
4. VTK Event Fired
   vtkSMSourceProxy::InvokeEvent(vtkCommand::DataUpdateEvent)
       ↓
5. Observer Notified
   vtkEventQtSlotConnect::Execute()
       ↓
6. Qt Signal Emitted
   pqPipelineSource::dataUpdated()
       ↓
7. UI Components Update
   - View refresh
   - Data information update
   - Pipeline browser update
```

### UI Component Updates

**View Updates:**

```cpp
// pqRenderView: Render view
class pqRenderView : public pqView
{
    Q_OBJECT

public slots:
    void onDataUpdated()
    {
        // Source data was updated
        // Schedule view render
        this->render();
    }

public:
    pqRenderView(const QString& type,
                 const QString& group,
                 const QString& name,
                 vtkSMViewProxy* viewProxy,
                 pqServer* server,
                 QObject* parent)
        : pqView(type, group, name, viewProxy, server, parent)
    {
        // Connect to source data updates
        // (via pqPipelineSource::dataUpdated signal)
    }
};
```

**Data Information Updates:**

```cpp
// pqDataInformationWidget: Shows data information
class pqDataInformationWidget : public QWidget
{
    Q_OBJECT

public slots:
    void onDataUpdated()
    {
        // Update data information display
        this->updateInformation();
    }

private:
    void updateInformation()
    {
        // Get current source
        pqPipelineSource* source = this->getCurrentSource();
        if (!source)
        {
            return;
        }

        // Get data information
        vtkPVDataInformation* info =
            source->getProxy()->GetDataInformation();

        // Update UI
        this->updateUI(info);
    }
};
```

## Complete Update Flow Example

### Scenario: Property Change → View Update

**Step-by-Step Flow:**

```
1. User Changes Property
   User modifies "Radius" property in Properties panel
       ↓
2. Property Modified Event
   vtkSMProperty::Modified()
   → vtkCommand::PropertyModifiedEvent
       ↓
3. Client Proxy Notified
   pqPipelineSource::onPropertyChanged()
   → Marks pipeline as modified
       ↓
4. View Render Requested
   pqRenderView::render()
   → Checks if pipeline needs update
       ↓
5. Pipeline Update
   vtkSMSourceProxy::UpdatePipeline()
   → VTK algorithm::Update()
       ↓
6. Data Produced
   vtkAlgorithm::RequestData()
   → New data created
       ↓
7. Data Update Event
   vtkSMSourceProxy::InvokeEvent(vtkCommand::DataUpdateEvent)
       ↓
8. Qt Signal Emitted
   pqPipelineSource::dataUpdated()
       ↓
9. View Refreshed
   pqRenderView::onDataUpdated()
   → Render with new data
       ↓
10. UI Updated
    - View displays new data
    - Data information panel updated
    - Pipeline browser reflects changes
```

### Code Implementation

**Complete Implementation:**

```cpp
// 1. Property change handler
void pqPipelineSource::onPropertyChanged(vtkSMProperty* property)
{
    // Mark pipeline as modified
    this->MarkModified();

    // Emit property changed signal
    emit propertyChanged(property);

    // Check if visible
    if (this->isVisible())
    {
        // Trigger view update
        this->updateViews();
    }
}

// 2. View update
void pqRenderView::render()
{
    // Update visible pipelines
    this->updateVisiblePipelines();

    // Render view
    this->getRenderView()->Render();
}

// 3. Pipeline update
void vtkSMSourceProxy::UpdatePipeline(double time)
{
    // Set time
    this->SetTime(time);

    // Update VTK algorithm
    vtkAlgorithm* algorithm = this->GetClientSideObject();
    if (algorithm)
    {
        algorithm->Update();
    }

    // Fire event
    this->InvokeEvent(vtkCommand::DataUpdateEvent);
}

// 4. Event to signal connection
void pqPipelineSource::connectVTKEventToQtSignal(
    unsigned long vtkEvent,
    QObject* qtObject,
    const char* qtSignal)
{
    vtkEventQtSlotConnect* connector = vtkEventQtSlotConnect::New();

    connector->Connect(
        this->getProxy(),
        vtkEvent,
        qtObject,
        qtSignal
    );

    this->EventConnector = connector;
}

// 5. View update handler
void pqRenderView::onDataUpdated()
{
    // Schedule render
    this->render();
}
```

## Time Step Updates

### Time-Aware Updates

**Time Step Change Flow:**

```
1. User Changes Time Slider
   Time slider value changed
       ↓
2. Time Keeper Updates
   pqTimeKeeper::setTime(time)
   → Emits timeChanged(time) signal
       ↓
3. Proxies Notified
   pqPipelineSource::onTimeChanged(time)
   → Updates proxy time
       ↓
4. Visible Pipelines Update
   vtkSMSourceProxy::UpdatePipeline(time)
   → Executes pipeline at new time
       ↓
5. Views Refresh
   pqRenderView::render()
   → Displays data at new time
```

**Implementation:**

```cpp
// pqTimeKeeper: Manages time
class pqTimeKeeper : public QObject
{
    Q_OBJECT

public slots:
    void setTime(double time)
    {
        if (this->Time != time)
        {
            this->Time = time;
            emit timeChanged(time);

            // Update visible pipelines
            this->updateVisiblePipelines(time);
        }
    }

signals:
    void timeChanged(double time);

private:
    void updateVisiblePipelines(double time)
    {
        // Get all visible sources
        QList<pqPipelineSource*> sources = this->getVisibleSources();

        foreach (pqPipelineSource* source, sources)
        {
            vtkSMSourceProxy* proxy =
                vtkSMSourceProxy::SafeDownCast(source->getProxy());

            if (proxy)
            {
                // Update at new time
                proxy->UpdatePipeline(time);
            }
        }
    }

    double Time = 0.0;
};
```

## Summary

### Execution Model

1. **Demand-Driven**: Pipeline executes only when needed
2. **UpdatePipeline(time)**: Executes pipeline at specific time
3. **UpdatePipelineInformation()**: Updates metadata without execution
4. **Time-Aware**: Supports time-varying data

### View Refresh

1. **Automatic Updates**: Views update when visible pipelines change
2. **Render Trigger**: Rendering triggers pipeline updates
3. **Visibility-Based**: Only visible pipelines are updated
4. **Efficient**: Minimizes unnecessary computations

### Data Information

1. **GetDataInformation()**: Returns metadata about output
2. **Caching**: Information cached and invalidated on changes
3. **Time-Aware**: Information available for specific time steps
4. **Automatic Updates**: Information updated when pipeline executes

### Observer/Event Mechanism

1. **VTK Events**: `vtkCommand::DataUpdateEvent`, `UpdateInformationEvent`
2. **Qt Signals**: `pqPipelineSource::dataUpdated()`, `informationUpdated()`
3. **Event Bridge**: `vtkEventQtSlotConnect` connects VTK events to Qt signals
4. **UI Updates**: Views, information panels, pipeline browser update via signals

### Key Classes

- **Server-Side**: `vtkSMSourceProxy`, `vtkSMProxy`, `vtkCommand`
- **Client-Side**: `pqPipelineSource`, `pqRenderView`, `pqTimeKeeper`
- **Event Bridge**: `vtkEventQtSlotConnect`
- **Data Info**: `vtkPVDataInformation`

This architecture ensures **efficient pipeline execution**, **responsive UI updates**, and **seamless synchronization** between pipeline state and view display.
