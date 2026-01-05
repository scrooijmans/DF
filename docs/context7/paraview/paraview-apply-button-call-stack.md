# ParaView Apply Button Call Stack

This document explains in detail the call stack for "Apply" in ParaView: from UI parameter changes → proxy property updates → pipeline execution → representation updates → view render, including where caching and invalidation occur.

## Overview

The "Apply" button in ParaView triggers a **multi-stage update process** that synchronizes UI changes with pipeline execution and view rendering:

1. **UI Parameter Changes**: User modifies properties in Properties panel
2. **Apply Button Click**: Commits changes to proxy
3. **Property Updates**: Proxy properties updated on server
4. **Cache Invalidation**: Cached data marked as invalid
5. **Pipeline Execution**: VTK pipeline executes with new parameters
6. **Representation Updates**: Representations update to reflect new data
7. **View Render**: View renders updated representations

**Key Principles:**

- **Deferred Execution**: Changes applied only when "Apply" clicked
- **Demand-Driven**: Pipeline executes only if output is visible
- **Cache Invalidation**: Caches invalidated when properties change
- **Efficient Updates**: Only modified components are updated

## Architecture Overview

```
┌─────────────────────────────────────────────────────────────┐
│         ParaView Apply Button Call Stack                    │
│                                                             │
│  1. UI Parameter Change                                     │
│     pqPropertyWidget::textChanged()                         │
│       ↓                                                      │
│  2. Apply Button Click                                      │
│     pqPropertyWidget::apply()                              │
│       ↓                                                      │
│  3. Proxy Property Update                                   │
│     vtkSMProperty::SetElement()                            │
│       ↓                                                      │
│  4. Cache Invalidation                                      │
│     vtkSMProxy::MarkDirty()                                │
│       ↓                                                      │
│  5. Pipeline Execution                                      │
│     vtkSMSourceProxy::UpdatePipeline()                     │
│       ↓                                                      │
│  6. Representation Update                                   │
│     pqRepresentation::dataUpdated()                        │
│       ↓                                                      │
│  7. View Render                                             │
│     pqRenderView::render()                                  │
│       ↓                                                      │
│  8. Display Updated Visualization                           │
└─────────────────────────────────────────────────────────────┘
```

## Stage 1: UI Parameter Changes

### Property Widget

**User Modifies Property:**

```cpp
// pqPropertyWidget: Base class for property widgets
class pqPropertyWidget : public QWidget
{
    Q_OBJECT

public slots:
    // Called when user changes property value
    void onTextChanged(const QString& text)
    {
        // Mark widget as modified
        this->Modified = true;

        // Enable Apply button
        this->enableApplyButton(true);

        // Store pending value (not yet applied)
        this->PendingValue = text;
    }

signals:
    // Emitted when Apply is clicked
    void changeAvailable();

private:
    bool Modified = false;
    QString PendingValue;
};
```

**Apply Button State:**

```cpp
void pqPropertyWidget::enableApplyButton(bool enabled)
{
    // Get Apply button from parent panel
    QPushButton* applyButton = this->findApplyButton();

    if (applyButton)
    {
        applyButton->setEnabled(enabled);
        applyButton->setStyleSheet(
            enabled ? "background-color: orange;" : ""
        );
    }
}
```

## Stage 2: Apply Button Click

### Apply Handler

**Apply Button Clicked:**

```cpp
// pqPropertiesPanel: Properties panel container
class pqPropertiesPanel : public QWidget
{
    Q_OBJECT

public slots:
    void onApplyClicked()
    {
        // Get current proxy
        pqProxy* proxy = this->getCurrentProxy();
        if (!proxy)
        {
            return;
        }

        // Apply all modified properties
        this->applyProperties(proxy);
    }

private:
    void applyProperties(pqProxy* proxy)
    {
        // Get all property widgets
        QList<pqPropertyWidget*> widgets = this->getPropertyWidgets();

        foreach (pqPropertyWidget* widget, widgets)
        {
            if (widget->isModified())
            {
                // Apply property change
                widget->apply();
            }
        }

        // Disable Apply button
        this->getApplyButton()->setEnabled(false);
    }
};
```

**Property Widget Apply:**

```cpp
void pqPropertyWidget::apply()
{
    if (!this->Modified)
    {
        return;
    }

    // Get server-side property
    vtkSMProperty* property = this->getProperty();
    if (!property)
    {
        return;
    }

    // Set property value on server
    this->setPropertyValue(property, this->PendingValue);

    // Mark as applied
    this->Modified = false;
    this->PendingValue.clear();

    // Emit signal
    emit changeApplied();
}
```

