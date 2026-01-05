# AG Grid JS: No Pagination with Scrolling

This document explains how AG Grid JS allows for no pagination and just scrolling, using virtual scrolling to efficiently handle large datasets.

## Overview

AG Grid uses **virtual scrolling** (row virtualization) by default, which means:
- Only visible rows are rendered in the DOM
- The grid maintains a scrollable viewport
- As you scroll, rows are dynamically created and destroyed
- No pagination controls needed
- Smooth scrolling through thousands or millions of rows

## Architecture

```
┌─────────────────────────────────────────────────────────────┐
│              AG Grid Viewport                                │
│                                                             │
│  ┌──────────────────────────────────────────────────────┐  │
│  │  Visible Rows (Rendered in DOM)                      │  │
│  │  - Only ~20-50 rows visible at once                  │  │
│  │  - Dynamically created/destroyed on scroll           │  │
│  └──────────────────────────────────────────────────────┘  │
│                          │                                  │
│                          ▼                                  │
│  ┌──────────────────────────────────────────────────────┐  │
│  │  Virtual Scrollbar                                    │  │
│  │  - Represents full dataset height                    │  │
│  │  - User scrolls through all data                     │  │
│  └──────────────────────────────────────────────────────┘  │
│                                                             │
│  ┌──────────────────────────────────────────────────────┐  │
│  │  Row Buffer (Pre-rendered)                           │  │
│  │  - Rows above/below viewport                         │  │
│  │  - Smooth scrolling experience                       │  │
│  └──────────────────────────────────────────────────────┘  │
└─────────────────────────────────────────────────────────────┘
                          │
                          ▼
┌─────────────────────────────────────────────────────────────┐
│              Data Source                                     │
│              - All data in memory (Client-side)             │
│              - Or lazy-loaded (Infinite/Server-side)       │
└─────────────────────────────────────────────────────────────┘
```

## Basic Configuration: Disable Pagination

### Simple Configuration

The simplest way to enable scrolling without pagination is to **not set pagination** or explicitly set it to `false`:

```javascript
import { createGrid } from 'ag-grid-community';

const columnDefs = [
  { field: 'id' },
  { field: 'name' },
  { field: 'email' },
  { field: 'status' }
];

const gridOptions = {
  columnDefs: columnDefs,
  rowData: [], // Will be populated with data
  pagination: false, // Disable pagination - use scrolling instead
  defaultColDef: {
    sortable: true,
    filter: true,
    resizable: true
  }
};

const eGridDiv = document.querySelector('#myGrid');
const gridApi = createGrid(eGridDiv, gridOptions);

// Load data - all rows will be scrollable
fetch('/api/data')
  .then(resp => resp.json())
  .then(data => {
    gridApi.setGridOption('rowData', data);
  });
```

### Complete Example

```javascript
import { createGrid } from 'ag-grid-community';
import 'ag-grid-community/styles/ag-grid.css';
import 'ag-grid-community/styles/ag-theme-quartz.css';

const columnDefs = [
  { field: 'id', headerName: 'ID', width: 100 },
  { field: 'name', headerName: 'Name', flex: 1 },
  { field: 'email', headerName: 'Email', flex: 1 },
  { field: 'status', headerName: 'Status', width: 150 }
];

const gridOptions = {
  columnDefs: columnDefs,
  rowData: [],
  
  // Disable pagination - use scrolling
  pagination: false,
  
  // Virtual scrolling is enabled by default
  // These settings optimize performance
  rowBuffer: 10, // Number of rows to render outside viewport
  suppressColumnVirtualisation: false, // Enable column virtualization
  suppressRowVirtualisation: false, // Enable row virtualization (default)
  
  // Grid height required for scrolling
  domLayout: 'normal', // 'normal' or 'autoHeight'
  
  defaultColDef: {
    sortable: true,
    filter: true,
    resizable: true
  },
  
  onGridReady: (params) => {
    // Load all data at once
    fetch('https://api.example.com/users')
      .then(resp => resp.json())
      .then(data => {
        params.api.setGridOption('rowData', data);
        console.log(`Loaded ${data.length} rows - all scrollable`);
      });
  }
};

const eGridDiv = document.querySelector('#myGrid');
const gridApi = createGrid(eGridDiv, gridOptions);
```

**HTML**:

```html
<div id="myGrid" style="height: 600px; width: 100%;"></div>
```

## Row Models for Scrolling

AG Grid supports different row models, each optimized for different use cases:

### 1. Client-Side Row Model (Default)

**Best for**: Small to medium datasets (up to ~100,000 rows)

