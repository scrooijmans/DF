# ParaView 2D Chart Creation and Management

This document explains how ParaView adds a basic 2D chart to the UI when a user triggers a button to add a chart, the inheritance structure used, and how chart interactions (adding and deleting charts) are managed.

## Overview

ParaView's 2D chart creation follows a **multi-layer architecture** that separates UI interactions, Server Manager proxies, and VTK rendering:

1. **UI Layer**: Button clicks trigger reactions that create chart views
2. **Server Manager Layer**: Proxies manage chart view state and configuration
3. **VTK Layer**: Chart views render data using VTK's charting infrastructure
4. **View Management**: Centralized system manages chart lifecycle (creation, deletion, updates)

**Key Principles:**
- **Proxy Pattern**: Chart views are managed through Server Manager proxies
- **Inheritance Hierarchy**: Chart views inherit from context views and base view classes
- **Reaction Pattern**: UI buttons trigger reactions that create/manage views
- **Observer Pattern**: Views observe data changes and update automatically

---

## 1. Chart Creation Flow: Button Click to Chart Display

### High-Level Flow

```
User Clicks "Add Chart" Button
    ↓
pqReaction (e.g., pqCreateViewReaction)
    ↓
CreateView("XYChartView") or CreateXYPlotView()
    ↓
vtkSMProxyManager::NewProxy("views", "XYChartView")
    ↓
vtkSMXYChartViewProxy Created
    ↓
pqChartView Widget Created (via pqViewManager)
    ↓
vtkPVXYChartView Initialized
    ↓
Chart Widget Added to UI Layout
    ↓
Chart Ready for Data Display
```

### Step-by-Step Process

#### Step 1: User Triggers Button

**UI Button Location**: ParaView toolbar or menu (e.g., "View" → "Create Chart View")

**Button Action**: Connected to a reaction class that handles view creation:

```cpp
// Example: Button connected to pqCreateViewReaction
class pqCreateViewReaction : public pqReaction
{
public:
    pqCreateViewReaction(QAction* parentAction, const QString& viewType);
    
protected:
    void onTriggered() override
    {
        // Create view via Server Manager
        vtkSMViewProxy* viewProxy = 
            vtkSMProxyManager::GetProxyManager()->NewProxy(
                "views", this->ViewType.toUtf8().data());
        
        if (viewProxy)
        {
            viewProxy->UpdateVTKObjects();
            
            // Register with view manager
            pqViewManager* viewManager = 
                pqApplicationCore::instance()->getViewManager();
            viewManager->createView(viewProxy);
        }
    }
    
private:
    QString ViewType; // e.g., "XYChartView"
};
```

#### Step 2: Server Manager Creates View Proxy

**Proxy Creation**:

```cpp
// vtkSMProxyManager creates proxy from XML definition
vtkSMProxy* viewProxy = vtkSMProxyManager::GetProxyManager()->NewProxy(
    "views",           // Group name
    "XYChartView"      // Proxy type name
);

// Proxy is configured from XML:
// <ServerManagerConfiguration>
//   <ProxyGroup name="views">
//     <Proxy name="XYChartView" 
//            class="vtkSMXYChartViewProxy"
//            base_proxygroup="views" 
//            base_proxyname="ContextView">
//       ...
//     </Proxy>
//   </ProxyGroup>
// </ServerManagerConfiguration>
```

**Proxy Hierarchy**:
- `vtkSMXYChartViewProxy` inherits from `vtkSMContextViewProxy`
- `vtkSMContextViewProxy` inherits from `vtkSMViewProxy`
- `vtkSMViewProxy` inherits from `vtkSMProxy`

#### Step 3: pqViewManager Creates UI Widget

**View Manager Registration**:

```cpp
// pqViewManager creates pqView wrapper
class pqViewManager
{
public:
    pqView* createView(vtkSMViewProxy* viewProxy)
    {
        // Determine view type and create appropriate pqView subclass
        QString viewType = viewProxy->GetXMLName();
        
        pqView* view = nullptr;
        if (viewType == "XYChartView")
        {
            view = new pqChartView(viewProxy);
        }
        // ... other view types
        
        // Register view
        this->registerView(view);
        
        // Create widget
        QWidget* widget = view->createWidget();
        
        // Add to layout
        this->addViewToLayout(view, widget);
        
        return view;
    }
};
```

