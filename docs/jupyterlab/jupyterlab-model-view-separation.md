# JupyterLab Model-View Separation and Multi-View Observation

This document explains how JupyterLab separates document models from their visual representations, and how multiple views can observe and mutate the same underlying model.

## Overview

JupyterLab uses a strict **Model-View-Controller (MVC)** architecture where:

- **Models**: Represent the data and business logic (document content, state)
- **Views (Widgets)**: Represent the visual presentation
- **Context**: Connects models to views and manages document lifecycle

This separation enables:

- Multiple views of the same document
- Synchronized updates across all views
- Independent view lifecycle from model lifecycle
- Efficient state management

## Architecture Components

### Document Model (IModel)

Models represent the underlying data structure:

```typescript
interface IModel {
  // Serialization
  toJSON(): PartialJSONValue;
  fromJSON(value: ReadonlyPartialJSONValue): void;

  // Change notifications
  contentChanged: ISignal<IModel, void>;
  stateChanged: ISignal<
    IModel,
    IChangedArgs<any, any, string>
  >;

  // State
  dirty: boolean;
  readOnly: boolean;
}
```

**Key Characteristics:**

- **Data-only**: Models contain no UI code
- **Observable**: Emit signals when state changes
- **Serializable**: Can be converted to/from JSON
- **Stateless UI**: Views observe models, models don't know about views

### Document Context (IContext)

Context connects models to the document management system:

```typescript
interface IContext<T extends IModel> {
  // Model reference
  model: T;

  // Document metadata
  path: string;
  localPath: string;
  contentsModel: null | Omit<Contents.IModel, 'content'>;

  // Lifecycle
  isReady: boolean;
  ready: Promise<void>;
  disposed: ISignal<IContext<T>, void>;

  // Change signals
  fileChanged: ISignal<
    IContext<T>,
    Omit<Contents.IModel, 'content'>
  >;
  pathChanged: ISignal<IContext<T>, string>;
  saveState: ISignal<
    IContext<T>,
    DocumentRegistry.SaveState
  >;

  // Multiple views
  addSibling(
    widget: Widget,
    options?: IOpenOptions
  ): IDisposable;

  // Operations
  save(): Promise<void>;
  revert(): Promise<void>;
  createCheckpoint(): Promise<ICheckpointModel>;
  restoreCheckpoint(checkpointID?: string): Promise<void>;
}
```

**Key Responsibilities:**

- **Model Management**: Owns and manages the document model
- **View Coordination**: Enables multiple views of the same model
- **File Operations**: Handles save, load, checkpoint operations
- **Lifecycle**: Manages document lifecycle independent of views

### Document Widget (IDocumentWidget)

Widgets are the visual representations:

```typescript
interface IDocumentWidget<
  T extends Widget,
  U extends IModel
> {
  context: IContext<U>;
  content: T;
  // ... widget properties
}
```

**Key Characteristics:**

- **View-only**: Widgets contain UI code, not data
- **Model Reference**: Access model through context
- **Observers**: Listen to model signals for updates
- **Independent Lifecycle**: Can be created/destroyed without affecting model

## Model-View Separation

### Creating Models

Models are created by **Model Factories** registered with the Document Registry:

```typescript
// Model factory creates the data model
const modelFactory = new TextModelFactory();
registry.addModelFactory(modelFactory);

// Model is created independently of any view
const model = modelFactory.createNew();
```

**Process:**

1. Model factory is registered with document registry
2. Model is created when document is opened
3. Model exists independently of any widgets
4. Model persists even if all views are closed

### Creating Views

Views are created by **Widget Factories** that receive a context:

```typescript
// Widget factory creates views
const widgetFactory = new FileEditorFactory({
  name: 'Editor',
  modelName: 'text',
  fileTypes: ['*'],
  defaultFor: ['*'],
});
registry.addWidgetFactory(widgetFactory);

// View is created with context (which contains model)
const widget = widgetFactory.createNew(context);
```

**Process:**

1. Widget factory is registered with document registry
2. Factory receives a context (which contains the model)
3. Widget is created and observes the model
4. Multiple widgets can be created for the same context

### Separation Benefits

**1. Model Independence:**

```typescript
// Model exists without any views
const model = modelFactory.createNew();
model.fromJSON({ content: 'Hello World' });

// Model persists even if views are closed
// Views can be recreated later using the same model
```

**2. View Independence:**

```typescript
// Multiple views can observe the same model
const widget1 = factory.createNew(context);
const widget2 = factory.createNew(context); // Same context = same model

// Views can be closed without affecting the model
widget1.dispose(); // Model still exists
```

**3. Lifecycle Independence:**

- Models can outlive views
- Views can be recreated from existing models
- Model state persists across view lifecycle

## Multiple Views of the Same Model

### Sibling Widgets