**Features**:
- All data loaded in memory
- Instant filtering, sorting, grouping
- Virtual scrolling for performance
- No server round-trips

**Configuration**:

```javascript
const gridOptions = {
  // rowModelType: 'clientSide' is the default
  rowModelType: 'clientSide', // Optional - this is default
  rowData: allData, // All data in memory
  pagination: false, // Scrolling instead of pagination
  rowBuffer: 10 // Rows to render outside viewport
};
```

**Example**:

```javascript
const gridOptions = {
  columnDefs: [
    { field: 'id' },
    { field: 'name' },
    { field: 'email' }
  ],
  rowData: [
    { id: 1, name: 'John', email: 'john@example.com' },
    { id: 2, name: 'Jane', email: 'jane@example.com' },
    // ... thousands more rows
  ],
  pagination: false, // All rows scrollable
  rowBuffer: 20 // Render 20 rows outside viewport for smooth scrolling
};

const gridApi = createGrid(document.querySelector('#myGrid'), gridOptions);
```

### 2. Infinite Row Model

**Best for**: Large datasets that need lazy loading

**Features**:
- Lazy-loads rows as user scrolls
- Auto-extending scrollbar
- Loads data in blocks
- Good for very large datasets

**Configuration**:

```javascript
const gridOptions = {
  rowModelType: 'infinite',
  pagination: false, // No pagination - infinite scroll
  
  // Configure datasource
  datasource: {
    getRows: (params) => {
      console.log('Loading rows:', params.startRow, 'to', params.endRow);
      
      // Fetch data from server
      fetch(`/api/data?start=${params.startRow}&end=${params.endRow}`)
        .then(resp => resp.json())
        .then(data => {
          // Call success callback with data
          params.successCallback(data.rows, data.lastRow);
        })
        .catch(err => {
          params.failCallback();
        });
    }
  },
  
  // Infinite scroll settings
  cacheBlockSize: 100, // Load 100 rows at a time
  maxBlocksInCache: 10, // Keep 10 blocks in cache
  blockLoadDebounceMillis: 0, // No debounce
  overflowSize: 100, // Extra rows to load before reaching end
  initialRowCount: 1 // Initial number of rows
};
```

**Complete Infinite Scroll Example**:

```javascript
import { createGrid } from 'ag-grid-community';

const columnDefs = [
  { field: 'id', headerName: 'ID', width: 100 },
  { field: 'name', headerName: 'Name', flex: 1 },
  { field: 'email', headerName: 'Email', flex: 1 }
];

let gridApi;

const gridOptions = {
  columnDefs: columnDefs,
  rowModelType: 'infinite',
  pagination: false, // No pagination - infinite scrolling
  
  datasource: {
    getRows: (params) => {
      const startRow = params.startRow;
      const endRow = params.endRow;
      
      console.log(`Loading rows ${startRow} to ${endRow}`);
      
      // Simulate API call
      fetch(`/api/users?start=${startRow}&end=${endRow}`)
        .then(resp => resp.json())
        .then(data => {
          // Determine if this is the last page
          const lastRow = data.totalRows ? data.totalRows : undefined;
          
          // Provide data to grid
          params.successCallback(data.rows, lastRow);
        })
        .catch(err => {
          console.error('Error loading data:', err);
          params.failCallback();
        });
    }
  },
  
  // Infinite scroll configuration
  cacheBlockSize: 50, // Load 50 rows per request
  maxBlocksInCache: 5, // Keep 5 blocks (250 rows) in cache
  overflowSize: 50, // Load 50 extra rows before reaching end
  initialRowCount: 1, // Start with 1 row to trigger initial load
  rowBuffer: 10 // Render 10 rows outside viewport
};

const eGridDiv = document.querySelector('#myGrid');
gridApi = createGrid(eGridDiv, gridOptions);
```

**Server-Side Implementation**:

```javascript
// Backend API endpoint
app.get('/api/users', (req, res) => {
  const start = parseInt(req.query.start) || 0;
  const end = parseInt(req.query.end) || 50;
  const limit = end - start;
  
  // Query database
  db.query(
    'SELECT * FROM users ORDER BY id LIMIT ? OFFSET ?',
    [limit, start],
    (err, rows) => {
      if (err) {
        return res.status(500).json({ error: err.message });
      }
      
      // Get total count
      db.query('SELECT COUNT(*) as total FROM users', (err, countResult) => {
        const total = countResult[0].total;
        
        res.json({
          rows: rows,
          totalRows: total
        });
      });
    }
  );
});
```

### 3. Server-Side Row Model (Enterprise)

**Best for**: Very large datasets with server-side filtering/sorting

