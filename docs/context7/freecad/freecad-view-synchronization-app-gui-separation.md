# FreeCAD View Synchronization: App/Gui Separation and Observer Patterns

This document explains in detail how FreeCAD synchronizes multiple views (3D view, tree view, property editor) through App/Gui separation and observer patterns.

## Overview

FreeCAD uses a **Model-View-Controller (MVC) architecture** with strict separation between application logic (`App`) and graphical interface (`Gui`). This separation, combined with observer patterns and signal/slot mechanisms, ensures that all views remain synchronized when the underlying data changes.

---

## 1. App/Gui Separation Architecture

### Core Principle

FreeCAD separates concerns into two distinct modules:

- **`App` Module**: Core application logic, data model, document management
- **`Gui` Module**: Graphical user interface, visual representation, user interaction

This separation enables:
- **Headless Operation**: FreeCAD can run without GUI (scripting, automation)
- **Multiple Views**: Multiple GUI views can observe the same App data
- **Testability**: App logic can be tested without GUI dependencies
- **Flexibility**: GUI can be replaced or extended without changing App logic

### Architecture Diagram

```
┌─────────────────────────────────────────────────────────┐
│                    App Module                            │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐ │
│  │   Document   │  │ DocumentObject│  │   Property   │ │
│  │              │  │              │  │              │ │
│  │ - Objects    │  │ - Properties │  │ - Type       │ │
│  │ - Signals    │  │ - Signals    │  │ - Value      │ │
│  └──────┬───────┘  └──────┬───────┘  └──────┬───────┘ │
│         │                  │                  │         │
│         └──────────────────┼──────────────────┘         │
│                            │                            │
│                    Signal Emission                      │
└────────────────────────────┼────────────────────────────┘
                             │
                             │ Signals (Boost.Signals2)
                             │
┌────────────────────────────┼────────────────────────────┐
│                    Gui Module                            │
│                            │                            │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐ │
│  │ ViewProvider │  │  Tree View    │  │ Property      │ │
│  │             │  │              │  │ Editor        │ │
│  │ - 3D View   │  │ - Object List│  │ - Properties  │ │
│  │ - Coin3D    │  │ - Selection  │  │ - Values      │ │
│  └──────┬───────┘  └──────┬───────┘  └──────┬───────┘ │
│         │                  │                  │         │
│         └──────────────────┼──────────────────┘         │
│                            │                            │
│                    Signal Observers                     │
└─────────────────────────────────────────────────────────┘
```

### DocumentObject and ViewProvider Pairing

Every `App::DocumentObject` has a corresponding `Gui::ViewProvider`:

```python
# App side: DocumentObject (data model)
box = doc.addObject("Part::Box", "Box")
# - Stores properties: Length, Width, Height
# - Emits signals when properties change
# - No knowledge of GUI

# Gui side: ViewProvider (visual representation)
if App.GuiUp:
    view_provider = box.ViewObject  # Gui::ViewProviderBox
    # - Listens to box signals
    # - Updates 3D view when box changes
    # - Handles user interaction
```

**Key Point**: The `App` object doesn't know about its `ViewProvider`, but the `ViewProvider` observes the `App` object.

---

## 2. Signal/Slot System

### Boost.Signals2 Foundation

FreeCAD uses **Boost.Signals2** for the signal/slot mechanism, providing:

- **Type-Safe Signals**: Signals are typed (know their parameter types)
- **Multiple Observers**: Multiple slots can connect to one signal
- **Thread Safety**: Signals can be emitted from different threads
- **Automatic Disconnection**: Slots are automatically disconnected when objects are destroyed

### Document Signals

The `App::Document` class emits signals for document-level events:

```cpp
// C++ Signal Definitions (simplified)
class Document {
    // Signal: New object added
    boost::signals2::signal<void (DocumentObject*)> signalNewObject;
    
    // Signal: Object deleted
    boost::signals2::signal<void (DocumentObject*)> signalDeletedObject;
    
    // Signal: Object changed (property modified)
    boost::signals2::signal<void (const DocumentObject&, const Property&)> signalChangedObject;
    
    // Signal: Document recomputed
    boost::signals2::signal<void (const Document&)> signalRecomputed;
    
    // Signal: Object renamed
    boost::signals2::signal<void (DocumentObject*)> signalRenamedObject;
};
```

