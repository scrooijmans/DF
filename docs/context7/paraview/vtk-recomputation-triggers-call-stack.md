# ParaView Recomputation Triggers: From UI Interaction to VTK Pipeline Execution

## Executive Summary

ParaView uses a **lazy evaluation model** where pipeline execution is triggered by specific user actions and system events. The three primary triggers are: (1) **Apply button clicks**, (2) **time step changes**, and (3) **parameter edits** (with auto-apply enabled). Each trigger follows a well-defined call stack from the UI layer through the Server Manager proxy system to the VTK pipeline execution engine. Understanding these triggers and the complete call stack is essential for developing ParaView plugins and understanding pipeline behavior.

**Key Triggers**:
- **Apply Button**: Explicit user request to execute pipeline
- **Time Step Changes**: Temporal data navigation triggers recomputation
- **Parameter Edits**: With auto-apply, parameter changes trigger immediate recomputation
- **View Updates**: Render views may trigger pipeline execution when data is needed

**Call Stack Layers**:
1. **UI Layer**: Qt widgets, user interactions
2. **Server Manager Layer**: Proxy management, property updates
3. **VTK Algorithm Layer**: Pipeline execution, request processing
4. **Data Object Layer**: Data creation and caching

---

## 1. Recomputation Triggers Overview

### Trigger Types

ParaView supports several mechanisms that trigger pipeline recomputation:

1. **Explicit Apply Button**
   - User clicks "Apply" button in Properties panel
   - Most common trigger for manual control
   - Gives user control over when computation occurs

2. **Time Step Changes**
   - User navigates temporal data (time slider, animation)
   - Triggers recomputation with new time step
   - Essential for time-varying datasets

3. **Parameter Edits (Auto-Apply)**
   - When auto-apply is enabled, parameter changes trigger immediate recomputation
   - Provides interactive feedback
   - Can be performance-intensive

4. **View Updates**
   - Render views may trigger pipeline execution when rendering
   - Automatic background updates
   - Lazy evaluation for visualization

5. **Pipeline Topology Changes**
   - Adding/removing filters
   - Changing connections
   - Automatically triggers recomputation

### Trigger Comparison

| Trigger | User Control | Performance | Use Case |
|---------|-------------|-------------|----------|
| Apply Button | High (explicit) | Efficient (on-demand) | Manual workflow, large datasets |
| Auto-Apply | Low (automatic) | Potentially expensive | Interactive exploration, small datasets |
| Time Step | Medium (navigation) | Efficient (cached per time) | Temporal data analysis |
| View Update | Low (automatic) | Efficient (lazy) | Background rendering |

---

## 2. Apply Button Trigger

### User Interaction Flow

**Step 1: User Clicks Apply Button**

```
UI Layer (Qt)
    │
    ├─> pqPropertiesPanel::onApply()
    │   └─> Emits signal: applyRequested()
    │
    └─> pqProxyWidget::apply()
        └─> Updates proxy properties
```

**Step 2: Property Update**

```cpp
// In pqProxyWidget::apply()
void pqProxyWidget::apply() {
    // Get the proxy
    vtkSMProxy* proxy = this->getProxy();
    
    // Update all properties from UI widgets
    this->updatePropertiesFromWidgets(proxy);
    
    // Mark proxy as modified
    proxy->MarkModified(nullptr);
    
    // Trigger pipeline update
    proxy->UpdateVTKObjects();
    proxy->UpdatePipeline();
}
```

**Step 3: Server Manager Pipeline Update**

```cpp
// In vtkSMProxy::UpdatePipeline()
void vtkSMProxy::UpdatePipeline(double time) {
    // Get the VTK algorithm
    vtkAlgorithm* algorithm = vtkAlgorithm::SafeDownCast(
        this->GetClientSideObject());
    
    if (algorithm) {
        // Update VTK object properties
        this->UpdateVTKObjects();
        
        // Set time step if temporal
        if (time >= 0.0) {
            vtkInformation* info = algorithm->GetOutputInformation(0);
            info->Set(vtkStreamingDemandDrivenPipeline::UPDATE_TIME_STEP(), time);
        }
        
        // Trigger VTK pipeline execution
        algorithm->Update();
    }
}
```

### Complete Apply Button Call Stack