**Features**:
- Server-side filtering, sorting, grouping
- Lazy loading with caching
- More features than Infinite Row Model
- Enterprise feature

**Configuration**:

```javascript
const gridOptions = {
  rowModelType: 'serverSide',
  pagination: false, // No pagination - scrolling only
  
  serverSideDatasource: {
    getRows: (params) => {
      const request = {
        startRow: params.request.startRow,
        endRow: params.request.endRow,
        filterModel: params.request.filterModel,
        sortModel: params.request.sortModel,
        groupKeys: params.request.groupKeys
      };
      
      fetch('/api/grid-data', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify(request)
      })
        .then(resp => resp.json())
        .then(data => {
          params.success({
            rowData: data.rows,
            rowCount: data.totalRows
          });
        })
        .catch(err => {
          params.fail();
        });
    }
  },
  
  // Server-side settings
  cacheBlockSize: 100,
  maxBlocksInCache: 10,
  serverSideInitialRowCount: 1
};
```

## Virtual Scrolling Configuration

### Row Buffer

Controls how many rows are rendered outside the visible viewport:

```javascript
const gridOptions = {
  pagination: false,
  rowBuffer: 20, // Render 20 rows above and below viewport
  // Higher values = smoother scrolling but more DOM nodes
  // Lower values = fewer DOM nodes but potential stuttering
};
```

### Column Virtualization

Virtual scrolling for columns (useful for many columns):

```javascript
const gridOptions = {
  pagination: false,
  suppressColumnVirtualisation: false, // Enable column virtualization
  // Only visible columns are rendered
};
```

### Row Virtualization

Row virtualization is enabled by default. You can disable it (not recommended for large datasets):

```javascript
const gridOptions = {
  pagination: false,
  suppressRowVirtualisation: false, // Keep enabled (default)
  // If set to true, all rows rendered (bad for performance)
};
```

## Performance Optimization

### For Large Datasets

```javascript
const gridOptions = {
  columnDefs: columnDefs,
  rowData: largeDataset, // 100,000+ rows
  pagination: false,
  
  // Performance optimizations
  rowBuffer: 10, // Smaller buffer for large datasets
  suppressColumnVirtualisation: false, // Enable column virtualization
  suppressRowVirtualisation: false, // Keep row virtualization enabled
  debounceVerticalScrollbar: true, // Debounce scrollbar updates
  suppressScrollOnNewData: false, // Don't scroll on data updates
  
  // Reduce re-renders
  suppressRowHoverHighlight: false,
  suppressCellFocus: false,
  
  // Optimize rendering
  animateRows: false, // Disable row animations for performance
  suppressMovableColumns: true, // Disable column moving if not needed
  suppressMenuHide: true, // Keep menus open for better UX
};
```

### React Example with Performance

```jsx
import React, { useMemo, useRef } from 'react';
import { AgGridReact } from 'ag-grid-react';
import 'ag-grid-community/styles/ag-grid.css';
import 'ag-grid-community/styles/ag-theme-quartz.css';

function LargeDataGrid() {
  const gridRef = useRef();
  const [rowData, setRowData] = React.useState([]);
  
  const columnDefs = useMemo(() => [
    { field: 'id', width: 100 },
    { field: 'name', flex: 1 },
    { field: 'email', flex: 1 },
    { field: 'status', width: 150 }
  ], []);
  
  const defaultColDef = useMemo(() => ({
    sortable: true,
    filter: true,
    resizable: true
  }), []);
  
  const gridOptions = useMemo(() => ({
    pagination: false, // No pagination - scrolling only
    rowBuffer: 10, // Optimize for large datasets
    suppressColumnVirtualisation: false,
    animateRows: false, // Better performance
    debounceVerticalScrollbar: true
  }), []);
  
  React.useEffect(() => {
    // Load all data
    fetch('/api/large-dataset')
      .then(resp => resp.json())
      .then(data => {
        setRowData(data);
        console.log(`Loaded ${data.length} rows - all scrollable`);
      });
  }, []);
  
  return (
    <div className="ag-theme-quartz" style={{ height: 600, width: '100%' }}>
      <AgGridReact
        ref={gridRef}
        columnDefs={columnDefs}
        rowData={rowData}
        defaultColDef={defaultColDef}
        {...gridOptions}
      />
    </div>
  );
}
```

## Height Configuration

For scrolling to work, the grid must have a **fixed height**:

### Fixed Height

```javascript
const gridOptions = {
  pagination: false
};

// HTML
<div id="myGrid" style="height: 600px; width: 100%;"></div>
```

### Full Viewport Height

```javascript
const gridOptions = {
  pagination: false
};

// HTML with CSS
<div id="myGrid" style="height: 100vh; width: 100%;"></div>
```

