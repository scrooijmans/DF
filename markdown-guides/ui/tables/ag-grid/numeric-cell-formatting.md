# Numeric Cell Formatting in AG-Grid Tables | MudRock

## Overview

This document describes the implementation of consistent numeric cell formatting across all AG-Grid tables in MudRock's frontend components. The formatting ensures uniform display of numeric values with proper decimal precision, type-aware editing, and consistent user experience.

## Architecture

### 🏗️ **Common Formatter Service**

**Location**: `src/lib/services/ag-grid-cell-formatters.ts`

This centralized service provides consistent cell formatting utilities for all AG-Grid components, eliminating code duplication and ensuring uniform behavior.

### 🎯 **Key Features**

- **Consistent Decimal Precision**: All numeric values display with 3 decimal places (e.g., `11.450`, `11.000`)
- **Null Value Handling**: Null/undefined values display as "-" for better UX
- **Type-Aware Editing**: Different cell editors based on data types (number, text, boolean, date)
- **Input Validation**: Invalid inputs revert to previous values with proper error handling
- **Filter Integration**: Filter values use the same formatting as cell values
- **Automatic Refresh**: Cells refresh after editing to ensure formatting consistency

## Implementation Details

### **Core Formatting Functions**

#### `formatNumericValue(value, decimalPlaces, showNullAsDash)`

```typescript
// Formats numeric values with consistent decimal places
formatNumericValue(11.45); // Returns "11.450"
formatNumericValue(null); // Returns "-"
formatNumericValue("invalid"); // Returns "-"
```

#### `parseNumericValue(newValue, oldValue, allowNegative, minValue)`

```typescript
// Validates and parses numeric input with fallback
parseNumericValue("12.345", 0); // Returns 12.345
parseNumericValue("invalid", 0); // Returns 0 (oldValue)
parseNumericValue("-5", 0, false); // Returns 0 (negative not allowed)
```

### **Column Definition Generators**

#### `createNumericColumnDefinition(field, headerName, config)`

Creates a complete numeric column definition with:

- `cellDataType: "number"`
- `valueFormatter`: 3-decimal display formatting
- `valueParser`: Input validation with fallback
- `cellEditor: "agNumberCellEditor"` with precision control
- `filterParams`: Consistent filter formatting

#### `createParquetColumnDefinitions(columns)`

Generates column definitions for Parquet data:

- **First column** (typically `well_name`): Text column
- **All other columns**: Numeric columns with 3 decimal places

#### `createLasCurveColumnDefinitions(curves)`

Generates column definitions for LAS curve data:

- **All curve columns**: Numeric with 3 decimal places
- **No negative values**: `allowNegative: false`
- **Minimum value**: `minValue: 0`

## Component Integration

### **1. Parquet Analytics Component**

**Location**: `src/lib/components/pages/home/content-main/content-data-analytics/content-data-analytics-file-content/content-data-analytics-file-content-AG-data-table/content-data-analytics-file-content-AG-data-table.svelte`

#### **Implementation**:

```typescript
// Import common formatter
import {
  createParquetColumnDefinitions,
  handleCellValueChange,
  refreshGridCells,
} from "$lib/services/ag-grid-cell-formatters";

// Generate column definitions using common formatter
columnDefs = generateColumnDefsFromParquetSchema(columns);

// Handle cell value changes with common error handling
onCellValueChanged: (event) => {
  handleCellValueChange(
    event,
    gridApi,
    async (event) => {
      // Custom logic for Parquet data updates
      await handleCellEdit(event);
    },
    (event, errorMessage) => {
      error = errorMessage;
    },
  );
};
```

#### **Features**:

- **First column** (`well_name`): Text formatting
- **All other columns**: Numeric with 3 decimal places
- **Editable cells**: Users can edit values with validation
- **Consistent display**: All numeric values show 3 decimal places
- **Error handling**: Invalid inputs revert to previous values

### **2. LAS Upload Confirm Component**

**Location**: `src/lib/components/pages/home/content-main/content-data-ingestion/content-data-las-upload-confirm-AG-data-table/content-data-las-upload-confirm-AG-data-table.svelte`

#### **Implementation**:

```typescript
// Import common formatter
import {
  createLasCurveColumnDefinitions,
  handleCellValueChange,
  refreshGridCells,
} from "$lib/services/ag-grid-cell-formatters";

// Generate column definitions using common formatter
const baseColumnDefs = createLasCurveColumnDefinitions(
  lasProcessingResult.curve_data,
);

// Return column definitions with common formatter
return baseColumnDefs.map((colDef) => ({
  ...colDef,
  cellRenderer: (params: any) => {
    // Use common formatter for data cells
    return colDef.valueFormatter ? colDef.valueFormatter(params) : params.value;
  },
}));
```

