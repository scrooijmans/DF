# Fiji/ImageJ Architecture Analysis: Processing Algorithm Execution Flow

## Executive Summary

This document provides a comprehensive architectural analysis of Fiji (Fiji Is Just ImageJ), an open-source image processing platform built on ImageJ2 and the SciJava framework. The analysis focuses on the end-to-end flow of processing algorithm execution: from user selection through execution to output registration and visualization.

**Key Insight**: Fiji uses a dual-architecture approach:
- **ImageJ1 (Legacy)**: Simple `PlugIn`/`PlugInFilter` interfaces with direct `run()` method invocation
- **ImageJ2/SciJava (Modern)**: Annotation-based plugin discovery with dependency injection and command pattern

---

## 1. High-Level Architecture

### Core Subsystems

```
┌─────────────────────────────────────────────────────────────────┐
│                         User Interface Layer                     │
│  (Menus, Toolbars, Dialogs, Image Windows, Results Tables)      │
└────────────────────────────┬────────────────────────────────────┘
                             │
                             ▼
┌─────────────────────────────────────────────────────────────────┐
│                      Plugin Framework Layer                      │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐          │
│  │   Plugin     │  │   Plugin     │  │   Plugin     │          │
│  │  Discovery   │  │ Registration │  │  Execution   │          │
│  │  (SciJava)   │  │  (Registry)  │  │  (Commands)  │          │
│  └──────────────┘  └──────────────┘  └──────────────┘          │
└────────────────────────────┬────────────────────────────────────┘
                             │
                             ▼
┌─────────────────────────────────────────────────────────────────┐
│                      Processing Engine Layer                     │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐          │
│  │  Algorithm   │  │   Parameter  │  │   Execution  │          │
│  │  Execution   │  │  Validation  │  │   Context    │          │
│  └──────────────┘  └──────────────┘  └──────────────┘          │
└────────────────────────────┬────────────────────────────────────┘
                             │
                             ▼
┌─────────────────────────────────────────────────────────────────┐
│                        Data Model Layer                          │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐          │
│  │  ImagePlus   │  │  WindowMgr   │  │   Dataset    │          │
│  │  (Image Data)│  │  (Registry)  │  │  (Metadata)  │          │
│  └──────────────┘  └──────────────┘  └──────────────┘          │
└────────────────────────────┬────────────────────────────────────┘
                             │
                             ▼
┌─────────────────────────────────────────────────────────────────┐
│                      Persistence Layer                           │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐          │
│  │  File I/O    │  │  Serialize   │  │   Metadata   │          │
│  │  (TIFF, etc) │  │  (Memory)    │  │   Storage    │          │
│  └──────────────┘  └──────────────┘  └──────────────┘          │
└─────────────────────────────────────────────────────────────────┘
```

### Responsibility Separation

1. **UI Layer**: User interaction, parameter dialogs, visualization
2. **Plugin Framework**: Discovery, registration, lifecycle management
3. **Processing Engine**: Algorithm execution, parameter binding, validation
4. **Data Model**: Image representation, metadata, window management
5. **Persistence**: File I/O, serialization, metadata storage

**Key Design Principle**: Clear separation allows plugins to be developed independently without knowledge of UI implementation details.

---

## 2. Algorithm / Tool Registration

### ImageJ1 (Legacy) Approach

**Interface-Based Registration**:

```java
// Simple plugin interface
public interface PlugIn {
    void run(String arg);
}

// Filter plugin interface (for in-place operations)
public interface PlugInFilter {
    int setup(String arg, ImagePlus imp);
    void run(ImageProcessor ip);
}
```

**Discovery Mechanism**:
- Classpath scanning at startup
- Plugins placed in `plugins/` directory or JAR files
- Menu integration via naming conventions or `plugins.config` file
- **No explicit registration step** - discovery is implicit

**Example**:
```java
public class GaussianBlur_ implements PlugInFilter {
    private double sigma = 2.0;
    
    public int setup(String arg, ImagePlus imp) {
        // Show dialog for parameters
        GenericDialog gd = new GenericDialog("Gaussian Blur");
        gd.addNumericField("Sigma:", sigma, 2);
        gd.showDialog();
        if (gd.wasCanceled()) return DONE;
        sigma = gd.getNextNumber();
        return DOES_ALL; // Process all image types
    }
    
    public void run(ImageProcessor ip) {
        ip.blurGaussian(sigma);
    }
}
```

### ImageJ2/SciJava (Modern) Approach

**Annotation-Based Registration**:

```java
@Plugin(type = Command.class, 
        menuPath = "Process>Filters>Gaussian Blur",
        headless = true)
public class GaussianBlurCommand implements Command {
    
    @Parameter
    private ImagePlusService imagePlusService;
    
    @Parameter(label = "Input Image")
    private ImagePlus inputImage;
    
    @Parameter(label = "Sigma", min = "0.1", max = "100.0")
    private double sigma = 2.0;
    
    @Parameter(label = "Output Image", choices = {"New Window", "Same Window"})
    private String outputType = "New Window";
    
    @Override
    public void run() {
        // Dependency injection provides services
        ImageProcessor ip = inputImage.getProcessor();
        ip.blurGaussian(sigma);
        
        ImagePlus result;
        if (outputType.equals("New Window")) {
            result = new ImagePlus("Gaussian Blur", ip.duplicate());
            result.show();
        } else {
            inputImage.updateAndDraw();
        }
    }
}
```