JupyterLab supports multiple views (siblings) of the same document:

```typescript
// Create first view
const widget1 = factory.createNew(context);

// Add sibling view (shares same model and context)
const disposable = context.addSibling(widget2, {
  mode: 'split-right',
  ref: widget1,
});
```

**Key Points:**

- Siblings share the **same context** and **same model**
- Changes in one view immediately reflect in all views
- Each view maintains its own UI state (scroll position, selection, etc.)
- Siblings can be removed independently

### Context Sharing

All widgets created from the same context share the model:

```typescript
// All these widgets observe the same model
const widget1 = factory1.createNew(context); // Editor view
const widget2 = factory2.createNew(context); // Preview view
const widget3 = factory3.createNew(context); // Another editor view

// All three widgets see the same model changes
```

**Architecture:**

```
                    ┌─────────────┐
                    │   Context   │
                    │  (Manager)  │
                    └──────┬──────┘
                           │
                           │ model
                           ▼
                    ┌─────────────┐
                    │    Model    │
                    │  (Data)     │
                    └──────┬──────┘
                           │
        ┌──────────────────┼──────────────────┐
        │                  │                  │
        ▼                  ▼                  ▼
  ┌─────────┐        ┌─────────┐        ┌─────────┐
  │ Widget1 │        │ Widget2 │        │ Widget3 │
  │ (View)  │        │ (View)  │        │ (View)  │
  └─────────┘        └─────────┘        └─────────┘
```

## Observing Model Changes

### Signal-Based Observation

Models emit signals when their state changes:

```typescript
// Model signals
model.contentChanged: ISignal<IModel, void>;
model.stateChanged: ISignal<IModel, IChangedArgs<any, any, string>>;
```

### Widget Observation Pattern

Widgets observe models through signals:

```typescript
class DocumentWidget extends Widget {
  constructor(context: IContext<IModel>) {
    super();
    this.context = context;

    // Observe model changes
    context.model.contentChanged.connect(
      this._onContentChanged,
      this
    );
    context.model.stateChanged.connect(
      this._onStateChanged,
      this
    );

    // Observe context changes
    context.fileChanged.connect(this._onFileChanged, this);
    context.pathChanged.connect(this._onPathChanged, this);
  }

  private _onContentChanged(sender: IModel): void {
    // Update view when model content changes
    this.update();
  }

  private _onStateChanged(
    sender: IModel,
    args: IChangedArgs
  ): void {
    // React to specific state changes
    if (args.name === 'dirty') {
      this._updateTitle();
    }
  }

  private _onFileChanged(
    sender: IContext,
    args: Contents.IModel
  ): void {
    // React to file system changes
    this._syncWithFile();
  }
}
```

### Update Request Pattern

Widgets request updates when model changes:

```typescript
class NotebookWidget extends Widget {
  private _onModelContentChanged(
    model: INotebookModel
  ): void {
    // Request widget update
    this.update();
  }

  protected onUpdateRequest(msg: Message): void {
    // Update DOM based on current model state
    this._renderCells();
    this._updateLayout();
  }
}
```

**Update Flow:**

1. Model changes → emits signal
2. Widget receives signal → calls `update()`
3. Widget's `onUpdateRequest()` is called
4. Widget re-renders based on current model state

## Mutating Models

### Direct Model Mutation

Views can mutate models directly:

```typescript
// Widget mutates model
class EditorWidget extends Widget {
  private _onTextChange(): void {
    // Direct mutation
    this.context.model.fromJSON({
      content: this._editor.value,
    });
    // Model emits contentChanged signal
    // All observing widgets update automatically
  }
}
```

### Observable Lists

For structured data (like notebook cells), models use observable lists:

```typescript
// Notebook model has observable cell list
interface INotebookModel {
  cells: CellList; // IObservableList<ICellModel>
}

// Adding a cell
model.cells.push(newCellModel);
// All notebook widgets automatically update

// Removing a cell
model.cells.remove(0);
// All notebook widgets automatically update

// Modifying a cell
cellModel.value = 'new code';
// All views of that cell update automatically
```

### Change Propagation

When a model is mutated:

```
┌─────────────┐
│   Widget1   │──┐
│  (mutates)  │  │
└─────────────┘  │
                 │ mutation
                 ▼
          ┌─────────────┐
          │    Model    │
          │  (changed)  │
          └──────┬──────┘
                 │
                 │ emits signals
                 │
    ┌────────────┼────────────┐
    │            │            │
    ▼            ▼            ▼
┌─────────┐ ┌─────────┐ ┌─────────┐
│ Widget1 │ │ Widget2 │ │ Widget3 │
│ (auto   │ │ (auto   │ │ (auto   │
│ update) │ │ update) │ │ update) │
└─────────┘ └─────────┘ └─────────┘
```

**Process:**