### DocumentObject Signals

Each `DocumentObject` emits signals for object-level events:

```cpp
// C++ Signal Definitions (simplified)
class DocumentObject {
    // Signal: Property changed
    boost::signals2::signal<void (const Property&)> signalChanged;
    
    // Signal: Object touched (needs recomputation)
    boost::signals2::signal<void ()> signalTouched;
    
    // Signal: Object recomputed
    boost::signals2::signal<void ()> signalRecomputed;
};
```

---

## 3. Observer Pattern Implementation

### DocumentObserver

FreeCAD provides `App::DocumentObserver` for observing document events:

```cpp
// C++ DocumentObserver (simplified)
class DocumentObserver {
public:
    // Virtual methods called when signals are emitted
    virtual void slotNewObject(const DocumentObject& obj);
    virtual void slotDeletedObject(const DocumentObject& obj);
    virtual void slotChangedObject(const DocumentObject& obj, const Property& prop);
    virtual void slotRecomputed(const Document& doc);
    
    // Attach to document
    void attachDocument(Document* doc);
    void detachDocument();
};
```

**Python DocumentObserver**:
```python
import FreeCAD as App

class MyDocumentObserver:
    def slotNewObject(self, obj):
        print(f"New object: {obj.Name}")
    
    def slotDeletedObject(self, obj):
        print(f"Deleted object: {obj.Name}")
    
    def slotChangedObject(self, obj, prop):
        print(f"Object {obj.Name} property {prop.Name} changed")
    
    def slotRecomputed(self, doc):
        print(f"Document {doc.Name} recomputed")

# Attach observer
observer = MyDocumentObserver()
doc = App.ActiveDocument
doc.addObserver(observer)
```

### ViewProvider as Observer

`ViewProvider` objects observe their corresponding `DocumentObject`:

```cpp
// C++ ViewProvider (simplified)
class ViewProvider {
    DocumentObject* pcObject;  // Observed object
    
    // Connect to object signals
    void attach(DocumentObject* obj) {
        pcObject = obj;
        
        // Connect to object signals
        obj->signalChanged.connect(
            boost::bind(&ViewProvider::updateData, this, _1)
        );
        obj->signalTouched.connect(
            boost::bind(&ViewProvider::onUpdate, this)
        );
    }
    
    // Called when object property changes
    virtual void updateData(const Property* prop);
    
    // Called when object is touched
    virtual void onUpdate();
};
```

---

## 4. View Synchronization Flow

### Scenario: Property Change Synchronization

When a property changes, all views are synchronized:

```
User changes property in Property Editor
    ↓
App::DocumentObject property setter
    ↓
signalChanged(property) emitted
    ↓
┌─────────────────────────────────────┐
│  Multiple Observers Receive Signal │
└─────────────────────────────────────┘
    │
    ├─→ ViewProvider.updateData()
    │   └─→ Updates 3D view (Coin3D scene graph)
    │
    ├─→ Tree View Observer
    │   └─→ Updates tree view item (if needed)
    │
    └─→ Property Editor Observer
        └─→ Updates property editor display
```

### Detailed Flow: Property Change

#### Step 1: Property Assignment

```python
# User changes property in Property Editor
box.Length = 20  # Or via script
```

#### Step 2: Signal Emission

```cpp
// C++ Property Setter (simplified)
void PropertyContainer::setPropertyValue(const char* name, PyObject* value) {
    Property* prop = getPropertyByName(name);
    prop->setPyObject(value);
    
    // Emit signal
    signalChanged(*prop);  // ← Signal emitted
}
```

#### Step 3: ViewProvider Receives Signal

```cpp
// ViewProvider::updateData() called via signal
void ViewProvider::updateData(const Property* prop) {
    if (prop == &pcObject->Length || 
        prop == &pcObject->Width || 
        prop == &pcObject->Height) {
        // Geometric property changed - update 3D view
        updateVisual();  // Update Coin3D representation
    }
}
```

#### Step 4: 3D View Update

```cpp
// ViewProvider updates Coin3D scene graph
void ViewProvider::updateVisual() {
    // Get updated shape from object
    TopoDS_Shape shape = pcObject->Shape.getValue();
    
    // Update Coin3D node
    SoShape* shapeNode = getShapeNode();
    shapeNode->shape.setValue(convertToCoin3D(shape));
    
    // Request redraw
    getViewer()->redraw();
}
```