**Discovery & Registration Flow**:

1. **Classpath Scanning**: SciJava scans for `@Plugin` annotations at startup
2. **Plugin Index Creation**: Builds an index of all discovered plugins
3. **Service Registration**: Registers plugins in the `PluginService` registry
4. **Menu Integration**: Automatically adds plugins to menus based on `menuPath`
5. **Metadata Extraction**: Extracts parameter metadata from annotations for UI generation

**Key Classes**:
- `org.scijava.plugin.PluginService`: Central registry for all plugins
- `org.scijava.plugin.PluginInfo`: Metadata container for each plugin
- `org.scijava.command.Command`: Base interface for executable commands
- `org.scijava.plugin.Parameter`: Annotation for parameter injection

**Metadata Definition**:
- **Name**: From class name or `@Plugin(name = "...")`
- **Menu Path**: From `@Plugin(menuPath = "...")`
- **Inputs**: From `@Parameter` annotations with type information
- **Outputs**: Implicit from return type or `@Parameter` output fields
- **Parameters**: Type, validation rules, default values from annotations

---

## 3. Execution Call Stack

### ImageJ1 Execution Flow

```
1. User clicks menu item "Process > Filters > Gaussian Blur"
   │
   ▼
2. MenuActionListener.actionPerformed()
   │
   ▼
3. Menus.runPlugin(className, arg)
   │
   ▼
4. Class.forName(className).newInstance()
   │
   ▼
5. PlugInFilter plugin = (PlugInFilter) instance;
   │
   ▼
6. ImagePlus imp = WindowManager.getCurrentImage();
   │
   ▼
7. int flags = plugin.setup(arg, imp);
   │   ├─> Shows GenericDialog for parameters
   │   └─> Returns processing flags (DOES_ALL, DOES_8G, etc.)
   │
   ▼
8. if (flags != DONE) {
       ImageProcessor ip = imp.getProcessor();
       plugin.run(ip);
       imp.updateAndDraw(); // Refresh display
   }
```

### ImageJ2/SciJava Execution Flow

```
1. User clicks menu item "Process > Filters > Gaussian Blur"
   │
   ▼
2. MenuActionListener.actionPerformed()
   │
   ▼
3. CommandService.run(CommandClass.class, true)
   │   ├─> true = show dialog for parameters
   │
   ▼
4. PluginService.getPlugin(CommandClass.class)
   │   └─> Returns PluginInfo from registry
   │
   ▼
5. CommandService.create(PluginInfo)
   │   ├─> Instantiates command class
   │   ├─> Injects @Parameter fields via dependency injection
   │   └─> Returns Command instance with parameters bound
   │
   ▼
6. if (showDialog) {
       ParameterDialog.show(Command)
       │   ├─> Auto-generates UI from @Parameter annotations
       │   ├─> Validates inputs based on annotation constraints
       │   └─> Updates Command instance with user values
   }
   │
   ▼
7. CommandService.run(Command)
   │
   ▼
8. Command.run()
   │   ├─> Executes algorithm logic
   │   ├─> Creates/modifies ImagePlus objects
   │   └─> Shows results in windows
   │
   ▼
9. WindowManager.addWindow(ImagePlus)
   │   └─> Registers new image in window registry
```

### Key Classes and Methods

**ImageJ1**:
- `ij.Menus.runPlugin(String className, String arg)`: Entry point for plugin execution
- `ij.plugin.PlugIn.run(String arg)`: Plugin execution method
- `ij.plugin.PlugInFilter.setup(String arg, ImagePlus imp)`: Parameter collection
- `ij.plugin.PlugInFilter.run(ImageProcessor ip)`: Processing execution
- `ij.WindowManager.getCurrentImage()`: Get active image
- `ij.ImagePlus.getProcessor()`: Get pixel data processor

**ImageJ2/SciJava**:
- `org.scijava.command.CommandService.run(Class<? extends Command>, boolean)`: Entry point
- `org.scijava.plugin.PluginService.getPlugin(Class<?>)`: Plugin lookup
- `org.scijava.ui.UIService.show(Command)`: Parameter dialog display
- `org.scijava.command.Command.run()`: Command execution
- `org.scijava.plugin.Parameter`: Parameter injection annotation

---

## 4. Data Model & Inputs

### ImagePlus: Core Data Structure

```java
public class ImagePlus {
    private String title;              // Window title / dataset name
    private ImageStack stack;          // Pixel data (2D or 3D)
    private Calibration calibration;   // Spatial calibration, units
    private Properties properties;     // Metadata (key-value pairs)
    private FileInfo fileInfo;         // File origin information
    private Overlay overlay;           // ROIs, annotations
    private int currentSlice;          // Current Z/T/C position
    private boolean composite;         // Multi-channel display mode
}
```