### Auto Height (No Scrolling)

If you want all rows visible without scrolling:

```javascript
const gridOptions = {
  pagination: false,
  domLayout: 'autoHeight' // All rows visible, no scrolling
};

// Grid height adjusts to content
// Not recommended for large datasets
```

## Comparison: Pagination vs. Scrolling

### With Pagination

```javascript
const gridOptions = {
  pagination: true,
  paginationPageSize: 20,
  // User clicks through pages
  // Only 20 rows visible at a time
};
```

**Pros**:
- Predictable memory usage
- Easy to navigate to specific pages
- Good for very large datasets

**Cons**:
- Requires clicking through pages
- Can't see all data at once
- Slower navigation

### Without Pagination (Scrolling)

```javascript
const gridOptions = {
  pagination: false,
  // All rows scrollable
  // Virtual scrolling handles performance
};
```

**Pros**:
- Smooth scrolling experience
- Can see all data by scrolling
- Faster navigation
- Better UX for most users

**Cons**:
- All data must be loaded (client-side model)
- Or requires infinite scroll setup
- More memory usage (client-side model)

## Complete Example: No Pagination with Scrolling

```javascript
import { createGrid } from 'ag-grid-community';
import 'ag-grid-community/styles/ag-grid.css';
import 'ag-grid-community/styles/ag-theme-quartz.css';

// Column definitions
const columnDefs = [
  { 
    field: 'id', 
    headerName: 'ID', 
    width: 100,
    sortable: true,
    filter: 'agNumberColumnFilter'
  },
  { 
    field: 'name', 
    headerName: 'Name', 
    flex: 1,
    sortable: true,
    filter: 'agTextColumnFilter'
  },
  { 
    field: 'email', 
    headerName: 'Email', 
    flex: 1,
    sortable: true,
    filter: 'agTextColumnFilter'
  },
  { 
    field: 'status', 
    headerName: 'Status', 
    width: 150,
    sortable: true,
    filter: 'agSetColumnFilter'
  },
  { 
    field: 'created', 
    headerName: 'Created', 
    width: 200,
    sortable: true,
    filter: 'agDateColumnFilter',
    valueFormatter: (params) => {
      return new Date(params.value).toLocaleDateString();
    }
  }
];

// Grid options
const gridOptions = {
  columnDefs: columnDefs,
  rowData: [],
  
  // Disable pagination - use scrolling
  pagination: false,
  
  // Virtual scrolling configuration
  rowBuffer: 20, // Render 20 rows outside viewport
  suppressColumnVirtualisation: false, // Enable column virtualization
  suppressRowVirtualisation: false, // Enable row virtualization (default)
  
  // Performance optimizations
  animateRows: false, // Disable animations for better performance
  debounceVerticalScrollbar: true, // Debounce scrollbar updates
  
  // Default column properties
  defaultColDef: {
    sortable: true,
    filter: true,
    resizable: true,
    floatingFilter: true // Show filter inputs in header
  },
  
  // Enable row selection
  rowSelection: 'multiple',
  suppressRowClickSelection: false,
  
  // Events
  onGridReady: (params) => {
    console.log('Grid ready');
    
    // Load data
    loadData(params.api);
  },
  
  onFirstDataRendered: (params) => {
    console.log('First data rendered');
    params.api.sizeColumnsToFit();
  },
  
  onRowDataChanged: (params) => {
    console.log(`Row data changed: ${params.api.getDisplayedRowCount()} rows`);
  },
  
  onScroll: (params) => {
    // Optional: Handle scroll events
    const scrollTop = params.top;
    console.log('Scrolled to:', scrollTop);
  }
};

// Load data function
function loadData(api) {
  fetch('/api/users')
    .then(resp => resp.json())
    .then(data => {
      api.setGridOption('rowData', data);
      console.log(`Loaded ${data.length} rows - all scrollable`);
    })
    .catch(err => {
      console.error('Error loading data:', err);
    });
}

// Create grid
const eGridDiv = document.querySelector('#myGrid');
const gridApi = createGrid(eGridDiv, gridOptions);

// Export grid API for external use
window.gridApi = gridApi;
```

**HTML**:

```html
<!DOCTYPE html>
<html>
<head>
  <link rel="stylesheet" href="ag-grid-community/styles/ag-grid.css">
  <link rel="stylesheet" href="ag-grid-community/styles/ag-theme-quartz.css">
</head>
<body>
  <div id="myGrid" style="height: 600px; width: 100%;"></div>
  <script type="module" src="grid.js"></script>
</body>
</html>
```

## Infinite Scroll Example