#### Step 5: Property Editor Update

The property editor observes the same signals:

```cpp
// Property Editor Observer
class PropertyEditorObserver {
    void slotChangedObject(const DocumentObject& obj, const Property& prop) {
        if (obj == selectedObject) {
            // Update property editor display
            updatePropertyDisplay(prop);
        }
    }
};
```

### Scenario: Object Addition Synchronization

When a new object is added:

```
doc.addObject("Part::Box", "Box")
    ↓
Document::addObject()
    ↓
signalNewObject(box) emitted
    ↓
┌─────────────────────────────────────┐
│  Multiple Observers Receive Signal │
└─────────────────────────────────────┘
    │
    ├─→ ViewProvider created
    │   └─→ Object appears in 3D view
    │
    ├─→ Tree View Observer
    │   └─→ New item added to tree
    │
    └─→ Selection Observer
        └─→ (If auto-selected) Updates selection
```

---

## 5. Tree View Synchronization

### Tree View Observer

The tree view observes document signals to stay synchronized:

```cpp
// Tree View Observer (simplified)
class TreeViewObserver : public DocumentObserver {
    QTreeWidget* treeWidget;
    
    void slotNewObject(const DocumentObject& obj) {
        // Add new item to tree
        QTreeWidgetItem* item = new QTreeWidgetItem();
        item->setText(0, obj.Label.getValue());
        item->setData(0, Qt::UserRole, obj.getID());
        treeWidget->addTopLevelItem(item);
    }
    
    void slotDeletedObject(const DocumentObject& obj) {
        // Remove item from tree
        QTreeWidgetItem* item = findItemByObject(obj);
        if (item) {
            delete item;
        }
    }
    
    void slotChangedObject(const DocumentObject& obj, const Property& prop) {
        if (prop == &obj.Label) {
            // Update tree item label
            QTreeWidgetItem* item = findItemByObject(obj);
            if (item) {
                item->setText(0, obj.Label.getValue());
            }
        }
    }
    
    void slotRenamedObject(DocumentObject* obj) {
        // Update tree item name
        QTreeWidgetItem* item = findItemByObject(*obj);
        if (item) {
            item->setText(0, obj->Label.getValue());
        }
    }
};
```

### Tree View Selection Synchronization

When user selects an object in tree view:

```
User clicks tree item
    ↓
Tree View Selection Changed
    ↓
Gui.Selection.setSelection([object])
    ↓
Selection signal emitted
    ↓
┌─────────────────────────────────────┐
│  Multiple Observers Receive Signal │
└─────────────────────────────────────┘
    │
    ├─→ 3D View
    │   └─→ Highlights object in 3D view
    │
    └─→ Property Editor
        └─→ Displays object properties
```

---

## 6. Property Editor Synchronization

### Property Editor Observer

The property editor observes selection and property changes:

```cpp
// Property Editor Observer (simplified)
class PropertyEditorObserver : public SelectionObserver {
    QWidget* propertyWidget;
    DocumentObject* currentObject;
    
    // Selection changed
    void onSelectionChanged(const SelectionChanges& msg) {
        if (msg.Type == SelectionChanges::AddSelection) {
            // Object selected - display its properties
            currentObject = msg.pObject;
            displayProperties(currentObject);
        } else if (msg.Type == SelectionChanges::ClrSelection) {
            // Selection cleared - clear property display
            currentObject = nullptr;
            clearProperties();
        }
    }
    
    // Property changed
    void slotChangedObject(const DocumentObject& obj, const Property& prop) {
        if (&obj == currentObject) {
            // Update property value in editor
            updatePropertyValue(prop);
        }
    }
    
    void displayProperties(DocumentObject* obj) {
        // Display all properties of selected object
        for (const char* propName : obj->getPropertyNames()) {
            Property* prop = obj->getPropertyByName(propName);
            addPropertyWidget(prop);
        }
    }
};
```

### Property Editor Update Flow