**Key Characteristics**:
- **In-Memory Representation**: Pixel data stored as `ImageStack` (array of `ImageProcessor`)
- **Metadata**: Stored in `Properties` object (key-value pairs)
- **Lazy Loading**: Large images can be loaded on-demand via `FileInfo.virtualStack`
- **Reference Semantics**: `ImagePlus` objects are passed by reference, not copied

### Input Data Referencing

**Methods of Referencing**:

1. **Current Image** (Most Common):
   ```java
   ImagePlus imp = WindowManager.getCurrentImage();
   if (imp == null) {
       IJ.error("No image is open.");
       return;
   }
   ```

2. **By Title**:
   ```java
   ImagePlus imp = WindowManager.getImage("MyImage.tif");
   ```

3. **By ID**:
   ```java
   ImagePlus imp = WindowManager.getImage(id);
   ```

4. **File Path** (Lazy Loading):
   ```java
   ImagePlus imp = IJ.openImage("/path/to/image.tif");
   // For virtual stacks, data loaded on-demand
   ```

5. **Direct Creation** (New Data):
   ```java
   ImagePlus imp = new ImagePlus("Title", new FloatProcessor(width, height));
   ```

### Data Handling Strategies

**Copy vs Reference**:
- **Copy**: `ImageProcessor.duplicate()` creates independent copy
- **Reference**: Direct `ImageProcessor` access modifies original
- **PlugInFilter.run()`: Typically modifies in-place (reference)
- **PlugIn.run()`: Typically creates new `ImagePlus` (copy)

**Lazy Loading**:
```java
// Virtual stack - data loaded on-demand
FileInfo fi = new FileInfo();
fi.virtualStack = true;
fi.directory = "/path/to/stack/";
fi.fileName = "slice_%04d.tif";
ImagePlus imp = new ImagePlus("Virtual Stack", new VirtualStack(fi));
// Slices loaded only when accessed
```

**Memory Management**:
- Images remain in memory until window is closed
- `WindowManager` maintains registry of all open images
- No automatic garbage collection of displayed images
- User must explicitly close windows to free memory

---

## 5. Output Creation

### Creating New Datasets

**Basic Output Creation**:

```java
// Method 1: Create from ImageProcessor
ImageProcessor ip = inputImage.getProcessor().duplicate();
ip.blurGaussian(2.0);
ImagePlus output = new ImagePlus("Blurred Image", ip);
output.show();

// Method 2: Create from pixel array
float[] pixels = new float[width * height];
// ... fill pixels ...
FloatProcessor fp = new FloatProcessor(width, height, pixels);
ImagePlus output = new ImagePlus("New Image", fp);
output.show();

// Method 3: Create stack (3D/4D)
ImageStack stack = new ImageStack(width, height);
for (int i = 1; i <= nSlices; i++) {
    ImageProcessor slice = processSlice(i);
    stack.addSlice("Slice " + i, slice);
}
ImagePlus output = new ImagePlus("Stack", stack);
output.show();
```

### Naming Conventions

**Automatic Naming**:
- Default: Based on operation name + input name
- Example: "Gaussian Blur of MyImage.tif"
- Can be customized in plugin code

**Versioning**:
- No built-in versioning system
- Users manually rename outputs
- Some plugins append suffixes (e.g., "_filtered", "_processed")

### Typing and Metadata

**Data Types**:
- `8-bit`: 0-255 integer values
- `16-bit`: 0-65535 integer values
- `32-bit float`: Floating-point values
- `RGB`: 24-bit color (3 channels)
- Type determined by `ImageProcessor` subclass

**Metadata Propagation**:
```java
// Copy calibration from input
output.setCalibration(inputImage.getCalibration().copy());

// Copy properties
output.setProperties(inputImage.getProperties());

// Add processing metadata
output.setProperty("Processing", "Gaussian Blur");
output.setProperty("Sigma", String.valueOf(sigma));
output.setProperty("Input Image", inputImage.getTitle());
```

### Immutability vs Mutability

**Default Behavior**:
- **ImageJ1 PlugInFilter**: Modifies input in-place (mutable)
- **ImageJ1 PlugIn**: Creates new output (immutable input)
- **ImageJ2 Commands**: Typically create new outputs (immutable)

**Overwriting**:
```java
// Option 1: Replace current image
ImagePlus imp = WindowManager.getCurrentImage();
imp.setProcessor("New Title", newProcessor);
imp.updateAndDraw();

// Option 2: Close and replace
imp.changes = false; // Mark as unchanged to avoid save prompt
imp.close();
new ImagePlus("New Title", newProcessor).show();
```

### Window Registration

**Automatic Registration**:
```java
ImagePlus output = new ImagePlus("Title", processor);
output.show(); // Automatically registered in WindowManager
```

