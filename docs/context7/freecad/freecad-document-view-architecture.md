# FreeCAD Document/View Architecture: App/Gui Separation and Multiple Views

This document explains in detail FreeCAD's document/view architecture: how adding/removing/updating DocumentObjects triggers GUI updates, the signal/observer mechanisms between App::Document and Gui::Document, and how multiple views reflect the same underlying object.

## Overview

FreeCAD uses a **strict separation** between application data (`App`) and graphical interface (`Gui`), with `App::Document` and `Gui::Document` working together through signal/observer mechanisms. This architecture enables multiple views (3D view, tree view, property editor) to observe and reflect the same underlying `DocumentObject` instances, ensuring consistency across all representations.

---

## 1. App::Document and Gui::Document Architecture

### Document Pairing

Every `App::Document` has a corresponding `Gui::Document` (when GUI is available):

```python
# App side: Core data
app_doc = App.newDocument("MyDocument")
# - Contains DocumentObjects
# - Manages object lifecycle
# - Emits signals for changes
# - No GUI knowledge

# Gui side: Visual representation
if App.GuiUp:
    gui_doc = Gui.ActiveDocument  # Gui::Document
    # - Observes App::Document
    # - Manages ViewProviders
    # - Updates views when App::Document changes
    # - Handles user interaction
```

### Document Relationship

```
App::Document (data model)
    ↓ (1:1 relationship)
Gui::Document (view model)
    ↓ (observes App::Document)
    ├─→ ViewProviders (one per DocumentObject)
    ├─→ Tree View Observer
    ├─→ Property Editor Observer
    └─→ 3D View Observers
```

**Key Point**: `Gui::Document` observes `App::Document` but `App::Document` doesn't know about `Gui::Document`.

---

## 2. Signal/Observer Mechanisms

### Boost.Signals2 Foundation

FreeCAD uses **Boost.Signals2** for signal/slot communication:

**Features**:
- **Type-Safe**: Signals know their parameter types
- **Multiple Observers**: Multiple slots can connect to one signal
- **Thread-Safe**: Signals can be emitted from different threads
- **Automatic Cleanup**: Slots disconnected when objects destroyed

### App::Document Signals

`App::Document` emits signals for document-level events:

```cpp
// C++ Signal Definitions (simplified)
class App::Document {
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
    
    // Signal: Object touched (needs recomputation)
    boost::signals2::signal<void (DocumentObject*)> signalTouchedObject;
};
```

### Gui::Document as Observer

`Gui::Document` observes `App::Document` signals:

```cpp
// C++ Gui::Document (simplified)
class Gui::Document {
    App::Document* pcDocument;  // Observed App document
    
    // Connect to App::Document signals
    void attachDocument(App::Document* doc) {
        pcDocument = doc;
        
        // Connect to document signals
        doc->signalNewObject.connect(
            boost::bind(&Gui::Document::slotNewObject, this, _1)
        );
        doc->signalDeletedObject.connect(
            boost::bind(&Gui::Document::slotDeletedObject, this, _1)
        );
        doc->signalChangedObject.connect(
            boost::bind(&Gui::Document::slotChangedObject, this, _1, _2)
        );
        doc->signalRecomputed.connect(
            boost::bind(&Gui::Document::slotRecomputed, this, _1)
        );
    }
    
    // Slot handlers
    void slotNewObject(DocumentObject* obj);
    void slotDeletedObject(DocumentObject* obj);
    void slotChangedObject(const DocumentObject& obj, const Property& prop);
    void slotRecomputed(const Document& doc);
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

## 3. Adding DocumentObjects: Complete Flow

### Scenario: Adding a New Object

```python
# User action or script
box = doc.addObject("Part::Box", "Box")
```

### Complete Flow

```
doc.addObject("Part::Box", "Box")
    ↓
App::Document::addObject()
    ↓
Create DocumentObject instance
    ↓
Register object in document
    ↓
signalNewObject(box) emitted
    ↓
┌─────────────────────────────────────┐
│  Multiple Observers Receive Signal │
└─────────────────────────────────────┘
    │
    ├─→ Gui::Document::slotNewObject()
    │   └─→ Create ViewProvider
    │       └─→ ViewProvider.attach(box)
    │           └─→ Connect to box signals
    │               └─→ Update 3D view
    │
    ├─→ Tree View Observer
    │   └─→ Add item to tree view
    │
    └─→ Property Editor Observer
        └─→ (If selected) Display properties