```
1. User clicks "Apply" button
   │
   ▼
2. Qt Signal/Slot: pqPropertiesPanel::onApply()
   │
   ▼
3. pqProxyWidget::apply()
   │   ├─> Update properties from UI widgets
   │   ├─> proxy->MarkModified(nullptr)
   │   └─> proxy->UpdateVTKObjects()
   │
   ▼
4. vtkSMProxy::UpdatePipeline()
   │   ├─> Get VTK algorithm: GetClientSideObject()
   │   ├─> Update VTK object properties
   │   ├─> Set time step (if temporal)
   │   └─> algorithm->Update()
   │
   ▼
5. vtkAlgorithm::Update()
   │   ├─> Check if execution needed: NeedToExecute()
   │   ├─> Propagate upstream: UpdateInformation()
   │   └─> Execute: ProcessRequest()
   │
   ▼
6. vtkAlgorithm::ProcessRequest()
   │   ├─> RequestInformation() - Metadata propagation
   │   ├─> RequestUpdateExtent() - Extent determination
   │   └─> RequestData() - Actual computation
   │
   ▼
7. vtkAlgorithm::RequestData()
   │   ├─> Get input data objects
   │   ├─> Create output data object
   │   ├─> Execute algorithm logic
   │   └─> Set output data
   │
   ▼
8. Data Available
   │   ├─> Output cached in algorithm
   │   ├─> Available for visualization
   │   └─> View updates automatically
```

### Code Example: Apply Button Handler

```cpp
// In pqPropertiesPanel (ParaView UI layer)
void pqPropertiesPanel::onApply() {
    // Get current proxy
    vtkSMProxy* proxy = this->getProxy();
    if (!proxy) return;
    
    // Update properties from UI
    pqProxyWidget* proxyWidget = this->getProxyWidget();
    proxyWidget->apply();
    
    // Mark as modified
    proxy->MarkModified(nullptr);
    
    // Update VTK objects
    proxy->UpdateVTKObjects();
    
    // Trigger pipeline execution
    proxy->UpdatePipeline();
    
    // Update view
    pqView* view = this->getView();
    if (view) {
        view->render();
    }
}
```

---

## 3. Time Step Change Trigger

### Temporal Data Navigation

**Step 1: User Changes Time Step**

```
UI Layer
    │
    ├─> pqAnimationTimeWidget::setAnimationTime()
    │   └─> Emits signal: timeChanged(double)
    │
    └─> pqTimeKeeper::setTime(double)
        └─> Updates time keeper
```

**Step 2: Time Keeper Update**

```cpp
// In pqTimeKeeper
void pqTimeKeeper::setTime(double time) {
    if (this->Time != time) {
        this->Time = time;
        
        // Notify all proxies of time change
        vtkSMProxyManager* proxyManager = 
            vtkSMProxyManager::GetProxyManager();
        
        // Get all proxies
        vtkSMProxyIterator* iter = vtkSMProxyIterator::New();
        iter->SetModeToAll();
        
        for (iter->Begin(); !iter->IsAtEnd(); iter->Next()) {
            vtkSMProxy* proxy = iter->GetProxy();
            
            // Update pipeline with new time
            proxy->UpdatePipeline(time);
        }
        
        iter->Delete();
        
        // Emit signal
        this->InvokeEvent(vtkCommand::TimeChangedEvent, &time);
    }
}
```

**Step 3: Pipeline Update with Time**

```cpp
// In vtkSMProxy::UpdatePipeline(double time)
void vtkSMProxy::UpdatePipeline(double time) {
    vtkAlgorithm* algorithm = vtkAlgorithm::SafeDownCast(
        this->GetClientSideObject());
    
    if (algorithm) {
        // Set time step in output information
        vtkInformation* outInfo = algorithm->GetOutputInformation(0);
        outInfo->Set(vtkStreamingDemandDrivenPipeline::UPDATE_TIME_STEP(), time);
        
        // Trigger update
        algorithm->Update();
    }
}
```

### Complete Time Step Call Stack