**WindowManager Registry**:
- Maintains list of all open `ImagePlus` windows
- Provides lookup by title, ID, or index
- Tracks "current" (active) image
- Manages window ordering and focus

---

## 6. Persistence & Database Interaction

### File-Based Persistence

**Primary Storage Mechanism**: File system, not database

**Supported Formats**:
- TIFF (most common, supports metadata)
- JPEG, PNG (lossy/lossless)
- DICOM (medical imaging)
- HDF5 (via plugins)
- Custom formats via `FileSaver` interface

**Saving Process**:
```java
ImagePlus imp = WindowManager.getCurrentImage();
IJ.save(imp, "/path/to/output.tif");
// Or
FileSaver fs = new FileSaver(imp);
fs.saveAsTiff("/path/to/output.tif");
```

### Metadata Storage

**TIFF Metadata**:
- Stored in TIFF tags
- Includes calibration, properties, overlay
- Preserved when saving/loading

**Properties Object**:
```java
// Store custom metadata
imp.setProperty("Author", "John Doe");
imp.setProperty("Processing Date", new Date().toString());
imp.setProperty("Parameters", "sigma=2.0, method=gaussian");

// Properties saved in TIFF file
IJ.save(imp, "output.tif");
// Properties restored when loading
ImagePlus loaded = IJ.openImage("output.tif");
String author = (String) loaded.getProperty("Author");
```

### In-Memory Lifecycle

**Lifecycle Stages**:
1. **Creation**: `new ImagePlus(...)` - allocated in heap
2. **Display**: `imp.show()` - registered in WindowManager
3. **Active Use**: User interacts, processes image
4. **Closure**: User closes window or `imp.close()`
5. **Garbage Collection**: When no references remain

**Serialization**:
- No built-in Java serialization
- Custom serialization via file formats
- Virtual stacks serialize file paths, not pixel data

### Transaction Boundaries

**No Database Transactions**:
- File operations are atomic at OS level
- No rollback mechanism
- User must manually manage backups

**Atomicity Concerns**:
- Saving large images: May take time, no intermediate state
- Multi-file operations: Not atomic (e.g., saving stack as separate files)

### Database Interaction (ImageJ2/SciJava)

**Optional Database Support**:
- ImageJ2 can integrate with databases via plugins
- `DatasetService` can store metadata in database
- Pixel data still typically in files
- Database used for search, indexing, provenance

**Example Integration** (Conceptual):
```java
@Parameter
private DatasetService datasetService;

public void run() {
    Dataset dataset = datasetService.create(outputImage);
    datasetService.save(dataset); // Saves metadata to DB
    // Pixel data saved to file, path stored in DB
}
```

---

## 7. Provenance & Reproducibility

### Limited Built-in Provenance

**ImageJ1**: No automatic provenance tracking

**Manual Tracking**:
- Users document workflows manually
- Properties object can store processing history
- No automatic DAG construction

**Example Manual Provenance**:
```java
String history = (String) imp.getProperty("Processing History");
if (history == null) history = "";
history += "\nGaussian Blur: sigma=" + sigma + ", date=" + new Date();
imp.setProperty("Processing History", history);
```

### ImageJ2/SciJava Provenance

**Enhanced Tracking** (via plugins):
- `CommandService` can log executed commands
- Parameters stored in command context
- Can reconstruct processing chain from logs

**Provenance Data Structure** (Conceptual):
```java
public class ProcessingStep {
    private String commandClass;
    private Map<String, Object> parameters;
    private List<String> inputImageIds;
    private String outputImageId;
    private Date timestamp;
}
```

### Workflow Representation

**No Built-in DAG**:
- ImageJ does not maintain explicit DAG
- Workflows are linear sequences
- Users can build DAGs via scripting (ImageJ Macro, Python, etc.)

**Script-Based Workflows**:
```javascript
// ImageJ Macro example
run("Gaussian Blur...", "sigma=2.0");
rename("blurred");
run("Threshold...", "method=Otsu");
rename("thresholded");
// Linear workflow, no explicit DAG
```

**External DAG Tools**:
- KNIME, CellProfiler: Can use ImageJ as processing nodes
- These tools maintain DAG separately
- ImageJ acts as "black box" processing unit

### Reproducibility Strategies

**1. Macro Recording**:
```javascript
// ImageJ can record user actions as macro
// Replay macro to reproduce workflow
run("Startup Macros");
// ... user actions recorded ...
run("Create Macro");
```

**2. Scripting**:
- ImageJ Macro language
- Python (via Jython)
- JavaScript (via Rhino)
- Scripts are reproducible, shareable

**3. Batch Processing**:
```javascript
// Process multiple images with same parameters
inputDir = "/path/to/inputs/";
outputDir = "/path/to/outputs/";
list = getFileList(inputDir);
for (i = 0; i < list.length; i++) {
    open(inputDir + list[i]);
    run("Gaussian Blur...", "sigma=2.0");
    saveAs("Tiff", outputDir + list[i]);
    close();
}
```