## Stage 3: Proxy Property Updates

### Property Update

**Set Property Value:**

```cpp
void pqPropertyWidget::setPropertyValue(vtkSMProperty* property,
                                       const QString& value)
{
    // Get proxy
    vtkSMProxy* proxy = property->GetParent();

    // Set property value
    if (property->IsA("vtkSMDoubleVectorProperty"))
    {
        vtkSMDoubleVectorProperty* doubleProp =
            vtkSMDoubleVectorProperty::SafeDownCast(property);
        doubleProp->SetElement(0, value.toDouble());
    }
    else if (property->IsA("vtkSMIntVectorProperty"))
    {
        vtkSMIntVectorProperty* intProp =
            vtkSMIntVectorProperty::SafeDownCast(property);
        intProp->SetElement(0, value.toInt());
    }
    else if (property->IsA("vtkSMStringVectorProperty"))
    {
        vtkSMStringVectorProperty* stringProp =
            vtkSMStringVectorProperty::SafeDownCast(property);
        stringProp->SetElement(0, value.toStdString().c_str());
    }

    // Property modified event
    property->Modified();
}
```

**Property Modified Event:**

```cpp
// vtkSMProperty: Server-side property
class vtkSMProperty : public vtkSMObject
{
public:
    void Modified()
    {
        // Fire property modified event
        this->InvokeEvent(vtkCommand::PropertyModifiedEvent);

        // Notify proxy
        vtkSMProxy* proxy = this->GetParent();
        if (proxy)
        {
            proxy->OnPropertyModified(this);
        }
    }
};
```

## Stage 4: Cache Invalidation

### Proxy Marking as Dirty

**Proxy Property Modified Handler:**

```cpp
// vtkSMProxy: Server-side proxy
class vtkSMProxy : public vtkSMObject
{
public:
    void OnPropertyModified(vtkSMProperty* property)
    {
        // Mark proxy as modified
        this->MarkDirty();

        // Invalidate dependent caches
        this->InvalidateCaches();
    }

    void MarkDirty()
    {
        // Set dirty flag
        this->Dirty = true;

        // Invalidate data information cache
        if (this->DataInformation)
        {
            this->DataInformation->Initialize();
        }

        // Fire proxy modified event
        this->InvokeEvent(vtkCommand::ModifiedEvent);
    }

    void InvalidateCaches()
    {
        // Invalidate data delivery cache
        vtkPVDataDeliveryManager* deliveryManager =
            this->GetDataDeliveryManager();
        if (deliveryManager)
        {
            deliveryManager->InvalidateCache(this);
        }

        // Invalidate representation caches
        this->InvalidateRepresentationCaches();
    }

private:
    bool Dirty = false;
    vtkPVDataInformation* DataInformation = nullptr;
};
```

### Data Delivery Cache Invalidation

**Cache Invalidation:**

```cpp
// vtkPVDataDeliveryManager: Manages data delivery and caching
class vtkPVDataDeliveryManager
{
public:
    void InvalidateCache(vtkSMProxy* proxy)
    {
        // Find all representations using this proxy
        QList<vtkSMRepresentationProxy*> reps =
            this->getRepresentationsForProxy(proxy);

        foreach (vtkSMRepresentationProxy* rep, reps)
        {
            // Invalidate representation cache
            rep->InvalidateCache();
        }
    }
};
```

**Representation Cache Invalidation:**

```cpp
// vtkSMRepresentationProxy: Representation proxy
class vtkSMRepresentationProxy : public vtkSMProxy
{
public:
    void InvalidateCache()
    {
        // Clear cached data
        this->CachedData = nullptr;

        // Mark representation as needing update
        this->NeedsUpdate = true;

        // Fire update needed event
        this->InvokeEvent(vtkCommand::UpdateEvent);
    }

private:
    vtkDataObject* CachedData = nullptr;
    bool NeedsUpdate = false;
};
```

## Stage 5: Pipeline Execution

### Pipeline Update Trigger

**Source Proxy Update:**