#### Step 4: pqChartView Creates Widget

**Widget Creation**:

```cpp
class pqChartView : public pqContextView
{
public:
    pqChartView(const QString& type, 
                const QString& group, 
                const QString& name,
                vtkSMViewProxy* viewProxy,
                pqServer* server,
                QObject* parent = nullptr)
        : pqContextView(type, group, name, viewProxy, server, parent)
    {
    }
    
protected:
    QWidget* createWidget() override
    {
        // Get VTK context view
        vtkContextView* contextView = this->getVTKContextView();
        
        // Create Qt widget wrapper
        QVTKOpenGLNativeWidget* widget = 
            new QVTKOpenGLNativeWidget();
        
        // Set render window
        widget->setRenderWindow(contextView->GetRenderWindow());
        
        return widget;
    }
};
```

#### Step 5: VTK Chart View Initialization

**VTK Chart View Setup**:

```cpp
// vtkSMXYChartViewProxy creates vtkPVXYChartView
class vtkSMXYChartViewProxy : public vtkSMContextViewProxy
{
protected:
    void CreateVTKObjects() override
    {
        // Create VTK chart view
        vtkPVXYChartView* chartView = vtkPVXYChartView::New();
        
        // Get chart from view
        vtkChart* chart = chartView->GetChart();
        
        // Configure chart (axes, title, etc.)
        this->SetupChart(chart);
        
        // Set on context view
        vtkSMContextViewProxy::SetContextView(chartView);
    }
};
```

---

## 2. Inheritance Structure

### Complete Inheritance Hierarchy

#### VTK Layer (Server-Side)

```
vtkObject
    └── vtkView
        └── vtkPVView
            └── vtkPVContextView
                └── vtkPVXYChartView
```

**Class Descriptions**:

1. **vtkObject**: Base VTK object with reference counting, events, and type information
2. **vtkView**: Abstract base class for all views in VTK
   - Manages representations (data visualizations)
   - Handles view updates and rendering
   - Provides view theme support
3. **vtkPVView**: ParaView-specific view base class
   - Adds Server Manager integration
   - Manages view layout and sizing
   - Handles multi-process rendering