---

## 8. Extensibility Points

### Adding a New Processing Algorithm

#### ImageJ1 Plugin

**Minimal Implementation**:

```java
import ij.*;
import ij.plugin.filter.PlugInFilter;
import ij.process.ImageProcessor;

public class MyFilter_ implements PlugInFilter {
    
    public int setup(String arg, ImagePlus imp) {
        // Return flags indicating what this plugin can process
        return DOES_ALL; // Processes 8-bit, 16-bit, 32-bit, RGB
    }
    
    public void run(ImageProcessor ip) {
        // Get pixel array
        int width = ip.getWidth();
        int height = ip.getHeight();
        
        // Process pixels
        for (int y = 0; y < height; y++) {
            for (int x = 0; x < width; x++) {
                double value = ip.getPixelValue(x, y);
                // ... processing ...
                ip.putPixelValue(x, y, processedValue);
            }
        }
    }
}
```

**Registration**:
1. Place `.class` file in `plugins/` directory, OR
2. Package in JAR file in `plugins/` directory, OR
3. Add to classpath
4. Menu integration via `plugins.config` or naming convention

#### ImageJ2/SciJava Command

**Minimal Implementation**:

```java
import org.scijava.command.Command;
import org.scijava.plugin.Parameter;
import org.scijava.plugin.Plugin;
import ij.ImagePlus;

@Plugin(type = Command.class, 
        menuPath = "Process>My Category>My Filter")
public class MyFilterCommand implements Command {
    
    @Parameter(label = "Input Image")
    private ImagePlus inputImage;
    
    @Parameter(label = "Parameter 1", min = "0", max = "100")
    private double param1 = 50.0;
    
    @Override
    public void run() {
        ImageProcessor ip = inputImage.getProcessor().duplicate();
        
        // Process image
        int width = ip.getWidth();
        int height = ip.getHeight();
        for (int y = 0; y < height; y++) {
            for (int x = 0; x < width; x++) {
                double value = ip.getPixelValue(x, y);
                // ... processing with param1 ...
                ip.putPixelValue(x, y, processedValue);
            }
        }
        
        // Create output
        ImagePlus output = new ImagePlus("Filtered " + inputImage.getTitle(), ip);
        output.show();
    }
}
```

**Registration**:
- Automatic via `@Plugin` annotation
- Discovered at startup by SciJava
- Menu automatically generated from `menuPath`

### Required Interfaces

**ImageJ1**:
- `ij.plugin.PlugIn`: For algorithms that create new images
- `ij.plugin.filter.PlugInFilter`: For algorithms that modify existing images
- `ij.plugin.PlugIn`: For file I/O, analysis tools

**ImageJ2/SciJava**:
- `org.scijava.command.Command`: For executable commands
- `org.scijava.plugin.Plugin`: Annotation for registration
- `org.scijava.plugin.Parameter`: Annotation for parameters

### Service Injection (ImageJ2)

**Available Services**:
```java
@Parameter
private ImagePlusService imagePlusService; // Image management

@Parameter
private UIService uiService; // UI dialogs, display

@Parameter
private LogService logService; // Logging

@Parameter
private StatusService statusService; // Status bar updates

@Parameter
private OpService opService; // ImageJ Ops (algorithm library)
```

**Dependency Injection Benefits**:
- Loose coupling: Plugin doesn't know service implementation
- Testability: Services can be mocked
- Flexibility: Services can be swapped

---

## 9. Design Patterns Worth Copying

### 1. Plugin Architecture Pattern

**Pattern**: Discovery-based plugin system with minimal coupling

**Benefits**:
- Extensibility without modifying core
- Third-party developers can add functionality
- Clear separation of concerns

**Implementation Strategy**:
- Use annotations or interfaces for plugin contracts
- Runtime discovery via classpath scanning
- Central registry for plugin lookup
- Dependency injection for service access

### 2. Command Pattern (ImageJ2)

**Pattern**: Encapsulate requests as objects

**Benefits**:
- Undo/redo capability (if implemented)
- Queuing and logging of operations
- Parameter validation before execution

**Implementation**:
```java
public interface Command {
    void run();
}

// Commands can be stored, logged, replayed
List<Command> history = new ArrayList<>();
history.add(new GaussianBlurCommand(input, sigma));
```

### 3. Dependency Injection (SciJava)

**Pattern**: Invert control of dependencies

**Benefits**:
- Loose coupling
- Testability
- Flexible service substitution

**Implementation**:
- Annotate fields with `@Parameter`
- Framework injects services at runtime
- No manual service lookup needed

### 4. Lazy Loading Pattern

**Pattern**: Load data only when needed

**Benefits**:
- Memory efficiency for large datasets
- Fast startup time
- On-demand resource usage

**Implementation**:
```java
public class VirtualStack implements ImageStack {
    public ImageProcessor getProcessor(int n) {
        // Load slice n from disk only when accessed
        return loadSliceFromDisk(n);
    }
}
```

### 5. Window Manager Pattern