```
1. User moves time slider / animation plays
   │
   ▼
2. pqAnimationTimeWidget::setAnimationTime(double time)
   │   └─> Emits: timeChanged(time)
   │
   ▼
3. pqTimeKeeper::setTime(double time)
   │   ├─> Update internal time
   │   ├─> Iterate all proxies
   │   └─> proxy->UpdatePipeline(time)
   │
   ▼
4. vtkSMProxy::UpdatePipeline(double time)
   │   ├─> Get VTK algorithm
   │   ├─> Set UPDATE_TIME_STEP in output information
   │   └─> algorithm->Update()
   │
   ▼
5. vtkAlgorithm::Update()
   │   ├─> Check cache validity (includes time step)
   │   ├─> If time changed, cache invalid
   │   └─> ProcessRequest()
   │
   ▼
6. vtkAlgorithm::ProcessRequest()
   │   ├─> RequestInformation() - with time step
   │   ├─> RequestUpdateExtent() - with time step
   │   └─> RequestData() - compute for time step
   │
   ▼
7. vtkAlgorithm::RequestData()
   │   ├─> Get time step from request
   │   ├─> Load/compute data for that time
   │   └─> Create output with time metadata
   │
   ▼
8. Temporal Data Available
   │   ├─> Output cached with time step
   │   ├─> View updates with temporal data
   │   └─> Animation continues
```

### Code Example: Time Step Handler

```cpp
// In vtkTemporalReader (example temporal source)
int vtkTemporalReader::RequestData(
    vtkInformation* request,
    vtkInformationVector** inputVector,
    vtkInformationVector* outputVector) {
    
    // Get requested time step
    double timeStep = 0.0;
    if (request->Has(vtkStreamingDemandDrivenPipeline::UPDATE_TIME_STEP())) {
        timeStep = request->Get(
            vtkStreamingDemandDrivenPipeline::UPDATE_TIME_STEP());
    }
    
    // Load data for this time step
    vtkDataObject* output = this->LoadTimeStep(timeStep);
    
    // Set time step in output information
    vtkInformation* outInfo = outputVector->GetInformationObject(0);
    outInfo->Set(vtkDataObject::DATA_TIME_STEP(), timeStep);
    
    // Set output
    outputVector->GetInformationObject(0)->Set(
        vtkDataObject::DATA_OBJECT(), output);
    
    return 1;
}
```

---

## 4. Parameter Edit Trigger (Auto-Apply)

### Auto-Apply Mode

**When Auto-Apply is Enabled**:

```
UI Layer
    │
    ├─> User edits parameter (slider, text field, etc.)
    │   └─> Widget emits: valueChanged()
    │
    └─> pqProxyWidget::propertyChanged()
        └─> If auto-apply enabled:
            └─> apply() immediately
```

**Step 1: Property Widget Change**

```cpp
// In pqDoubleSliderWidget (example property widget)
void pqDoubleSliderWidget::setValue(double value) {
    if (this->Value != value) {
        this->Value = value;
        
        // Emit signal
        emit this->valueChanged(value);
        
        // If auto-apply enabled, apply immediately
        if (this->AutoApply) {
            this->apply();
        }
    }
}
```

**Step 2: Immediate Apply**

```cpp
// In pqProxyWidget
void pqProxyWidget::propertyChanged() {
    // Check if auto-apply is enabled
    pqSettings* settings = pqApplicationCore::instance()->settings();
    bool autoApply = settings->value("autoApply", false).toBool();
    
    if (autoApply) {
        // Apply immediately
        this->apply();
    } else {
        // Just mark as modified (wait for Apply button)
        vtkSMProxy* proxy = this->getProxy();
        proxy->MarkModified(nullptr);
    }
}
```

### Complete Auto-Apply Call Stack

```
1. User edits parameter (slider, text field, etc.)
   │
   ▼
2. Property widget: valueChanged() signal
   │   └─> pqProxyWidget::propertyChanged()
   │
   ▼
3. Check auto-apply setting
   │   ├─> If enabled: apply() immediately
   │   └─> If disabled: MarkModified() only
   │
   ▼
4. pqProxyWidget::apply()
   │   ├─> Update proxy properties
   │   ├─> proxy->MarkModified(nullptr)
   │   └─> proxy->UpdatePipeline()
   │
   ▼
5. vtkSMProxy::UpdatePipeline()
   │   ├─> Update VTK object properties
   │   └─> algorithm->Update()
   │
   ▼
6. vtkAlgorithm::Update()
   │   ├─> Modified() was called (parameter changed)
   │   ├─> MTime > OutputTime (cache invalid)
   │   └─> ProcessRequest()
   │
   ▼
7. vtkAlgorithm::ProcessRequest()
   │   └─> RequestData() - recompute with new parameter
   │
   ▼
8. New Output Available
   │   ├─> Output cached with new parameter value
   │   └─> View updates automatically
```