```

### Detailed Steps

#### Step 1: App::Document::addObject()

```cpp
// C++ Implementation (simplified)
DocumentObject* Document::addObject(const char* type, const char* name)
{
    // Create object instance
    DocumentObject* obj = createObject(type, name);
    
    // Register in document
    Objects.push_back(obj);
    objectMap[name] = obj;
    
    // Emit signal
    signalNewObject(obj);  // ← Signal emitted
    
    return obj;
}
```

#### Step 2: Gui::Document Receives Signal

```cpp
// Gui::Document::slotNewObject()
void Gui::Document::slotNewObject(DocumentObject* obj)
{
    // Create ViewProvider for object
    ViewProvider* vp = createViewProvider(obj);
    
    if (vp) {
        // Attach ViewProvider to object
        vp->attach(obj);
        
        // Store ViewProvider
        viewProviders[obj] = vp;
        
        // Update views
        updateViews();
    }
}
```

#### Step 3: ViewProvider Creation

```cpp
// ViewProvider::attach()
void ViewProvider::attach(DocumentObject* obj)
{
    pcObject = obj;
    
    // Connect to object signals
    obj->signalChanged.connect(
        boost::bind(&ViewProvider::updateData, this, _1)
    );
    obj->signalTouched.connect(
        boost::bind(&ViewProvider::onUpdate, this)
    );
    
    // Initial visual update
    updateData(nullptr);  // Update all properties
}
```

#### Step 4: Tree View Update

```cpp
// Tree View Observer
void TreeViewObserver::slotNewObject(const DocumentObject& obj)
{
    // Add new item to tree
    QTreeWidgetItem* item = new QTreeWidgetItem();
    item->setText(0, obj.Label.getValue());
    item->setData(0, Qt::UserRole, obj.getID());
    treeWidget->addTopLevelItem(item);
    
    // Update tree view
    treeWidget->update();
}
```

#### Step 5: 3D View Update

```cpp
// ViewProvider::updateData()
void ViewProvider::updateData(const Property* prop)
{
    // Update Coin3D scene graph
    SoSeparator* root = getRootNode();
    
    // Add shape node
    SoShape* shapeNode = getShapeNode();
    TopoDS_Shape shape = pcObject->Shape.getValue();
    shapeNode->shape.setValue(convertToCoin3D(shape));
    
    // Request redraw
    getViewer()->redraw();
}
```

---

## 4. Removing DocumentObjects: Complete Flow

### Scenario: Removing an Object

```python
# User action or script
doc.removeObject(box)
```

### Complete Flow

```
doc.removeObject(box)
    ↓
App::Document::removeObject()
    ↓
signalDeletedObject(box) emitted
    ↓
┌─────────────────────────────────────┐
│  Multiple Observers Receive Signal │
└─────────────────────────────────────┘
    │
    ├─→ Gui::Document::slotDeletedObject()
    │   └─→ Destroy ViewProvider
    │       └─→ Remove from 3D view
    │
    ├─→ Tree View Observer
    │   └─→ Remove item from tree
    │
    └─→ Property Editor Observer
        └─→ (If selected) Clear display
    ↓
Remove object from document
```

### Detailed Steps

#### Step 1: App::Document::removeObject()

```cpp
// C++ Implementation (simplified)
void Document::removeObject(DocumentObject* obj)
{
    // Emit signal before removal
    signalDeletedObject(obj);  // ← Signal emitted
    
    // Remove from document
    Objects.erase(std::remove(Objects.begin(), Objects.end(), obj), Objects.end());
    objectMap.erase(obj->getName());
    
    // Delete object
    delete obj;
}
```

#### Step 2: Gui::Document Receives Signal

```cpp
// Gui::Document::slotDeletedObject()
void Gui::Document::slotDeletedObject(DocumentObject* obj)
{
    // Get ViewProvider
    ViewProvider* vp = viewProviders[obj];
    
    if (vp) {
        // Detach from object
        vp->detach();
        
        // Remove from 3D view
        removeFromView(vp);
        
        // Delete ViewProvider
        delete vp;
        viewProviders.erase(obj);
    }
}
```

#### Step 3: Tree View Update

```cpp
// Tree View Observer
void TreeViewObserver::slotDeletedObject(const DocumentObject& obj)
{
    // Find and remove item
    QTreeWidgetItem* item = findItemByObject(obj);
    if (item) {
        treeWidget->removeItemWidget(item, 0);
        delete item;
    }
    
    // Update tree view
    treeWidget->update();
}
```

---

## 5. Updating DocumentObjects: Complete Flow

### Scenario: Property Change

```python
# User changes property
box.Length = 20
```

### Complete Flow

```
box.Length = 20
    ↓