**Pattern**: Central registry for UI objects

**Benefits**:
- Single source of truth for open items
- Easy lookup and management
- Consistent state across application

**Implementation**:
```java
public class WindowManager {
    private static Map<Integer, ImagePlus> windows = new HashMap<>();
    
    public static ImagePlus getCurrentImage() {
        return windows.get(currentId);
    }
    
    public static void addWindow(ImagePlus imp) {
        windows.put(imp.getID(), imp);
    }
}
```

### 6. Metadata-Driven UI Generation

**Pattern**: Generate UI from metadata annotations

**Benefits**:
- Consistent UI across plugins
- Automatic validation
- Less boilerplate code

**Implementation**:
```java
@Parameter(label = "Threshold", min = "0", max = "255")
private int threshold = 128;

// Framework generates:
// - Label: "Threshold"
// - Input field with range validation
// - Default value: 128
```

---

## 10. Constraints & Tradeoffs

### Performance vs. Flexibility

**Tradeoff**: Plugin system adds indirection overhead

**Mitigation**:
- Direct method calls for hot paths
- Caching of plugin lookups
- Compile-time optimization where possible

**Impact**: 
- Plugin execution: ~1-5% overhead vs. direct calls
- Acceptable for most use cases
- Critical paths can bypass plugin system

### Memory Usage

**Constraint**: All open images kept in memory

**Tradeoff**: 
- Fast access vs. memory consumption
- Large datasets can exhaust memory

**Mitigation**:
- Virtual stacks (lazy loading)
- User must manage window lifecycle
- Batch mode (hide images during processing)

### User Experience vs. Complexity

**Tradeoff**: Rich functionality vs. learning curve

**ImageJ1**: Simple but limited
- Easy to understand
- Manual parameter dialogs
- Inconsistent UI

**ImageJ2**: Powerful but complex
- Automatic UI generation
- Dependency injection
- Steeper learning curve for developers

### Provenance vs. Performance

**Constraint**: No built-in provenance tracking

**Tradeoff**:
- Fast execution vs. reproducibility
- Users must manually document workflows

**Mitigation**:
- Scripting for reproducibility
- External tools for workflow management
- Optional provenance plugins

### Extensibility vs. Stability

**Tradeoff**: Easy plugin addition vs. API stability

**Challenge**:
- Plugin APIs must remain stable
- Core changes can break plugins
- Versioning complexity

**Mitigation**:
- Deprecation warnings
- Compatibility layers
- Plugin version requirements

---

## 11. Architectural Diagram (Text Description)

```
┌─────────────────────────────────────────────────────────────────────┐
│                            USER INTERFACE                            │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐             │
│  │    Menus     │  │   Toolbars   │  │    Windows   │             │
│  │  (Actions)   │  │  (Tools)     │  │  (Display)   │             │
│  └──────┬───────┘  └──────┬───────┘  └──────┬───────┘             │
│         │                  │                  │                      │
│         └──────────────────┼──────────────────┘                      │
│                            │                                          │
└────────────────────────────┼──────────────────────────────────────────┘
                             │
                             ▼
┌─────────────────────────────────────────────────────────────────────┐
│                         PLUGIN FRAMEWORK                             │
│  ┌──────────────────────────────────────────────────────────────┐  │
│  │              PluginService (Registry)                         │  │
│  │  ┌──────────┐  ┌──────────┐  ┌──────────┐  ┌──────────┐    │  │
│  │  │ Plugin 1 │  │ Plugin 2 │  │ Plugin 3 │  │ Plugin N │    │  │
│  │  │ (Info)   │  │ (Info)   │  │ (Info)   │  │ (Info)   │    │  │
│  │  └──────────┘  └──────────┘  └──────────┘  └──────────┘    │  │
│  └──────────────────────────────────────────────────────────────┘  │
│                            │                                          │
│         ┌──────────────────┼──────────────────┐                      │
│         │                  │                  │                      │
│         ▼                  ▼                  ▼                      │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐             │
│  │   Command    │  │   Parameter  │  │   Execution  │             │
│  │   Service    │  │    Dialog    │  │   Context    │             │
│  └──────────────┘  └──────────────┘  └──────────────┘             │
└────────────────────────────┼──────────────────────────────────────────┘
                             │
                             ▼
┌─────────────────────────────────────────────────────────────────────┐
│                        PROCESSING ENGINE                             │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐             │
│  │  Algorithm   │  │   Parameter  │  │   Execution  │             │
│  │  Execution   │  │  Validation  │  │   Monitor    │             │
│  └──────────────┘  └──────────────┘  └──────────────┘             │
└────────────────────────────┼──────────────────────────────────────────┘
                             │
                             ▼
┌─────────────────────────────────────────────────────────────────────┐
│                          DATA MODEL                                  │
│  ┌──────────────────────────────────────────────────────────────┐  │
│  │                    WindowManager                              │  │
│  │  ┌──────────┐  ┌──────────┐  ┌──────────┐  ┌──────────┐    │  │
│  │  │ImagePlus │  │ImagePlus │  │ImagePlus │  │ImagePlus │    │  │
│  │  │ (Image1) │  │ (Image2) │  │ (Image3) │  │ (ImageN) │    │  │
│  │  └──────────┘  └──────────┘  └──────────┘  └──────────┘    │  │
│  └──────────────────────────────────────────────────────────────┘  │
│         │                                                            │
│         │ Each ImagePlus contains:                                  │
│         │  - ImageStack (pixel data)                                │
│         │  - Calibration (spatial info)                             │
│         │  - Properties (metadata)                                  │
│         │  - Overlay (ROIs, annotations)                            │
│         │                                                            │
└────────────────────────────┼──────────────────────────────────────────┘
                             │
                             ▼
┌─────────────────────────────────────────────────────────────────────┐
│                        PERSISTENCE LAYER                             │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐             │
│  │   File I/O   │  │ Serialization│  │   Metadata   │             │
│  │  (TIFF, etc) │  │   (Memory)   │  │   Storage    │             │
│  └──────────────┘  └──────────────┘  └──────────────┘             │
└─────────────────────────────────────────────────────────────────────┘
```