```cpp
// vtkSMSourceProxy: Source proxy
class vtkSMSourceProxy : public vtkSMProxy
{
public:
    void UpdatePipeline(double time = -1)
    {
        // Check if pipeline needs update
        if (!this->NeedsUpdate())
        {
            return;
        }

        // Set time if specified
        if (time >= 0)
        {
            this->SetTime(time);
        }

        // Get VTK algorithm
        vtkAlgorithm* algorithm = this->GetClientSideObject();
        if (!algorithm)
        {
            return;
        }

        // Update pipeline
        algorithm->Update();

        // Clear dirty flag
        this->Dirty = false;

        // Fire data update event
        this->InvokeEvent(vtkCommand::DataUpdateEvent);
    }

    bool NeedsUpdate()
    {
        // Check if proxy is dirty
        if (this->Dirty)
        {
            return true;
        }

        // Check if visible in any view
        if (this->IsVisible())
        {
            return true;
        }

        return false;
    }
};
```

**VTK Algorithm Update:**

```cpp
// vtkAlgorithm: VTK pipeline algorithm
class vtkAlgorithm : public vtkObject
{
public:
    void Update()
    {
        // Update information first
        this->UpdateInformation();

        // Update data
        this->UpdateData();
    }

    void UpdateData()
    {
        // Check if data needs update
        if (!this->GetOutputInformation(0)->Get(
            vtkDataObject::DATA_OBJECT())->GetMTime() <
            this->GetMTime())
        {
            // Execute pipeline
            this->RequestData(nullptr, nullptr, nullptr);
        }
    }
};
```

## Stage 6: Representation Updates

### Representation Data Update

**Representation Update Handler:**

```cpp
// pqRepresentation: Client-side representation
class pqRepresentation : public pqProxy
{
    Q_OBJECT

public slots:
    void onDataUpdated()
    {
        // Source data was updated
        // Update representation
        this->updateRepresentation();
    }

private:
    void updateRepresentation()
    {
        // Get server-side representation proxy
        vtkSMRepresentationProxy* repProxy =
            vtkSMRepresentationProxy::SafeDownCast(this->getProxy());

        if (!repProxy)
        {
            return;
        }

        // Update representation
        repProxy->UpdatePipeline();

        // Update mapper with new data
        this->updateMapper();

        // Emit representation updated signal
        emit representationUpdated();
    }

    void updateMapper()
    {
        // Get mapper proxy
        vtkSMMapperProxy* mapperProxy =
            repProxy->GetMapperProxy();

        if (mapperProxy)
        {
            // Update mapper with new input
            mapperProxy->UpdatePipeline();
        }
    }
};
```

**Representation Proxy Update:**

```cpp
// vtkSMRepresentationProxy: Server-side representation
class vtkSMRepresentationProxy : public vtkSMProxy
{
public:
    void UpdatePipeline(double time = -1)
    {
        // Get input source
        vtkSMSourceProxy* inputSource = this->GetInputProxy();

        if (inputSource)
        {
            // Update input source first
            inputSource->UpdatePipeline(time);
        }

        // Update representation properties
        this->UpdatePropertyInformation();

        // Update mapper
        vtkSMMapperProxy* mapper = this->GetMapperProxy();
        if (mapper)
        {
            mapper->UpdatePipeline(time);
        }

        // Fire representation updated event
        this->InvokeEvent(vtkCommand::UpdateEvent);
    }
};
```

## Stage 7: View Render

### View Render Trigger

**View Render:**

```cpp
// pqRenderView: Render view
class pqRenderView : public pqView
{
    Q_OBJECT

public:
    void render() override
    {
        // Update all visible representations
        this->updateVisibleRepresentations();

        // Render view
        this->getRenderView()->Render();
    }

private:
    void updateVisibleRepresentations()
    {
        // Get all visible representations
        QList<pqRepresentation*> reps = this->getVisibleRepresentations();

        foreach (pqRepresentation* rep, reps)
        {
            // Check if representation needs update
            if (rep->needsUpdate())
            {
                // Update representation
                rep->updateRepresentation();
            }
        }
    }
};
```

**Render View Render:**

```cpp
// vtkPVRenderView: Server-side render view
class vtkPVRenderView : public vtkRenderView
{
public:
    void Render()
    {
        // Update all representations
        this->Update();

        // Render
        vtkRenderView::Render();
    }

    void Update()
    {
        // Get all representations
        vtkCollection* reps = this->GetRepresentations();

        reps->InitTraversal();
        while (vtkSMRepresentationProxy* rep =
               vtkSMRepresentationProxy::SafeDownCast(
                   reps->GetNextItemAsObject()))
        {
            // Update representation
            rep->UpdatePipeline();
        }
    }
};
```

## Complete Call Stack