Property setter
    ↓
Property value stored
    ↓
onChanged(Length) called
    ↓
signalChanged(Length) emitted
    ↓
┌─────────────────────────────────────┐
│  Multiple Observers Receive Signal │
└─────────────────────────────────────┘
    │
    ├─→ ViewProvider::updateData(Length)
    │   └─→ Update 3D view (if geometric property)
    │
    ├─→ Tree View Observer
    │   └─→ (If Label changed) Update tree item
    │
    └─→ Property Editor Observer
        └─→ Update property display
```

### Detailed Steps

#### Step 1: Property Assignment

```cpp
// Property setter
void PropertyContainer::setPropertyValue(const char* name, PyObject* value)
{
    Property* prop = getPropertyByName(name);
    prop->setPyObject(value);  // Store value
    
    // Call onChanged()
    onChanged(prop);
}
```

#### Step 2: onChanged() and Signal Emission

```cpp
// DocumentObject::onChanged()
void DocumentObject::onChanged(const Property* prop)
{
    // Object-specific handling
    // ...
    
    // Emit signal
    signalChanged(*prop);  // ← Signal emitted
}
```

#### Step 3: ViewProvider Receives Signal

```cpp
// ViewProvider::updateData()
void ViewProvider::updateData(const Property* prop)
{
    if (prop == &pcObject->Length || 
        prop == &pcObject->Width || 
        prop == &pcObject->Height) {
        // Geometric property changed
        // Mark for recomputation (Shape will change)
        onUpdate();
    } else if (prop == &pcObject->Placement) {
        // Placement changed - update position
        updatePlacement();
    } else if (prop == &pcObject->Visibility) {
        // Visibility changed
        if (pcObject->Visibility.getValue()) {
            show();
        } else {
            hide();
        }
    }
}
```

#### Step 4: 3D View Update

```cpp
// ViewProvider::updateVisual()
void ViewProvider::updateVisual()
{
    // Get updated shape
    TopoDS_Shape shape = pcObject->Shape.getValue();
    
    // Update Coin3D node
    SoShape* shapeNode = getShapeNode();
    shapeNode->shape.setValue(convertToCoin3D(shape));
    
    // Update placement
    SoTransform* transform = getTransformNode();
    Base::Placement placement = pcObject->Placement.getValue();
    transform->translation.setValue(
        placement.getPosition().x,
        placement.getPosition().y,
        placement.getPosition().z
    );
    
    // Request redraw
    getViewer()->redraw();
}
```

---

## 6. Multiple Views Reflecting Same Object

### View Types

FreeCAD supports multiple views of the same document:

1. **3D Views**: Multiple 3D viewports showing the same document
2. **Tree View**: Hierarchical object list
3. **Property Editor**: Property display for selected object
4. **Report View**: Console messages and errors

### View Synchronization

All views observe the same `App::Document` and `DocumentObject` signals:

```
App::Document (single source of truth)
    ↓ (signals)
┌─────────────────────────────────────┐
│  Multiple Observers                 │
└─────────────────────────────────────┘
    │
    ├─→ ViewProvider 1 (3D View 1)
    ├─→ ViewProvider 2 (3D View 2)
    ├─→ Tree View Observer
    └─→ Property Editor Observer
```

### Example: Multiple 3D Views

```python
# Create document
doc = App.newDocument("MyDocument")
box = doc.addObject("Part::Box", "Box")

# Multiple 3D views observe same object
# View 1: Front view
# View 2: Top view
# View 3: Isometric view