### Performance Considerations

**Auto-Apply Trade-offs**:

```cpp
// With Auto-Apply ON:
// - Immediate feedback (good for exploration)
// - But: Expensive for large datasets
// - But: May trigger many unnecessary updates

// With Auto-Apply OFF:
// - User controls when computation occurs
// - Efficient for large datasets
// - But: Requires explicit Apply button click
```

**Optimization Strategies**:

```cpp
// Debouncing for auto-apply
void pqProxyWidget::propertyChanged() {
    if (this->AutoApply) {
        // Cancel previous timer
        this->ApplyTimer->stop();
        
        // Start new timer (debounce)
        this->ApplyTimer->start(500);  // 500ms delay
    }
}

// On timer timeout
void pqProxyWidget::onApplyTimer() {
    this->apply();  // Apply after debounce period
}
```

---

## 5. Complete Call Stack: UI to VTK

### Full Execution Path

**Layer 1: UI Layer (Qt/ParaView UI)**

```cpp
// User interaction
pqPropertiesPanel::onApply()
    │
    ├─> pqProxyWidget::apply()
    │   ├─> Update properties from widgets
    │   └─> proxy->MarkModified(nullptr)
    │
    └─> proxy->UpdateVTKObjects()
        └─> proxy->UpdatePipeline()
```

**Layer 2: Server Manager Layer**

```cpp
// vtkSMProxy (Server Manager Proxy)
vtkSMProxy::UpdatePipeline(double time = -1.0)
    │
    ├─> Get VTK algorithm
    │   └─> vtkAlgorithm* algorithm = GetClientSideObject()
    │
    ├─> Update VTK object properties
    │   └─> UpdateVTKObjects()
    │       └─> For each property:
    │           └─> property->UpdateVTKObject(algorithm)
    │
    ├─> Set time step (if temporal)
    │   └─> outInfo->Set(UPDATE_TIME_STEP(), time)
    │
    └─> Trigger VTK update
        └─> algorithm->Update()
```

**Layer 3: VTK Algorithm Layer**

```cpp
// vtkAlgorithm (VTK Pipeline)
vtkAlgorithm::Update()
    │
    ├─> Check if execution needed
    │   └─> if (!NeedToExecute()) return;  // Cache hit
    │
    ├─> Update information upstream
    │   └─> UpdateInformation()
    │       └─> ProcessRequest(REQUEST_INFORMATION)
    │
    └─> Execute pipeline
        └─> ProcessRequest(REQUEST_DATA)
```

**Layer 4: Request Processing**

```cpp
// vtkAlgorithm::ProcessRequest()
vtkAlgorithm::ProcessRequest(
    vtkInformation* request,
    vtkInformationVector** inputVector,
    vtkInformationVector* outputVector)
    │
    ├─> RequestInformation (metadata)
    │   └─> RequestInformation(request, inputVector, outputVector)
    │       ├─> Propagate upstream
    │       ├─> Determine output type
    │       └─> Set output extents
    │
    ├─> RequestUpdateExtent (extents)
    │   └─> RequestUpdateExtent(request, inputVector, outputVector)
    │       ├─> Determine required input extents
    │       └─> Set input update extents
    │
    └─> RequestData (computation)
        └─> RequestData(request, inputVector, outputVector)
            ├─> Get input data objects
            ├─> Create output data object
            ├─> Execute algorithm logic
            └─> Set output data
```

**Layer 5: Data Object Creation**

```cpp
// vtkAlgorithm::RequestData() (in filter implementation)
int vtkMyFilter::RequestData(...)
    │
    ├─> Get input
    │   └─> vtkDataObject* input = 
    │       inputVector[0]->GetInformationObject(0)->Get(DATA_OBJECT())
    │
    ├─> Get or create output
    │   └─> vtkDataObject* output = 
    │       outputVector->GetInformationObject(0)->Get(DATA_OBJECT())
    │
    ├─> Execute algorithm
    │   └─> Process input -> output
    │
    └─> Set output
        └─> outputVector->GetInformationObject(0)->Set(DATA_OBJECT(), output)
```

### Visual Call Stack Diagram

