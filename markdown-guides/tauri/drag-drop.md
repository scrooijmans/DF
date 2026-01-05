# Tauri v2 Drag and Drop Configuration

## Overview

This guide explains how to properly configure drag and drop functionality in Tauri v2 applications, specifically for file upload scenarios using libraries like Dropzone.js.

## Key Configuration

### Tauri Configuration (`src-tauri/tauri.conf.json`)

**IMPORTANT**: Set `dragDropEnabled` to `false` to enable proper drag and drop functionality:

```json
{
  "app": {
    "windows": [
      {
        "fullscreen": false,
        "resizable": true,
        "title": "Your App",
        "width": 1200,
        "height": 800,
        "minWidth": 800,
        "minHeight": 600,
        "dragDropEnabled": false // ← Set to FALSE for proper drag and drop
      }
    ]
  }
}
```

## Why `dragDropEnabled: false`?

### The Counter-Intuitive Solution

Despite the name suggesting it enables drag and drop, setting `dragDropEnabled: false` is actually what makes drag and drop work properly in Tauri v2. Here's why:

1. **Native vs Web Drag and Drop**: When `dragDropEnabled: true`, Tauri intercepts drag and drop events at the native level, preventing web libraries like Dropzone.js from receiving them.

2. **Web Library Compatibility**: Setting it to `false` allows the webview to handle drag and drop events normally, which enables JavaScript libraries to work as expected.

3. **Event Propagation**: With `dragDropEnabled: false`, drag events properly propagate to the DOM and can be handled by JavaScript event listeners.

## Migration from Tauri v1

### v1 Configuration (Deprecated)

```json
{
  "tauri": {
    "windows": [
      {
        "fileDropEnabled": true // ← Old v1 setting
      }
    ]
  }
}
```

### v2 Configuration (Current)

```json
{
  "app": {
    "windows": [
      {
        "dragDropEnabled": false // ← New v2 setting (note: false!)
      }
    ]
  }
}
```

## Implementation Example

### Dropzone.js Integration

When using Dropzone.js with Tauri v2, ensure your configuration includes:

```typescript
// Prevent default drag behaviors on the document
const preventDefaults = (e: DragEvent) => {
  e.preventDefault();
  e.stopPropagation();
};

// Add event listeners to prevent default drag behaviors
["dragenter", "dragover", "dragleave", "drop"].forEach((eventName) => {
  document.addEventListener(
    eventName,
    (e: Event) => {
      preventDefaults(e as DragEvent);
    },
    false,
  );
});

// Configure Dropzone
const dropzoneInstance = new Dropzone(dropzoneContainer, {
  url: "/api/upload",
  paramName: "file",
  maxFilesize: 100, // MB
  maxFiles: 10,
  acceptedFiles: ".csv", // Only allow specific file types
  addRemoveLinks: true,
  autoProcessQueue: false, // Let user control upload timing
  // ... other configuration
});
```

### Event Handling

Add proper event listeners for debugging and functionality:

```typescript
dropzone.on("dragenter", (e: any) => {
  console.log("🎯 Dropzone dragenter event");
});

dropzone.on("dragover", (e: any) => {
  console.log("🎯 Dropzone dragover event");
});

dropzone.on("dragleave", (e: any) => {
  console.log("🎯 Dropzone dragleave event");
});

dropzone.on("drop", (e: any) => {
  console.log("🎯 Dropzone drop event");
});
```

## Troubleshooting

### Common Issues

1. **Drag and Drop Not Working**
   - Ensure `dragDropEnabled: false` in `tauri.conf.json`
   - Restart the Tauri application after configuration changes
   - Check browser console for JavaScript errors

2. **Files Not Being Accepted**
   - Verify `acceptedFiles` configuration in Dropzone
   - Check file type validation logic
   - Ensure proper MIME type checking

3. **Events Not Firing**
   - Add document-level event listeners to prevent default behaviors
   - Check that Dropzone is properly initialized
   - Verify event listener cleanup in component destruction

### Debug Console Output

When working correctly, you should see:

```
🖱️ Document dragenter event triggered
🖱️ Document dragover event triggered
🎯 Dropzone dragenter event
🎯 Dropzone dragover event
🔍 Validating file: example.csv
📁 File path: example.csv
📊 File size: 1234 bytes
📄 File type: text/csv
✅ CSV file accepted: example.csv
📥 File added to dropzone: example.csv
```

## Best Practices

1. **Configuration**: Always set `dragDropEnabled: false` for web-based drag and drop
2. **Event Prevention**: Prevent default drag behaviors at the document level
3. **File Validation**: Implement proper file type and size validation
4. **User Feedback**: Provide clear visual feedback during drag operations
5. **Error Handling**: Implement comprehensive error handling and user messaging
6. **Cleanup**: Properly remove event listeners on component destruction

## Platform Considerations

- **Windows**: Works consistently with `dragDropEnabled: false`
- **macOS**: Same configuration applies
- **Linux**: Same configuration applies

## Related Documentation

- [Tauri v2 Migration Guide](https://tauri.app/v2/guides/upgrade-from-v1/)
- [Dropzone.js Documentation](https://docs.dropzone.dev/)
- [Tauri Window Configuration](https://tauri.app/v2/api/config/#windowconfig)