# All views show the same box
# When box.Length changes:
box.Length = 20
# All 3 views update simultaneously
```

### ViewProvider Sharing

**Key Point**: Each `DocumentObject` has **one ViewProvider**, but that ViewProvider can update **multiple 3D views**:

```cpp
// ViewProvider updates all views
void ViewProvider::updateVisual()
{
    // Update Coin3D scene graph
    // (Scene graph is shared across views)
    SoShape* shapeNode = getShapeNode();
    shapeNode->shape.setValue(convertToCoin3D(shape));
    
    // All views using this scene graph update
    // (Coin3D handles view updates automatically)
}
```

### Tree View and Property Editor

**Tree View**:
- Observes `App::Document` signals
- Updates when objects added/removed/changed
- Shows all objects in document

**Property Editor**:
- Observes selection signals
- Observes property change signals
- Updates when selected object changes
- Updates when properties change

---

## 7. Signal/Observer Registration

### Document Observer Registration

**Gui::Document** registers as observer of **App::Document**:

```cpp
// Gui::Document initialization
void Gui::Document::initDocument(App::Document* doc)
{
    pcDocument = doc;
    
    // Register as observer
    doc->signalNewObject.connect(
        boost::bind(&Gui::Document::slotNewObject, this, _1)
    );
    doc->signalDeletedObject.connect(
        boost::bind(&Gui::Document::slotDeletedObject, this, _1)
    );
    doc->signalChangedObject.connect(
        boost::bind(&Gui::Document::slotChangedObject, this, _1, _2)
    );
    doc->signalRecomputed.connect(
        boost::bind(&Gui::Document::slotRecomputed, this, _1)
    );
}
```

### ViewProvider Observer Registration

**ViewProvider** registers as observer of **DocumentObject**:

```cpp
// ViewProvider::attach()
void ViewProvider::attach(DocumentObject* obj)
{
    pcObject = obj;
    
    // Connect to object signals
    obj->signalChanged.connect(
        boost::bind(&ViewProvider::updateData, this, _1)
    );
    obj->signalTouched.connect(
        boost::bind(&ViewProvider::onUpdate, this)
    );
    obj->signalRecomputed.connect(
        boost::bind(&ViewProvider::onRecomputed, this)
    );
}
```

### Tree View Observer Registration

**Tree View** registers as observer of **App::Document**:

```cpp
// Tree View Observer
class TreeViewObserver : public DocumentObserver {
    void attachDocument(App::Document* doc) {
        DocumentObserver::attachDocument(doc);
        
        // Already connected via DocumentObserver base class
        // slotNewObject, slotDeletedObject, etc. are called automatically
    }
};
```

---

## 8. Complete Example: Object Lifecycle

### Example: Creating and Modifying an Object

```python
# Step 1: Create document
doc = App.newDocument("MyDocument")
# App::Document created
# Gui::Document created (if GUI available)
# Gui::Document observes App::Document

# Step 2: Add object
box = doc.addObject("Part::Box", "Box")
# App::Document::addObject() called
# signalNewObject(box) emitted
# Gui::Document::slotNewObject() called
# ViewProvider created and attached
# Tree view item added
# 3D view updated (box appears)

# Step 3: Modify property
box.Length = 20
# Property setter called
# onChanged(Length) called
# signalChanged(Length) emitted
# ViewProvider::updateData(Length) called
# Object marked for recomputation
# Tree view unchanged (Length doesn't affect tree)

# Step 4: Recompute
doc.recompute()
# execute() called on box
# box.Shape recomputed
# signalRecomputed() emitted
# ViewProvider::onRecomputed() called
# 3D view updated (box size changes)

# Step 5: Change label
box.Label = "MyBox"
# Property setter called
# onChanged(Label) called
# signalChanged(Label) emitted
# ViewProvider::updateData(Label) called
# Tree view observer updates item text
# 3D view unchanged (Label doesn't affect geometry)