For very large datasets, use infinite scrolling:

```javascript
import { createGrid } from 'ag-grid-community';

const columnDefs = [
  { field: 'id', headerName: 'ID', width: 100 },
  { field: 'name', headerName: 'Name', flex: 1 },
  { field: 'email', headerName: 'Email', flex: 1 }
];

let gridApi;

const gridOptions = {
  columnDefs: columnDefs,
  rowModelType: 'infinite',
  pagination: false, // No pagination - infinite scrolling
  
  datasource: {
    getRows: (params) => {
      const { startRow, endRow } = params;
      
      console.log(`Loading rows ${startRow} to ${endRow}`);
      
      // Fetch from API
      fetch(`/api/users?start=${startRow}&end=${endRow}`)
        .then(resp => resp.json())
        .then(data => {
          // Check if this is the last page
          const lastRow = data.totalRows ? data.totalRows : undefined;
          
          // Provide data to grid
          params.successCallback(data.rows, lastRow);
        })
        .catch(err => {
          console.error('Error:', err);
          params.failCallback();
        });
    }
  },
  
  // Infinite scroll settings
  cacheBlockSize: 50, // Load 50 rows per request
  maxBlocksInCache: 5, // Keep 5 blocks in cache
  overflowSize: 50, // Load 50 extra rows before end
  initialRowCount: 1, // Start with 1 row
  rowBuffer: 10 // Render 10 rows outside viewport
};

const eGridDiv = document.querySelector('#myGrid');
gridApi = createGrid(eGridDiv, gridOptions);
```

## API Methods for Scrolling

### Programmatic Scrolling

```javascript
// Scroll to specific row
gridApi.ensureIndexVisible(100); // Scroll to row index 100

// Scroll to top
gridApi.ensureIndexVisible(0);

// Scroll to bottom
const rowCount = gridApi.getDisplayedRowCount();
gridApi.ensureIndexVisible(rowCount - 1);

// Get current scroll position
const scrollTop = gridApi.getVerticalPixelRange().top;
console.log('Current scroll position:', scrollTop);
```

### Scroll Events

```javascript
const gridOptions = {
  pagination: false,
  
  onScroll: (params) => {
    const scrollTop = params.top;
    const scrollLeft = params.left;
    console.log('Scrolled:', { scrollTop, scrollLeft });
  },
  
  onBodyScroll: (params) => {
    // More detailed scroll information
    console.log('Body scrolled:', params);
  }
};
```

## Best Practices

### 1. Always Set Grid Height

```javascript
// ✅ Good - Fixed height enables scrolling
<div id="myGrid" style="height: 600px;"></div>

// ❌ Bad - No height means no scrolling
<div id="myGrid"></div>
```

### 2. Use Row Buffer Appropriately

```javascript
// Small dataset (< 1,000 rows)
rowBuffer: 20

// Medium dataset (1,000 - 10,000 rows)
rowBuffer: 10

// Large dataset (> 10,000 rows)
rowBuffer: 5
```

### 3. Disable Animations for Performance

```javascript
const gridOptions = {
  pagination: false,
  animateRows: false, // Better performance
  animateColumns: false // Better performance
};
```

### 4. Use Infinite Scroll for Very Large Datasets

```javascript
// For datasets > 100,000 rows
rowModelType: 'infinite'

// For datasets < 100,000 rows
rowModelType: 'clientSide' // Default
```

### 5. Optimize Column Definitions

```javascript
// Use flex for responsive columns
{ field: 'name', flex: 1 }

// Use fixed width when possible
{ field: 'id', width: 100 }

// Avoid minWidth/maxWidth when possible
```

## Summary

**To enable scrolling without pagination in AG Grid:**

1. **Set `pagination: false`** in gridOptions
2. **Set a fixed height** on the grid container
3. **Virtual scrolling is enabled by default** - no additional configuration needed
4. **For large datasets**, consider:
   - Infinite Row Model for lazy loading
   - Server-Side Row Model for server-side operations (Enterprise)
   - Optimize rowBuffer and disable animations

**Key Points**:
- ✅ Virtual scrolling renders only visible rows
- ✅ Smooth scrolling through thousands of rows
- ✅ No pagination controls needed
- ✅ Better UX than pagination for most use cases
- ✅ Automatic performance optimization

**Row Models**:
- **Client-Side**: All data in memory, best for < 100K rows
- **Infinite**: Lazy loading, best for very large datasets
- **Server-Side**: Enterprise feature with server-side operations

AG Grid's virtual scrolling makes it possible to scroll through millions of rows efficiently without pagination, providing a smooth user experience while maintaining excellent performance.