1. Widget mutates model (direct or through context)
2. Model updates its internal state
3. Model emits change signals (`contentChanged`, `stateChanged`)
4. All observing widgets receive signals
5. Widgets call `update()` to refresh their views
6. All views reflect the new model state

## Example: Notebook Model and Views

### Notebook Model Structure

```typescript
interface INotebookModel extends IModel {
  cells: CellList; // Observable list of cells
  metadata: IObservableMap<any>;
  nbformat: number;
  nbformatMinor: number;

  // Signals
  contentChanged: ISignal<INotebookModel, void>;
  stateChanged: ISignal<INotebookModel, IChangedArgs>;
  cellsChanged: ISignal<
    CellList,
    IObservableList.IChangedArgs<ICellModel>
  >;
}
```

### Multiple Notebook Views

```typescript
// Create notebook context (contains model)
const context = await docManager.open('notebook.ipynb');

// Create multiple views
const view1 = notebookFactory.createNew(context); // Editor view
const view2 = notebookFactory.createNew(context); // Another editor view

// Both views observe the same model
// Changes in one view immediately appear in the other
```

### Cell Mutation Example

```typescript
// User edits cell in view1
view1.model.cells.get(0).value = "print('Hello')";

// Model emits signals:
// - cellsChanged (cell list changed)
// - contentChanged (notebook content changed)
// - stateChanged (dirty state changed)

// Both views automatically update:
// - view1: Shows edited cell
// - view2: Shows same edited cell
```

## Serialization and Persistence

### Model Serialization

Models can be serialized to JSON:

```typescript
// Serialize model to JSON
const json = model.toJSON();
// Save to file or send to server

// Deserialize from JSON
model.fromJSON(json);
// Emits contentChanged signal
// All views update automatically
```

### Context Save Operations

Context handles model persistence:

```typescript
// Save model to file
await context.save();

// Process:
// 1. Context gets model.toJSON()
// 2. Sends to server
// 3. Emits saveState signal
// 4. All views can react to save completion
```

**Save Flow:**

```
Widget → Context.save() → Model.toJSON() → Server
                                    ↓
                            saveState signal
                                    ↓
                            All widgets update
```

## Document Registry Architecture

### Factory Registration

The Document Registry manages model and widget factories:

```typescript
// Register model factory
registry.addModelFactory({
  name: 'text',
  contentType: 'file',
  fileFormat: 'text',
  createNew: (options) => new DocumentModel(),
});

// Register widget factory
registry.addWidgetFactory({
  name: 'Editor',
  modelName: 'text', // Links to model factory
  fileTypes: ['*'],
  createNew: (context) => new EditorWidget(context),
});
```

### Factory Relationship

```
Document Registry
├── Model Factories
│   ├── TextModelFactory → creates DocumentModel
│   ├── NotebookModelFactory → creates NotebookModel
│   └── ...
│
└── Widget Factories
    ├── EditorFactory (modelName: 'text')
    ├── NotebookFactory (modelName: 'notebook')
    └── ...
```

**Linking:**

- Widget factories specify `modelName` to link to model factory
- Context is created with appropriate model factory
- Widget factory receives context with model

## Best Practices

### Model Design

1. **Keep Models Pure**: No UI code in models
2. **Emit Signals**: Always emit signals on state changes
3. **Use Observable Collections**: For lists/maps, use observable types
4. **Immutable Updates**: Prefer immutable update patterns
5. **Serializable**: Ensure models can be serialized to JSON

### Widget Design

1. **Observe, Don't Store**: Access model through context, don't cache
2. **React to Signals**: Connect to all relevant model signals
3. **Request Updates**: Call `update()` when model changes
4. **Handle Disposal**: Disconnect signals on disposal
5. **UI State Separate**: Keep view-specific state (scroll, selection) separate from model

### Context Usage

1. **One Context Per Document**: Each document has one context
2. **Multiple Widgets Per Context**: Create siblings for multiple views
3. **Context Lifecycle**: Context outlives individual widgets
4. **Operations Through Context**: Use context for save/load operations
5. **Signal Observation**: Observe context signals for file operations

## Summary

JupyterLab's model-view separation provides:

1. **Separation of Concerns**: Models contain data, widgets contain UI
2. **Multiple Views**: Multiple widgets can observe the same model
3. **Synchronized Updates**: Changes propagate automatically via signals
4. **Independent Lifecycles**: Models and views have independent lifecycles
5. **Efficient State Management**: Single source of truth (model) with multiple observers
6. **Serialization**: Models can be persisted independently of views
7. **Extensibility**: New views can be added without modifying models

The architecture enables powerful features like:

- Split views of the same document
- Real-time collaboration (multiple users, one model)
- Undo/redo (model history, not view history)
- Efficient rendering (only update what changed)
- Plugin extensibility (new views for existing models)