### Detailed Call Sequence

**Complete Call Stack:**

```
1. User Modifies Property
   User changes "Radius" in Properties panel
       ↓
2. Property Widget Modified
   pqPropertyWidget::onTextChanged("10.0")
   → Modified = true
   → PendingValue = "10.0"
   → Apply button enabled
       ↓
3. User Clicks Apply
   pqPropertiesPanel::onApplyClicked()
       ↓
4. Apply Properties
   pqPropertyWidget::apply()
   → setPropertyValue(property, "10.0")
       ↓
5. Set Property on Server
   vtkSMDoubleVectorProperty::SetElement(0, 10.0)
   → property->Modified()
       ↓
6. Property Modified Event
   vtkSMProperty::Modified()
   → InvokeEvent(vtkCommand::PropertyModifiedEvent)
   → proxy->OnPropertyModified(property)
       ↓
7. Proxy Marked Dirty
   vtkSMProxy::OnPropertyModified(property)
   → MarkDirty()
   → InvalidateCaches()
       ↓
8. Cache Invalidation
   vtkSMProxy::InvalidateCaches()
   → DataInformation->Initialize()
   → deliveryManager->InvalidateCache(proxy)
   → representation->InvalidateCache()
       ↓
9. Check if Pipeline Needs Update
   vtkSMSourceProxy::NeedsUpdate()
   → Check if Dirty (true)
   → Check if Visible (true)
   → Return true
       ↓
10. Update Pipeline
    vtkSMSourceProxy::UpdatePipeline()
    → SetTime(time)
    → algorithm->Update()
        ↓
11. VTK Algorithm Update
    vtkAlgorithm::Update()
    → UpdateInformation()
    → UpdateData()
    → RequestData()
        ↓
12. Data Produced
    vtkAlgorithm::RequestData()
    → New data created
    → DataUpdateEvent fired
        ↓
13. Data Update Event
    vtkSMSourceProxy::InvokeEvent(vtkCommand::DataUpdateEvent)
    → Observer notified
        ↓
14. Qt Signal Emitted
    pqPipelineSource::dataUpdated()
    → Representation::onDataUpdated()
        ↓
15. Representation Update
    pqRepresentation::onDataUpdated()
    → updateRepresentation()
    → repProxy->UpdatePipeline()
    → mapper->UpdatePipeline()
        ↓
16. Representation Updated Signal
    pqRepresentation::representationUpdated()
    → View::onRepresentationUpdated()
        ↓
17. View Render
    pqRenderView::onRepresentationUpdated()
    → render()
    → updateVisibleRepresentations()
    → getRenderView()->Render()
        ↓
18. Render View Update
    vtkPVRenderView::Render()
    → Update()
    → Render()
        ↓
19. Display Updated
    View displays new visualization
```

## Caching and Invalidation Points

### Cache Invalidation Locations

**1. Property Modified:**

```cpp
void vtkSMProxy::OnPropertyModified(vtkSMProperty* property)
{
    // Invalidate data information cache
    if (this->DataInformation)
    {
        this->DataInformation->Initialize();
    }

    // Mark proxy as dirty
    this->MarkDirty();
}
```

**2. Proxy Marked Dirty:**

```cpp
void vtkSMProxy::MarkDirty()
{
    // Invalidate data delivery cache
    vtkPVDataDeliveryManager* deliveryManager =
        this->GetDataDeliveryManager();
    if (deliveryManager)
    {
        deliveryManager->InvalidateCache(this);
    }
}
```

**3. Representation Cache:**

```cpp
void vtkSMRepresentationProxy::InvalidateCache()
{
    // Clear cached data
    this->CachedData = nullptr;

    // Mark as needing update
    this->NeedsUpdate = true;
}
```

### Cache Usage

**Data Information Cache:**

```cpp
vtkPVDataInformation* vtkSMSourceProxy::GetDataInformation()
{
    // Check if cache is valid
    if (this->DataInformation &&
        !this->DataInformation->IsEmpty() &&
        !this->Dirty)
    {
        // Return cached information
        return this->DataInformation;
    }

    // Update pipeline if needed
    if (this->Dirty)
    {
        this->UpdatePipeline();
    }

    // Update information
    this->UpdatePipelineInformation();

    // Create/update cache
    if (!this->DataInformation)
    {
        this->DataInformation = vtkPVDataInformation::New();
    }

    // Copy from data object
    vtkDataObject* dataObject = this->GetOutputDataObject(0);
    this->DataInformation->CopyFromDataObject(dataObject);

    return this->DataInformation;
}
```

