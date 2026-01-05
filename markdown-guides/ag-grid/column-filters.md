# AG-Grid Column Filters Configuration

This document describes how column filters are configured in the MudRock application following the official AG-Grid documentation patterns.

## Overview

The column filters are implemented using AG-Grid's built-in filter system, with type-specific filters applied based on the database column data types. Filters are integrated directly into column headers, providing a clean and professional user experience.

## Implementation Location

The column filter configuration is handled in:

- **`src/lib/tauri-commands/schema-fetching.ts`** - Column definition generation with filter types
- **`src/lib/components/pages/home/content-main/content-data/AG-data-table/AG-data-table.svelte`** - Grid options and default column configuration

## Filter Types by Data Type

### 1. Text/String Columns

**Data Types**: `text`, `character varying`
**Filter Type**: `agTextColumnFilter`
**Configuration**:

```typescript
{
  filter: "agTextColumnFilter",
  filterParams: {
    debounceMs: 200,
  },
}
```

**Features**:

- Text search with 200ms debounce
- Case-insensitive matching
- Supports partial string matching

### 2. Number Columns

**Data Types**: `integer`, `bigint`, `smallint`, `numeric`, `real`, `double precision`
**Filter Type**: `agNumberColumnFilter`
**Configuration**:

```typescript
{
  filter: "agNumberColumnFilter",
  filterParams: {
    debounceMs: 200,
  },
}
```

**Features**:

- Numeric range filtering
- Greater than, less than, equals comparisons
- Supports decimal numbers for `numeric` types

### 3. Date/Timestamp Columns

**Data Types**: `timestamp with time zone`, `timestamp without time zone`, `date`
**Filter Type**: `agDateColumnFilter`
**Configuration**:

```typescript
{
  filter: "agDateColumnFilter",
  filterParams: {
    comparator: (filterLocalDateAtMidnight: Date, cellValue: string) => {
      const dateAsString = cellValue;
      if (dateAsString == null) return -1;
      const cellDate = new Date(dateAsString);
      if (filterLocalDateAtMidnight.getTime() === cellDate.getTime()) {
        return 0;
      }
      if (cellDate < filterLocalDateAtMidnight) {
        return -1;
      }
      if (cellDate > filterLocalDateAtMidnight) {
        return 1;
      }
      return 0;
    },
  },
}
```

**Features**:

- Date range filtering
- Custom comparator for proper date comparison
- Supports various date formats from database

### 4. Boolean Columns

**Data Types**: `boolean`
**Filter Type**: `agSetColumnFilter`
**Configuration**:

```typescript
{
  filter: "agSetColumnFilter",
  filterParams: {
    values: ["Yes", "No"],
    valueFormatter: (params: any) => {
      if (params.value === null || params.value === undefined)
        return "-";
      return params.value ? "Yes" : "No";
    },
  },
}
```

**Features**:

- Dropdown selection with "Yes"/"No" options
- Custom value formatter for display
- Handles null/undefined values gracefully

### 5. Default/Unknown Columns

**Data Types**: Any other type not specifically handled
**Filter Type**: `agTextColumnFilter`
**Configuration**:

```typescript
{
  filter: "agTextColumnFilter",
  filterParams: {
    debounceMs: 200,
  },
}
```

**Features**:

- Fallback to text filter for unknown types
- Handles JSON, JSONB, and other complex types
- Converts values to strings for filtering

## Grid Configuration

### Default Column Definition

```typescript
defaultColDef: {
  enableCellChangeFlash: true,
  suppressMovable: true,
  resizable: true,
  sortable: true,
  editable: true,
  flex: 1,
  minWidth: 150,
  filter: "agTextColumnFilter", // Default filter for all columns
  suppressHeaderMenuButton: false,
  suppressHeaderContextMenu: false,
}
```

### Column Header Configuration

- **`suppressHeaderMenuButton: false`** - Enables the filter menu button in column headers
- **`suppressHeaderContextMenu: false`** - Enables right-click context menu on headers
- **`flex: 1`** - Makes columns responsive to available space
- **`minWidth: 150`** - Ensures minimum column width for usability

## Filter Access Methods

### 1. Column Header Menu

- Click the filter icon (🔽) in any column header
- Access filter controls directly from the header
- Most common method for users

### 2. Column Context Menu

- Right-click on any column header
- Access additional column options including filters
- Provides more advanced column management

### 3. Tool Panel (if enabled)

- Access filters through the AG-Grid tool panel
- Provides centralized filter management
- Useful for complex filtering scenarios

## Filter Behavior

### Debounce Configuration

All text and number filters use a 200ms debounce to:

- Reduce server load during typing
- Improve performance with large datasets
- Provide smooth user experience

### Value Formatting

- **Dates**: Formatted as `MM/DD/YYYY` for display
- **Numbers**: Formatted with appropriate decimal places
- **Booleans**: Displayed as "Yes"/"No" instead of true/false
- **Null values**: Displayed as "-" for better UX

### Filter Persistence

- Filters are maintained during grid operations
- Survive data refreshes and updates
- Can be cleared individually or all at once

## Integration with Database Schema

The filter configuration is automatically generated based on the database schema:

1. **Schema Fetching**: Table schema is fetched from Supabase
2. **Type Detection**: Column data types are analyzed
3. **Filter Assignment**: Appropriate filter type is assigned based on data type
4. **Configuration**: Filter parameters are set with optimal defaults

## Example Usage

### Accessing Filters

1. Navigate to the Data section in the application
2. Select a table from the table list
3. Click the filter icon in any column header
4. Use the filter controls to filter data
5. Clear filters using the "Clear" button

### Filter Types in Action

- **Text columns**: Type to search for partial matches
- **Number columns**: Set ranges using the number input controls
- **Date columns**: Use the date picker for date ranges
- **Boolean columns**: Select from "Yes"/"No" dropdown

## Performance Considerations

- **Debounce**: 200ms delay prevents excessive filtering during typing
- **Client-side**: All filtering happens on the client side for fast response
- **Memory efficient**: Only visible data is processed for filtering
- **Lazy loading**: Filters are applied only when data is displayed

## Customization

To add new filter types or modify existing ones:

1. **Update `schema-fetching.ts`**: Add new data type cases in the `generateColumnDefsFromSchema` function
2. **Configure filter parameters**: Adjust `filterParams` for specific needs
3. **Add value formatters**: Customize how data is displayed in filter controls
4. **Test thoroughly**: Ensure filters work correctly with your data types

## Troubleshooting

### Common Issues

- **Filter not appearing**: Check that `suppressHeaderMenuButton: false` is set
- **Date filter not working**: Verify the comparator function handles your date format
- **Boolean filter showing raw values**: Ensure `valueFormatter` is properly configured
- **Performance issues**: Increase `debounceMs` value for slower systems

### Debug Tips

- Check browser console for filter-related errors
- Verify column data types match expected filter types
- Test with different data values to ensure proper filtering
- Use AG-Grid's built-in debugging tools for filter inspection

This implementation provides a robust, type-aware filtering system that automatically adapts to your database schema while following AG-Grid best practices.