```
┌─────────────────────────────────────────────────────────────┐
│ UI Layer (Qt/ParaView)                                        │
│  ┌──────────────────────────────────────────────────────┐   │
│  │ pqPropertiesPanel::onApply()                         │   │
│  │   └─> pqProxyWidget::apply()                        │   │
│  │       └─> proxy->UpdatePipeline()                   │   │
│  └──────────────────────────────────────────────────────┘   │
└────────────────────────────┬────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────┐
│ Server Manager Layer                                         │
│  ┌──────────────────────────────────────────────────────┐   │
│  │ vtkSMProxy::UpdatePipeline()                         │   │
│  │   ├─> GetClientSideObject() → vtkAlgorithm*        │   │
│  │   ├─> UpdateVTKObjects()                            │   │
│  │   └─> algorithm->Update()                            │   │
│  └──────────────────────────────────────────────────────┘   │
└────────────────────────────┬────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────┐
│ VTK Algorithm Layer                                         │
│  ┌──────────────────────────────────────────────────────┐   │
│  │ vtkAlgorithm::Update()                              │   │
│  │   ├─> NeedToExecute() → Check cache validity        │   │
│  │   ├─> UpdateInformation() → Metadata propagation   │   │
│  │   └─> ProcessRequest() → Execute pipeline           │   │
│  └──────────────────────────────────────────────────────┘   │
└────────────────────────────┬────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────┐
│ Request Processing Layer                                     │
│  ┌──────────────────────────────────────────────────────┐   │
│  │ vtkAlgorithm::ProcessRequest()                       │   │
│  │   ├─> RequestInformation() → Metadata flow          │   │
│  │   ├─> RequestUpdateExtent() → Extent flow           │   │
│  │   └─> RequestData() → Data computation              │   │
│  └──────────────────────────────────────────────────────┘   │
└────────────────────────────┬────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────┐
│ Data Object Layer                                           │
│  ┌──────────────────────────────────────────────────────┐   │
│  │ vtkMyFilter::RequestData()                           │   │
│  │   ├─> Get input data objects                         │   │
│  │   ├─> Create output data object                      │   │
│  │   ├─> Execute algorithm logic                        │   │
│  │   └─> Set output data                                │   │
│  └──────────────────────────────────────────────────────┘   │
└─────────────────────────────────────────────────────────────┘
```

---

## 6. Cache Validity and Execution Decision

### NeedToExecute() Logic

**Cache Validity Check**:

```cpp
// In vtkAlgorithm
bool vtkAlgorithm::NeedToExecute() {
    // Check if algorithm was modified
    if (this->OutputTime < this->MTime) {
        return true;  // Algorithm parameters changed
    }
    
    // Check if any input changed
    for (int i = 0; i < this->GetNumberOfInputConnections(); i++) {
        vtkAlgorithm* input = this->GetInputAlgorithm(i);
        if (input && input->GetOutputTime() > this->OutputTime) {
            return true;  // Input changed
        }
    }
    
    // Check if request changed (e.g., time step)
    vtkInformation* outInfo = this->GetOutputInformation(0);
    if (outInfo->Has(vtkStreamingDemandDrivenPipeline::UPDATE_TIME_STEP())) {
        double requestedTime = outInfo->Get(
            vtkStreamingDemandDrivenPipeline::UPDATE_TIME_STEP());
        
        if (outInfo->Has(vtkDataObject::DATA_TIME_STEP())) {
            double cachedTime = outInfo->Get(vtkDataObject::DATA_TIME_STEP());
            if (requestedTime != cachedTime) {
                return true;  // Time step changed
            }
        } else {
            return true;  // No cached time step
        }
    }
    
    return false;  // Cache is valid
}
```

### Execution Decision Flow

```
vtkAlgorithm::Update()
    │
    ├─> NeedToExecute()?
    │   │
    │   ├─> NO → Return (cache hit, no execution)
    │   │
    │   └─> YES → Continue
    │       │
    │       ├─> UpdateInformation()
    │       │   └─> ProcessRequest(REQUEST_INFORMATION)
    │       │
    │       ├─> Update()
    │       │   └─> ProcessRequest(REQUEST_UPDATE_EXTENT)
    │       │
    │       └─> ProcessRequest(REQUEST_DATA)
    │           └─> RequestData() → Execute algorithm
    │
    └─> Update OutputTime
        └─> this->OutputTime.Modified()
```

---

## 7. Practical Examples

### Example 1: Apply Button Click