4. **vtkPVContextView**: Base class for context-based views (charts, plots)
   - Manages vtkContextView (VTK's 2D rendering context)
   - Provides chart/plot infrastructure
   - Handles 2D interaction and selection
5. **vtkPVXYChartView**: Specific implementation for XY charts
   - Manages vtkChartXY (VTK's XY chart class)
   - Provides axis configuration
   - Handles chart-specific rendering

#### Server Manager Layer (Proxy)

```
vtkSMProxy
    └── vtkSMViewProxy
        └── vtkSMContextViewProxy
            └── vtkSMXYChartViewProxy
```

**Class Descriptions**:

1. **vtkSMProxy**: Base proxy class
   - Manages VTK object lifecycle
   - Handles property management
   - Provides client-server communication
2. **vtkSMViewProxy**: Base class for view proxies
   - Manages view-specific properties
   - Handles view layout and positioning
   - Provides view update mechanisms
3. **vtkSMContextViewProxy**: Base class for context view proxies
   - Manages context view properties
   - Handles chart/plot configuration
   - Provides 2D view-specific functionality
4. **vtkSMXYChartViewProxy**: Specific proxy for XY chart views
   - Manages chart-specific properties (axes, titles, etc.)
   - Handles chart representation management
   - Provides chart export functionality

#### UI Layer (Client-Side)

```
QObject
    └── pqServerManagerModelItem
        └── pqProxy
            └── pqView
                └── pqContextView
                    └── pqChartView
```

**Class Descriptions**:

1. **QObject**: Qt base class for objects with signals/slots
2. **pqServerManagerModelItem**: Base class for Server Manager model items
   - Provides name and modification state management
   - Emits signals for state changes
3. **pqProxy**: Base class for proxy wrappers
   - Wraps vtkSMProxy objects
   - Provides Qt-friendly interface
   - Manages helper proxies
4. **pqView**: Base class for all ParaView views
   - Manages view widget creation
   - Handles view selection and activation
   - Provides view-specific UI actions
5. **pqContextView**: Base class for context views
   - Manages context view-specific UI
   - Handles chart/plot interactions
   - Provides selection support
6. **pqChartView**: Specific implementation for chart views
   - Manages chart-specific UI elements
   - Handles chart property widgets
   - Provides chart export actions

### Key Methods in Inheritance Chain

**vtkPVXYChartView**:
```cpp
class vtkPVXYChartView : public vtkPVContextView
{
public:
    vtkChart* GetChart();
    
    // Axis configuration methods (generated via macros)
    void SetXAxisTitle(const char* title);
    void SetYAxisTitle(const char* title);
    void SetXAxisLogScale(bool log);
    void SetYAxisLogScale(bool log);
    // ... many more axis configuration methods
};
```

**pqChartView**:
```cpp
class pqChartView : public pqContextView
{
public:
    // Inherited from pqView
    QWidget* createWidget() override;
    
    // Inherited from pqContextView
    vtkSMContextViewProxy* getContextViewProxy() const override;
    vtkContextView* getVTKContextView() const override;
    
    // Chart-specific methods
    void resetDisplay(bool closest = false) override;
    bool supportsCapture() const override { return true; }
};
```

---

## 3. Chart Management: Adding and Deleting Charts

### View Manager Architecture

**Centralized View Management**:

```cpp
class pqViewManager : public QObject
{
    Q_OBJECT
    
public:
    // Create new view
    pqView* createView(vtkSMViewProxy* viewProxy);
    
    // Get all views
    QList<pqView*> getViews() const;
    
    // Get active view
    pqView* getActiveView() const;
    
    // Remove view
    void removeView(pqView* view);
    
signals:
    void viewCreated(pqView* view);
    void viewRemoved(pqView* view);
    void activeViewChanged(pqView* view);
    
private:
    QList<pqView*> Views;
    pqView* ActiveView = nullptr;
};
```

### Adding Charts

#### Method 1: Via Python API

```python
import paraview.simple as pv

# Create XY plot chart view
chartView = pv.CreateXYPlotView()

# Or using generic CreateView
chartView = pv.CreateView("XYChartView")

# Configure chart properties
chartView.XAxisTitle = "Time"
chartView.YAxisTitle = "Value"
chartView.XAxisLogScale = False
chartView.YAxisLogScale = False

# Show data in chart
source = pv.Sphere()
plot = pv.PlotOnActiveView(source)
pv.Show(plot, chartView)
pv.Render(chartView)
```

#### Method 2: Via UI Button

**Button Action Flow**:

```cpp
// 1. Button in UI triggers action
QAction* createChartAction = new QAction("Create Chart View", this);
connect(createChartAction, &QAction::triggered, 
        this, &MainWindow::onCreateChartView);

// 2. Action handler creates view
void MainWindow::onCreateChartView()
{
    // Get view manager
    pqViewManager* viewManager = 
        pqApplicationCore::instance()->getViewManager();
    
    // Create view proxy
    vtkSMProxyManager* proxyManager = 
        vtkSMProxyManager::GetProxyManager();
    vtkSMViewProxy* viewProxy = 
        proxyManager->NewProxy("views", "XYChartView");
    
    // Initialize proxy
    viewProxy->UpdateVTKObjects();
    
    // Create view via view manager
    pqView* view = viewManager->createView(viewProxy);
    
    // Activate view
    viewManager->setActiveView(view);
}
```

#### Method 3: Via Reaction Class

**Reaction Pattern**:

```cpp
class pqCreateChartViewReaction : public pqReaction
{
    Q_OBJECT
    
public:
    pqCreateChartViewReaction(QAction* parentAction)
        : pqReaction(parentAction)
    {
    }
    
protected:
    void onTriggered() override
    {
        // Get view manager
        pqViewManager* viewManager = 
            pqApplicationCore::instance()->getViewManager();
        
        // Create view proxy
        vtkSMProxyManager* proxyManager = 
            vtkSMProxyManager::GetProxyManager();
        vtkSMViewProxy* viewProxy = 
            proxyManager->NewProxy("views", "XYChartView");
        
        if (viewProxy)
        {
            viewProxy->UpdateVTKObjects();
            
            // Create view
            pqView* view = viewManager->createView(viewProxy);
            
            // Activate
            viewManager->setActiveView(view);
        }
    }
};
```

### Deleting Charts

#### Method 1: Via Python API

```python
import paraview.simple as pv

# Get active view
activeView = pv.GetActiveView()

# Delete view
pv.Delete(activeView)

# Or delete specific view
chartView = pv.FindView("XYChartView1")
if chartView:
    pv.Delete(chartView)
```

#### Method 2: Via UI (Close Button)

**Close Button Flow**:

```cpp
// View frame has close button
class pqViewFrame : public QFrame
{
    Q_OBJECT
    
public slots:
    void closeView()
    {
        pqView* view = this->getView();
        if (view)
        {
            // Get view manager
            pqViewManager* viewManager = 
                pqApplicationCore::instance()->getViewManager();
            
            // Remove view
            viewManager->removeView(view);
        }
    }
};
```

#### Method 3: Via Delete Reaction

**Delete Reaction**:

```cpp
class pqDeleteReaction : public pqReaction
{
    Q_OBJECT
    
public:
    enum DeleteModes {
        SELECTED,  // Delete selected items
        ALL,       // Delete all items
        TREE       // Delete entire tree
    };
    
    pqDeleteReaction(DeleteModes mode, QAction* parentAction)
        : pqReaction(parentAction), Mode(mode)
    {
    }
    
protected:
    void onTriggered() override
    {
        pqViewManager* viewManager = 
            pqApplicationCore::instance()->getViewManager();
        
        pqView* activeView = viewManager->getActiveView();
        
        if (activeView && this->Mode == SELECTED)
        {
            // Delete active view
            viewManager->removeView(activeView);
        }
    }
    
private:
    DeleteModes Mode;
};
```

### View Removal Process

**Complete Removal Flow**:

```cpp
void pqViewManager::removeView(pqView* view)
{
    if (!view || !this->Views.contains(view))
        return;
    
    // 1. Emit signal before removal
    emit this->viewRemoved(view);
    
    // 2. Remove from active view if needed
    if (this->ActiveView == view)
    {
        this->ActiveView = nullptr;
        emit this->activeViewChanged(nullptr);
    }
    
    // 3. Get view proxy
    vtkSMViewProxy* viewProxy = view->getViewProxy();
    
    // 4. Remove from layout
    this->removeViewFromLayout(view);
    
    // 5. Delete widget
    QWidget* widget = view->widget();
    if (widget)
    {
        widget->deleteLater();
    }
    
    // 6. Remove from list
    this->Views.removeAll(view);
    
    // 7. Delete proxy
    if (viewProxy)
    {
        viewProxy->Delete();
    }
    
    // 8. Delete view object
    view->deleteLater();
}
```

---

## 4. Chart Interactions and Updates

### Adding Data to Charts

**Representation Creation**:

```cpp
// When user selects data source and chooses "Plot Selection"
class pqPlotSelectionReaction : public pqReaction
{
protected:
    void onTriggered() override
    {
        // Get active source
        pqOutputPort* port = pqActiveObjects::instance().activePort();
        if (!port)
            return;
        
        // Get active view (must be chart view)
        pqView* view = pqActiveObjects::instance().activeView();
        pqChartView* chartView = qobject_cast<pqChartView*>(view);
        if (!chartView)
            return;
        
        // Create representation
        vtkSMProxy* representationProxy = 
            vtkSMParaViewPipelineControllerWithRendering::NewRepresentation(
                port->getSource()->getProxy(),
                chartView->getViewProxy());
        
        // Show representation
        vtkSMPropertyHelper(representationProxy, "Visibility").Set(1);
        representationProxy->UpdateVTKObjects();
        
        // Render
        chartView->render();
    }
};
```

### Chart Update Mechanism

**Observer Pattern for Updates**:

```cpp
// Chart view observes data changes
class pqChartView : public pqContextView
{
protected:
    void setupRepresentation(pqRepresentation* repr) override
    {
        // Connect representation signals
        connect(repr, &pqRepresentation::dataUpdated,
                this, &pqChartView::onDataUpdated);
        
        // Connect proxy property changes
        vtkSMProxy* proxy = repr->getProxy();
        proxy->AddObserver(vtkCommand::PropertyModifiedEvent,
                          this, &pqChartView::onPropertyModified);
    }
    
private slots:
    void onDataUpdated()
    {
        // Data changed, update chart
        this->render();
    }
    
    void onPropertyModified(vtkObject* caller, 
                            unsigned long event,
                            void* callData)
    {
        // Property changed, update chart
        this->render();
    }
};
```

### Multiple Chart Management

**Tabbed View Widget**:

```cpp
class pqTabbedMultiViewWidget : public QWidget
{
    Q_OBJECT
    
public:
    // Create new tab with chart
    void createTab(pqServer* server)
    {
        // Create layout for tab
        vtkSMViewLayoutProxy* layout = 
            vtkSMViewLayoutProxy::New();
        
        // Create chart view in layout
        vtkSMViewProxy* chartView = 
            this->createViewInLayout("XYChartView", layout);
        
        // Add tab
        QWidget* tabWidget = this->createTabWidget(layout);
        this->addTab(tabWidget, "Chart View");
    }
    
    // Switch between charts
    void setActiveTab(int index)
    {
        QWidget* tabWidget = this->widget(index);
        if (tabWidget)
        {
            // Get view from tab
            pqView* view = this->getViewFromTab(tabWidget);
            if (view)
            {
                // Activate view
                pqViewManager* viewManager = 
                    pqApplicationCore::instance()->getViewManager();
                viewManager->setActiveView(view);
            }
        }
    }
};
```

---

## 5. Key Design Patterns

### Proxy Pattern

**Separation of Client and Server**:

- **Client Side**: `pqChartView` (Qt widget, UI interactions)
- **Server Side**: `vtkPVXYChartView` (VTK object, rendering)
- **Bridge**: `vtkSMXYChartViewProxy` (Server Manager proxy)

This pattern enables:
- Remote visualization (client-server separation)
- State management (proxy holds configuration)
- Undo/redo (proxy state can be saved/restored)

### Observer Pattern

**Event-Driven Updates**:

- Views observe proxy property changes
- Representations observe data updates
- UI observes view state changes

This ensures:
- Automatic synchronization
- Efficient updates (only changed components update)
- Loose coupling between components

### Factory Pattern

**View Creation**:

- `pqViewManager` acts as factory
- Creates appropriate `pqView` subclass based on proxy type
- XML registration determines available view types

### Reaction Pattern

**UI Action Handling**:

- `pqReaction` base class for UI actions
- Subclasses handle specific actions (create, delete, etc.)
- Actions can be enabled/disabled based on context

---

## 6. Summary

ParaView's 2D chart creation and management system uses:

1. **Multi-Layer Architecture**:
   - UI Layer: Qt widgets and reactions
   - Server Manager Layer: Proxies for state management
   - VTK Layer: Chart rendering and data visualization

2. **Inheritance Hierarchy**:
   - VTK: `vtkObject` → `vtkView` → `vtkPVView` → `vtkPVContextView` → `vtkPVXYChartView`
   - Server Manager: `vtkSMProxy` → `vtkSMViewProxy` → `vtkSMContextViewProxy` → `vtkSMXYChartViewProxy`
   - UI: `QObject` → `pqServerManagerModelItem` → `pqProxy` → `pqView` → `pqContextView` → `pqChartView`

3. **Chart Management**:
   - **Adding**: Via Python API, UI buttons, or reaction classes
   - **Deleting**: Via close buttons, delete reactions, or Python API
   - **Updates**: Automatic via observer pattern when data or properties change

4. **Key Features**:
   - Centralized view management through `pqViewManager`
   - Proxy-based state management
   - Event-driven updates
   - Support for multiple charts in tabbed interface

This architecture provides a robust, extensible system for creating and managing 2D charts in ParaView while maintaining separation of concerns and enabling remote visualization capabilities.