```
User edits property in Property Editor
    ↓
Property Editor sets property value
    ↓
obj.Property = newValue
    ↓
App::DocumentObject property setter
    ↓
signalChanged(property) emitted
    ↓
┌─────────────────────────────────────┐
│  Multiple Observers Receive Signal │
└─────────────────────────────────────┘
    │
    ├─→ ViewProvider.updateData()
    │   └─→ 3D view updates
    │
    ├─→ Tree View (if Label changed)
    │   └─→ Tree item updates
    │
    └─→ Property Editor (self-update)
        └─→ Property value display updates
```

---

## 7. 3D View Synchronization

### ViewProvider Update Methods

`ViewProvider` has multiple update methods for different scenarios:

```cpp
// ViewProvider update methods
class ViewProvider {
    // Called when object property changes
    virtual void updateData(const Property* prop);
    
    // Called when object is touched (needs recomputation)
    virtual void onUpdate();
    
    // Called when object is recomputed
    virtual void onChanged(const Property* prop);
    
    // Called when object visibility changes
    virtual void show();
    virtual void hide();
    
    // Called when object selection changes
    virtual void highlight(bool on);
};
```

### 3D View Update Flow

```
Property change → signalChanged()
    ↓
ViewProvider.updateData(property)
    ↓
Check property type
    ↓
┌─────────────────────────────────────┐
│  Update Appropriate Visual Element  │
└─────────────────────────────────────┘
    │
    ├─→ Shape property → Update geometry
    ├─→ Placement property → Update position
    ├─→ Visibility property → Show/hide
    ├─→ Color property → Update color
    └─→ Transparency property → Update transparency
    ↓
Update Coin3D scene graph
    ↓
Request redraw
    ↓
3D view refreshes
```

### Coin3D Scene Graph Update

```cpp
// ViewProvider updates Coin3D (simplified)
void ViewProvider::updateVisual() {
    // Get root separator
    SoSeparator* root = getRootNode();
    
    // Update shape node
    if (pcObject->isDerivedFrom("Part::Feature")) {
        Part::Feature* feature = static_cast<Part::Feature*>(pcObject);
        TopoDS_Shape shape = feature->Shape.getValue();
        
        SoShape* shapeNode = getShapeNode();
        shapeNode->shape.setValue(convertToCoin3D(shape));
    }
    
    // Update placement node
    SoTransform* transform = getTransformNode();
    Base::Placement placement = pcObject->Placement.getValue();
    transform->translation.setValue(
        placement.getPosition().x,
        placement.getPosition().y,
        placement.getPosition().z
    );
    
    // Touch scene graph (trigger update)
    root->touch();
    
    // Request viewer redraw
    getViewer()->redraw();
}
```

---

## 8. Selection Synchronization

### SelectionObserver

FreeCAD provides `Gui::SelectionObserver` for observing selection changes:

```cpp
// SelectionObserver (simplified)
class SelectionObserver {
    // Called when selection changes
    virtual void onSelectionChanged(const SelectionChanges& msg);
    
    // Attach to selection
    void attachSelection();
    void detachSelection();
};
```

**Python SelectionObserver**:
```python
import FreeCAD as App
import FreeCADGui as Gui

class MySelectionObserver:
    def addSelection(self, doc, obj, sub, pnt):
        print(f"Object selected: {obj.Name}")
        # Update views based on selection
    
    def removeSelection(self, doc, obj, sub):
        print(f"Object deselected: {obj.Name}")
    
    def setSelection(self, doc):
        print("Selection cleared")
    
    def clearSelection(self, doc):
        print("Selection cleared")

# Attach observer
observer = MySelectionObserver()
Gui.Selection.addObserver(observer)
```

### Selection Synchronization Flow

```
User selects object in 3D view
    ↓
Gui.Selection.addSelection(object)
    ↓
Selection signal emitted
    ↓
┌─────────────────────────────────────┐
│  Multiple Observers Receive Signal │
└─────────────────────────────────────┘
    │
    ├─→ Tree View Observer
    │   └─→ Highlights tree item
    │
    ├─→ Property Editor Observer
    │   └─→ Displays object properties
    │
    └─→ ViewProvider
        └─→ Highlights object in 3D view
```

---

## 9. Complete Synchronization Example

### Scenario: User Changes Box Length