# Step 6: Remove object
doc.removeObject(box)
# App::Document::removeObject() called
# signalDeletedObject(box) emitted
# Gui::Document::slotDeletedObject() called
# ViewProvider destroyed
# Tree view item removed
# 3D view updated (box disappears)
```

---

## 9. View Update Patterns

### Pattern 1: Immediate Update

**When**: Property changes that affect visual representation immediately.

**Example**: Visibility, Color, Transparency

```python
box.Visibility = False
# signalChanged(Visibility) emitted
# ViewProvider::updateData(Visibility) called immediately
# 3D view updates immediately (box hidden)
```

### Pattern 2: Deferred Update (After Recomputation)

**When**: Property changes that require recomputation.

**Example**: Length, Width, Height (affect Shape)

```python
box.Length = 20
# signalChanged(Length) emitted
# ViewProvider::updateData(Length) called
# Object marked for recomputation
# 3D view not updated yet

doc.recompute()
# execute() called
# box.Shape recomputed
# signalRecomputed() emitted
# ViewProvider::onRecomputed() called
# 3D view updated (box size changes)
```

### Pattern 3: Batch Update

**When**: Multiple properties change together.

**Example**: Changing multiple dimensions

```python
box.Length = 20
box.Width = 15
box.Height = 10
# Multiple signalChanged() calls
# ViewProvider::updateData() called multiple times
# But recomputation happens once
doc.recompute()
# Single recomputation updates all changes
```

---

## 10. Error Handling in View Updates

### Error Communication

When view updates fail, errors are communicated:

```cpp
// ViewProvider::updateData() with error handling
void ViewProvider::updateData(const Property* prop)
{
    try {
        if (prop == &pcObject->Shape) {
            // Update visual representation
            updateVisual();
        }
    } catch (const std::exception& e) {
        // Error updating view
        Console().Error("Error updating view for %s: %s\n",
                       pcObject->Label.getValue(), e.what());
        
        // Mark object as erroneous
        pcObject->setStatus(ObjectStatus::Erroneous, true);
    }
}
```

### View Update Failures

**If view update fails**:
- Error logged to console
- Object marked as erroneous
- View may show error indicator
- Other views continue to function

---

## 11. Multiple Document Support

### Multiple Documents

FreeCAD supports multiple documents, each with its own views:

```python
# Create multiple documents
doc1 = App.newDocument("Document1")
doc2 = App.newDocument("Document2")

# Each has its own Gui::Document
gui_doc1 = Gui.getDocument("Document1")
gui_doc2 = Gui.getDocument("Document2")

# Each Gui::Document observes its App::Document
# Views are document-specific
```

### Active Document

**Active Document**: The currently active document.

```python
# Get active documents
app_doc = App.ActiveDocument      # App::Document
gui_doc = Gui.ActiveDocument      # Gui::Document

# Set active document
Gui.setActiveDocument("Document1")
```

---

## 12. Summary

### Document/View Architecture

1. **App::Document**: Core data model, emits signals
2. **Gui::Document**: View model, observes App::Document
3. **ViewProviders**: One per DocumentObject, observe DocumentObject
4. **Multiple Views**: All observe same App::Document

### Signal/Observer Mechanisms

1. **App::Document Signals**: signalNewObject, signalDeletedObject, signalChangedObject
2. **DocumentObject Signals**: signalChanged, signalTouched, signalRecomputed
3. **Observer Registration**: Gui::Document and ViewProviders register as observers
4. **Automatic Updates**: Views update automatically when signals emitted

### Object Lifecycle

1. **Add Object**: signalNewObject → ViewProvider created → Views updated
2. **Update Object**: signalChanged → ViewProvider::updateData() → Views updated
3. **Remove Object**: signalDeletedObject → ViewProvider destroyed → Views updated

### Multiple Views

1. **Same Object**: Multiple views observe same DocumentObject
2. **Synchronized Updates**: All views update when object changes
3. **View Types**: 3D views, tree view, property editor all synchronized

### Key Principles

1. **Separation**: App and Gui are completely separate
2. **Observer Pattern**: Gui observes App, not vice versa
3. **Signal-Based**: All updates triggered by signals
4. **Automatic Synchronization**: Views stay synchronized automatically

---

## References

- Document Architecture: `App::Document`, `Gui::Document`
- Signal System: Boost.Signals2
- Observer Pattern: `App::DocumentObserver`, `Gui::DocumentObserver`
- ViewProviders: `Gui::ViewProvider`
- View Updates: `ViewProvider::updateData()`, `ViewProvider::onUpdate()`