---

## 12. Step-by-Step Execution Flow (Detailed)

### Complete Flow: User Selects Algorithm → Output Available

**Step 1: User Selection**
- User clicks menu item: "Process > Filters > Gaussian Blur"
- UI layer captures menu action event

**Step 2: Plugin Lookup**
- Menu system resolves menu path to plugin class name
- `PluginService.getPlugin(className)` called
- Returns `PluginInfo` with metadata

**Step 3: Command Instantiation**
- `CommandService.create(PluginInfo)` called
- Framework instantiates command class via reflection
- Dependency injection populates `@Parameter` fields with services

**Step 4: Parameter Collection**
- If interactive mode: `ParameterDialog.show(command)` called
- Dialog auto-generated from `@Parameter` annotations
- User enters: sigma = 2.0, output = "New Window"
- Dialog validates inputs (range checks, type validation)
- Command instance updated with user values

**Step 5: Input Resolution**
- Command's `@Parameter ImagePlus inputImage` field
- If not set: `WindowManager.getCurrentImage()` called
- Input image loaded into memory if not already loaded
- Reference to `ImagePlus` object passed to command

**Step 6: Validation**
- Framework validates all required parameters are set
- Input image validated (not null, correct type, etc.)
- Parameter constraints checked (sigma in valid range)

**Step 7: Execution**
- `CommandService.run(command)` called
- Command's `run()` method invoked
- Algorithm executes:
  ```java
  ImageProcessor ip = inputImage.getProcessor().duplicate();
  ip.blurGaussian(2.0);
  ImagePlus output = new ImagePlus("Gaussian Blur", ip);
  ```

**Step 8: Output Creation**
- New `ImagePlus` object created with processed data
- Metadata copied from input (calibration, properties)
- Processing metadata added (algorithm, parameters, timestamp)

**Step 9: Window Registration**
- `output.show()` called
- `WindowManager.addWindow(output)` called internally
- Window added to window list
- Window displayed on screen

**Step 10: Availability**
- Output image immediately available for:
  - Visualization (displayed in window)
  - Further processing (can be selected as input)
  - Saving to disk
  - Analysis operations

**Step 11: Persistence (Optional)**
- User can save output: `IJ.save(output, "output.tif")`
- File written to disk with metadata
- Image remains in memory until window closed

---

## 13. Simplified Pseudo-Code Example

### Algorithm Definition

```java
// ImageJ2/SciJava style
@Plugin(type = Command.class, 
        menuPath = "Process>My Category>My Algorithm",
        headless = true)
public class MyAlgorithmCommand implements Command {
    
    // Dependency injection
    @Parameter
    private ImagePlusService imageService;
    
    @Parameter
    private LogService logService;
    
    // User parameters (auto-generated UI)
    @Parameter(label = "Input Image")
    private ImagePlus inputImage;
    
    @Parameter(label = "Threshold", min = "0", max = "255")
    private int threshold = 128;
    
    @Parameter(label = "Output Type", 
               choices = {"New Window", "Replace Input"})
    private String outputType = "New Window";
    
    @Override
    public void run() {
        // 1. Validate inputs
        if (inputImage == null) {
            logService.error("No input image selected");
            return;
        }
        
        // 2. Get pixel data
        ImageProcessor ip = inputImage.getProcessor();
        
        // 3. Process image
        int width = ip.getWidth();
        int height = ip.getHeight();
        ImageProcessor result = ip.duplicate();
        
        for (int y = 0; y < height; y++) {
            for (int x = 0; x < width; x++) {
                double value = ip.getPixelValue(x, y);
                double processed = (value > threshold) ? 255 : 0;
                result.putPixelValue(x, y, processed);
            }
        }
        
        // 4. Create output
        ImagePlus output = new ImagePlus(
            "Thresholded " + inputImage.getTitle(), 
            result
        );
        
        // 5. Copy metadata
        output.setCalibration(inputImage.getCalibration().copy());
        output.setProperties(inputImage.getProperties());
        output.setProperty("Algorithm", "MyAlgorithm");
        output.setProperty("Threshold", String.valueOf(threshold));
        
        // 6. Display result
        if (outputType.equals("New Window")) {
            output.show();
        } else {
            inputImage.setProcessor(output.getTitle(), result);
            inputImage.updateAndDraw();
        }
        
        logService.info("Processing complete");
    }
}
```