```python
# Initial state
box = doc.addObject("Part::Box", "Box")
box.Length = 10
box.Width = 10
box.Height = 10
doc.recompute()

# All views are synchronized:
# - 3D view: Shows 10x10x10 box
# - Tree view: Shows "Box" item
# - Property editor: Shows Length=10, Width=10, Height=10
```

**User changes Length in Property Editor**:

```python
box.Length = 20  # User edits in property editor
```

**Synchronization Flow**:

1. **Property Setter**:
   ```cpp
   box.Length = 20
   // → PropertyContainer::setPropertyValue("Length", 20)
   // → Property::setPyObject(20)
   // → signalChanged(Length) emitted
   ```

2. **ViewProvider Receives Signal**:
   ```cpp
   ViewProvider::updateData(&Length)
   // → Detects Length changed
   // → Marks object for recomputation
   // → Updates visual representation
   ```

3. **Document Recomputation**:
   ```cpp
   Document::recompute()
   // → execute() on box
   // → box.Shape recomputed (20x10x10)
   // → signalRecomputed() emitted
   ```

4. **3D View Update**:
   ```cpp
   ViewProvider::updateVisual()
   // → Updates Coin3D shape node
   // → 3D view redraws
   // → User sees updated box size
   ```

5. **Property Editor Update**:
   ```cpp
   PropertyEditorObserver::slotChangedObject(box, Length)
   // → Updates Length display (if needed)
   // → Property editor shows Length=20
   ```

6. **Tree View Update**:
   ```cpp
   // Tree view doesn't need update (Length change doesn't affect tree)
   // But if Label changed, tree would update
   ```

**Result**: All views are synchronized:
- ✅ 3D view: Shows 20x10x10 box
- ✅ Property editor: Shows Length=20
- ✅ Tree view: Unchanged (no tree update needed)

---

## 10. Headless Mode (No GUI)

### App-Only Operation

FreeCAD can run without GUI (headless mode):

```python
# Headless mode (no GUI)
import FreeCAD as App

# App module works without Gui
doc = App.newDocument()
box = doc.addObject("Part::Box", "Box")
box.Length = 10
doc.recompute()

# No ViewProvider created (Gui not available)
# No 3D view, tree view, or property editor
# But App logic works perfectly
```

**Key Point**: App module is completely independent of Gui module. Signals are still emitted, but no observers receive them (no GUI to observe).

### Gui Availability Check

Code can check if GUI is available:

```python
if App.GuiUp:
    # GUI is available
    view_provider = box.ViewObject
    # Use GUI features
else:
    # Headless mode
    # Skip GUI features
```

---

## 11. Observer Pattern Benefits

### Loose Coupling

- **App doesn't know about Gui**: App emits signals, doesn't care who listens
- **Gui observes App**: Gui connects to App signals, but App doesn't depend on Gui
- **Multiple Observers**: Multiple views can observe the same signals

### Extensibility

- **New Views**: New views can be added by creating new observers
- **Custom Observers**: Users can create custom observers for automation
- **Plugin System**: Plugins can observe signals without modifying core code

### Testability

- **App Testing**: App logic can be tested without GUI
- **Mock Observers**: Test observers can be created to verify signal emission
- **Isolation**: App and Gui can be tested independently

---

## 12. Summary

### Key Components

1. **App Module**: Core logic, data model, signal emission
2. **Gui Module**: Visual representation, user interaction, signal observation
3. **Signals**: Boost.Signals2-based notification system
4. **Observers**: DocumentObserver, SelectionObserver, ViewProvider

### Synchronization Flow

1. **Property Change** → Signal emitted
2. **Observers Receive** → Multiple views notified
3. **Views Update** → 3D view, tree view, property editor update
4. **User Sees Changes** → All views synchronized

### Architecture Benefits

- **Separation of Concerns**: App and Gui are independent
- **Multiple Views**: All views stay synchronized automatically
- **Headless Operation**: App works without GUI
- **Extensibility**: Easy to add new views or observers

---

## References

- FreeCAD Architecture: App/Gui Separation
- Signal System: Boost.Signals2
- DocumentObserver: `App::DocumentObserver`
- SelectionObserver: `Gui::SelectionObserver`
- ViewProvider: `Gui::ViewProvider`
- Document Signals: `signalNewObject`, `signalChangedObject`, etc.