```cpp
// Complete flow for Apply button
void exampleApplyButton() {
    // 1. User clicks Apply
    //    → pqPropertiesPanel::onApply()
    
    // 2. Update properties
    vtkSMProxy* proxy = getProxy();
    proxy->UpdateVTKObjects();
    
    // 3. Trigger pipeline
    proxy->UpdatePipeline();
    //    → vtkSMProxy::UpdatePipeline()
    //       → algorithm->Update()
    //          → ProcessRequest(REQUEST_DATA)
    //             → RequestData()
    
    // 4. Data available
    vtkDataObject* output = algorithm->GetOutput();
}
```

### Example 2: Time Step Change

```cpp
// Complete flow for time step change
void exampleTimeStepChange() {
    // 1. User moves time slider
    //    → pqTimeKeeper::setTime(0.5)
    
    // 2. Update all proxies
    vtkSMProxy* proxy = getProxy();
    proxy->UpdatePipeline(0.5);
    //    → Sets UPDATE_TIME_STEP in output information
    //    → algorithm->Update()
    //       → NeedToExecute() returns true (time changed)
    //       → ProcessRequest(REQUEST_DATA)
    //          → RequestData() with time step 0.5
    
    // 3. Temporal data available
    vtkDataObject* output = algorithm->GetOutput();
    double timeStep = output->GetInformation()->Get(
        vtkDataObject::DATA_TIME_STEP());
    // timeStep == 0.5
}
```

### Example 3: Auto-Apply Parameter Edit

```cpp
// Complete flow for auto-apply
void exampleAutoApply() {
    // 1. User edits parameter (slider)
    //    → pqDoubleSliderWidget::setValue(2.0)
    //       → emit valueChanged(2.0)
    
    // 2. Property changed
    //    → pqProxyWidget::propertyChanged()
    //       → Check auto-apply: enabled
    //       → apply() immediately
    
    // 3. Update and execute
    vtkSMProxy* proxy = getProxy();
    proxy->UpdateVTKObjects();
    proxy->UpdatePipeline();
    //    → algorithm->Modified() was called
    //    → algorithm->Update()
    //       → NeedToExecute() returns true (MTime > OutputTime)
    //       → ProcessRequest(REQUEST_DATA)
    //          → RequestData() with new parameter
    
    // 4. New output available
    vtkDataObject* output = algorithm->GetOutput();
    // Output reflects new parameter value
}
```

---

## 8. Summary: Recomputation Triggers

### Trigger Mechanisms

1. **Apply Button**:
   - Explicit user control
   - `pqPropertiesPanel::onApply()` → `vtkSMProxy::UpdatePipeline()`
   - Most common for manual workflows

2. **Time Step Changes**:
   - Temporal data navigation
   - `pqTimeKeeper::setTime()` → `vtkSMProxy::UpdatePipeline(time)`
   - Essential for time-varying data

3. **Parameter Edits (Auto-Apply)**:
   - Immediate feedback when enabled
   - `pqProxyWidget::propertyChanged()` → `apply()` → `UpdatePipeline()`
   - Useful for interactive exploration

4. **View Updates**:
   - Automatic background updates
   - Render views trigger pipeline when needed
   - Lazy evaluation for visualization

### Call Stack Summary

**UI → Server Manager → VTK → Data**:
1. **UI Layer**: User interaction, property widgets
2. **Server Manager**: Proxy management, property updates
3. **VTK Algorithm**: Pipeline execution, request processing
4. **Data Object**: Data creation and caching

### Key Methods

- `pqPropertiesPanel::onApply()` - Apply button handler
- `pqTimeKeeper::setTime()` - Time step change handler
- `pqProxyWidget::apply()` - Property update handler
- `vtkSMProxy::UpdatePipeline()` - Server Manager pipeline trigger
- `vtkAlgorithm::Update()` - VTK pipeline execution
- `vtkAlgorithm::ProcessRequest()` - Request processing
- `vtkAlgorithm::RequestData()` - Actual computation

### Best Practices

1. **Use Apply Button** for large datasets or when user control is needed
2. **Enable Auto-Apply** only for small datasets and interactive exploration
3. **Understand Cache Validity** to optimize performance
4. **Handle Time Steps** properly for temporal data
5. **Respect Lazy Evaluation** - don't force unnecessary updates

This architecture enables ParaView to efficiently handle user interactions while maintaining performance through lazy evaluation and intelligent caching.