**Representation Data Cache:**

```cpp
vtkDataObject* vtkSMRepresentationProxy::GetRenderedData()
{
    // Check cache
    if (this->CachedData && !this->NeedsUpdate)
    {
        return this->CachedData;
    }

    // Update pipeline
    this->UpdatePipeline();

    // Get data from mapper
    vtkSMMapperProxy* mapper = this->GetMapperProxy();
    if (mapper)
    {
        this->CachedData = mapper->GetInputDataObject();
    }

    this->NeedsUpdate = false;
    return this->CachedData;
}
```

## Event/Signal Flow

### Observer Registration

**Client-Side Observer:**

```cpp
// pqPipelineSource: Client-side source
class pqPipelineSource : public pqProxy
{
    Q_OBJECT

public:
    pqPipelineSource(vtkSMProxy* proxy, pqServer* server, QObject* parent)
        : pqProxy(proxy, server, parent)
    {
        // Connect VTK event to Qt signal
        vtkEventQtSlotConnect* connector = vtkEventQtSlotConnect::New();

        connector->Connect(
            proxy,
            vtkCommand::DataUpdateEvent,
            this,
            SLOT(onDataUpdated())
        );

        this->EventConnector = connector;
    }

signals:
    void dataUpdated();

private slots:
    void onDataUpdated()
    {
        emit dataUpdated();
    }

    vtkEventQtSlotConnect* EventConnector = nullptr;
};
```

**Representation Observer:**

```cpp
// pqRepresentation: Client-side representation
class pqRepresentation : public pqProxy
{
    Q_OBJECT

public:
    pqRepresentation(vtkSMProxy* proxy, pqServer* server, QObject* parent)
        : pqProxy(proxy, server, parent)
    {
        // Connect to source data updates
        pqPipelineSource* source = this->getInput();
        if (source)
        {
            connect(source, SIGNAL(dataUpdated()),
                    this, SLOT(onDataUpdated()));
        }
    }

private slots:
    void onDataUpdated()
    {
        // Update representation
        this->updateRepresentation();

        // Notify view
        emit representationUpdated();
    }
};
```

**View Observer:**

```cpp
// pqRenderView: Render view
class pqRenderView : public pqView
{
    Q_OBJECT

public:
    pqRenderView(vtkSMViewProxy* viewProxy, pqServer* server, QObject* parent)
        : pqView(viewProxy, server, parent)
    {
        // Connect to representation updates
        // (via representationUpdated signals)
    }

private slots:
    void onRepresentationUpdated()
    {
        // Schedule render
        this->render();
    }
};
```

## Summary

### Call Stack Summary

1. **UI Parameter Change**: `pqPropertyWidget::onTextChanged()`
2. **Apply Click**: `pqPropertiesPanel::onApplyClicked()`
3. **Property Update**: `vtkSMProperty::SetElement()`
4. **Cache Invalidation**: `vtkSMProxy::MarkDirty()` → `InvalidateCaches()`
5. **Pipeline Execution**: `vtkSMSourceProxy::UpdatePipeline()` → `vtkAlgorithm::Update()`
6. **Representation Update**: `pqRepresentation::onDataUpdated()` → `updateRepresentation()`
7. **View Render**: `pqRenderView::render()` → `vtkPVRenderView::Render()`

### Caching Points

1. **Data Information Cache**: Cached in `vtkSMSourceProxy::DataInformation`
2. **Representation Data Cache**: Cached in `vtkSMRepresentationProxy::CachedData`
3. **Data Delivery Cache**: Managed by `vtkPVDataDeliveryManager`

### Invalidation Points

1. **Property Modified**: `vtkSMProxy::OnPropertyModified()` → `MarkDirty()`
2. **Proxy Dirty**: `vtkSMProxy::MarkDirty()` → `InvalidateCaches()`
3. **Representation Cache**: `vtkSMRepresentationProxy::InvalidateCache()`

### Event Flow

1. **VTK Events**: `vtkCommand::PropertyModifiedEvent`, `DataUpdateEvent`
2. **Qt Signals**: `pqPipelineSource::dataUpdated()`, `pqRepresentation::representationUpdated()`
3. **Event Bridge**: `vtkEventQtSlotConnect` connects VTK events to Qt slots

This architecture ensures **efficient updates**, **proper cache invalidation**, and **synchronized UI and pipeline state**.