#### **Features**:

- **All curve columns**: Numeric with 3 decimal places
- **Data rows**: Consistent numeric formatting
- **No negative values**: LAS curve data validation
- **Editable data**: Users can edit curve values
- **Note**: Curve type and unit information is now handled separately in the curve mapping component (`curve-mapping.svelte`), not displayed in the data table

## Configuration Options

### **Default Configuration**

```typescript
const DEFAULT_CONFIG = {
  decimalPlaces: 3, // 3 decimal places for all numeric values
  allowNegative: true, // Allow negative values (except LAS curves)
  minValue: 0, // Minimum allowed value
  maxTextLength: 50, // Maximum text input length
  showNullAsDash: true, // Show null values as "-"
};
```

### **Custom Configuration Examples**

```typescript
// LAS curve data (no negatives, min value 0)
createLasCurveColumnDefinitions(curves, {
  allowNegative: false,
  minValue: 0,
});

// Custom decimal places
createNumericColumnDefinition("field", "Header", {
  decimalPlaces: 2,
});

// Allow negatives with custom minimum
createNumericColumnDefinition("field", "Header", {
  allowNegative: true,
  minValue: -100,
});
```

## Data Flow

### **1. Data Loading**

```
Component loads data
    ↓
Column definitions generated using common formatter
    ↓
AG-Grid renders with formatted columns
```

### **2. Cell Editing**

```
User edits cell
    ↓
valueParser validates input
    ↓
Invalid input → revert to oldValue
Valid input → update cell
    ↓
refreshGridCells() ensures formatting applied
```

### **3. Filtering**

```
User applies filter
    ↓
filterParams.valueFormatter formats filter values
    ↓
Consistent display in filter dropdown
```

## Key Benefits

### **Consistency**

- All numeric values display with identical formatting
- Uniform user experience across all AG-Grid tables
- Consistent error handling and validation

### **Maintainability**

- Single source of truth for formatting logic
- Easy to update formatting rules globally
- No code duplication between components

### **User Experience**

- Clear visual feedback for null values ("-")
- Intuitive editing with proper validation
- Consistent filter behavior

### **Type Safety**

- Proper TypeScript interfaces
- Runtime validation with fallbacks
- Error handling with user feedback

## Usage Examples

### **Basic Usage**

```typescript
// For Parquet data
const parquetColDefs = createParquetColumnDefinitions(columns);

// For LAS curve data
const lasColDefs = createLasCurveColumnDefinitions(curves);

// Custom numeric column
const customColDef = createNumericColumnDefinition("field", "Header", {
  decimalPlaces: 2,
  allowNegative: false,
  minValue: 0,
});
```

### **Component Integration**

```typescript
// In component
import {
  createParquetColumnDefinitions,
  handleCellValueChange,
} from "$lib/services/ag-grid-cell-formatters";

// Generate columns
const columnDefs = createParquetColumnDefinitions(columns);

// Handle cell changes
onCellValueChanged: (event) => {
  handleCellValueChange(event, gridApi, onSuccess, onError);
};
```

## File Structure

```
src/lib/services/
├── ag-grid-cell-formatters.ts          # Common formatting utilities
└── parquet-data-service.ts             # Updated to use common formatter

src/lib/components/pages/home/content-main/
├── content-data-analytics/
│   └── content-data-analytics-file-content-AG-data-table/
│       └── content-data-analytics-file-content-AG-data-table.svelte
└── content-data-ingestion/
    └── content-data-las-upload-confirm-AG-data-table/
        └── content-data-las-upload-confirm-AG-data-table.svelte
```

## Future Enhancements

### **Planned Features**

- **Custom decimal places per column**: Allow different precision for different data types
- **Currency formatting**: Add currency symbol and locale support
- **Scientific notation**: Support for very large/small numbers
- **Unit display**: Show units alongside numeric values

### **Performance Improvements**

- **Lazy formatting**: Format values only when displayed
- **Caching**: Cache formatted values for better performance
- **Batch updates**: Optimize multiple cell updates

This implementation provides a robust, maintainable solution for consistent numeric cell formatting across all AG-Grid tables in MudRock, ensuring a professional and uniform user experience.