### Execution

```java
// Framework execution (simplified)
public class CommandService {
    
    public void run(Class<? extends Command> commandClass, 
                    boolean showDialog) {
        // 1. Get plugin info
        PluginInfo info = pluginService.getPlugin(commandClass);
        
        // 2. Create command instance
        Command command = createCommand(info);
        
        // 3. Show parameter dialog if needed
        if (showDialog) {
            parameterDialog.show(command);
            if (parameterDialog.wasCanceled()) {
                return;
            }
        }
        
        // 4. Validate parameters
        validateParameters(command);
        
        // 5. Execute
        try {
            command.run();
        } catch (Exception e) {
            logService.error("Execution failed", e);
        }
    }
    
    private Command createCommand(PluginInfo info) {
        // Instantiate via reflection
        Class<?> clazz = info.getPluginClass();
        Command command = (Command) clazz.newInstance();
        
        // Inject services
        injectServices(command);
        
        // Inject parameters (if pre-set)
        injectParameters(command, info);
        
        return command;
    }
}
```

### Output Registration

```java
// WindowManager (simplified)
public class WindowManager {
    private static Map<Integer, ImagePlus> windows = new HashMap<>();
    private static int currentId = 0;
    private static int nextId = 1;
    
    public static void addWindow(ImagePlus imp) {
        int id = nextId++;
        imp.setID(id);
        windows.put(id, imp);
        currentId = id;
        
        // Create and show window
        ImageWindow win = new ImageWindow(imp);
        win.setVisible(true);
    }
    
    public static ImagePlus getCurrentImage() {
        return windows.get(currentId);
    }
    
    public static ImagePlus getImage(String title) {
        for (ImagePlus imp : windows.values()) {
            if (imp.getTitle().equals(title)) {
                return imp;
            }
        }
        return null;
    }
}

// ImagePlus.show() method
public void show() {
    if (win == null) {
        WindowManager.addWindow(this);
    } else {
        win.toFront();
    }
}
```

---

## 14. Key Takeaways for Re-Implementation

### Essential Components

1. **Plugin Registry**: Central discovery and lookup mechanism
2. **Command Pattern**: Encapsulate operations as executable objects
3. **Dependency Injection**: Loose coupling via service injection
4. **Data Model**: Rich in-memory representation with metadata
5. **Window Management**: Central registry for UI objects
6. **Parameter System**: Metadata-driven UI generation

### Design Principles

1. **Separation of Concerns**: UI, processing, data, persistence are separate
2. **Extensibility First**: Plugin system enables third-party additions
3. **Immediate Feedback**: Results available instantly for further processing
4. **Metadata Preservation**: Processing history and parameters tracked
5. **Lazy Loading**: Large datasets loaded on-demand

### Implementation Strategy

1. **Start Simple**: Begin with basic plugin interface (like ImageJ1)
2. **Add Metadata**: Introduce annotations for richer metadata
3. **Inject Dependencies**: Add DI framework for services
4. **Generate UI**: Auto-generate dialogs from metadata
5. **Track Provenance**: Add optional provenance tracking

### Tradeoffs to Consider

1. **Performance**: Plugin overhead vs. flexibility
2. **Memory**: In-memory data vs. lazy loading
3. **Complexity**: Rich features vs. learning curve
4. **Provenance**: Automatic tracking vs. performance cost
5. **Stability**: API changes vs. plugin compatibility

---

## 15. Conclusion

Fiji/ImageJ demonstrates a mature, extensible architecture for scientific data processing applications. The dual-architecture approach (ImageJ1 simplicity + ImageJ2 power) provides both ease of use and advanced capabilities.

**Key Strengths**:
- Highly extensible plugin system
- Immediate result availability
- Rich metadata support
- Lazy loading for large datasets

**Key Limitations**:
- Limited built-in provenance tracking
- No explicit DAG representation
- Memory management requires user attention
- File-based persistence (no database)

**Applicability to Other Domains**:
The architecture patterns are highly applicable to domains like:
- Well log processing (petrophysics)
- Seismic data analysis
- Time series analysis
- Scientific computing workflows

The plugin-based extensibility, metadata-driven UI, and immediate result availability are particularly valuable patterns to adopt.

---

## References

- ImageJ Documentation: https://imagej.net/
- SciJava Framework: https://scijava.org/
- ImageJ Source Code: https://github.com/imagej/imagej
- Fiji Source Code: https://github.com/fiji/fiji
- Plugin Development Guide: https://imagej.net/develop/plugins

