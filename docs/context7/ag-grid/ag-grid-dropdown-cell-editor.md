# AG Grid Dropdown Options in Cells

This document explains how AG Grid JS allows for dropdown options in cells using built-in cell editors and custom implementations.

## Overview

AG Grid provides multiple ways to add dropdown options to cells:

1. **Built-in Select Cell Editor** (`agSelectCellEditor`) - Standard HTML select dropdown
2. **Rich Select Cell Editor** (`agRichSelectCellEditor`) - Enterprise feature with search and filtering
3. **Custom Cell Editors** - Fully customizable dropdown implementations
4. **Dynamic Options** - Dropdown values that change based on other cell values

## Architecture

```
┌─────────────────────────────────────────────────────────────┐
│              AG Grid Cell                                    │
│              (editable: true)                                │
└────────────────────────────┬────────────────────────────────┘
                             │
                             │ User clicks cell
                             ▼
┌─────────────────────────────────────────────────────────────┐
│              Cell Editor (Dropdown)                         │
│              - agSelectCellEditor                            │
│              - agRichSelectCellEditor                        │
│              - Custom Editor                                 │
└────────────────────────────┬────────────────────────────────┘
                             │
                             │ User selects option
                             ▼
┌─────────────────────────────────────────────────────────────┐
│              Value Updated                                   │
│              - onCellValueChanged event                      │
│              - Data model updated                            │
└─────────────────────────────────────────────────────────────┘
```

## 1. Built-in Select Cell Editor (`agSelectCellEditor`)

### Basic Configuration

The `agSelectCellEditor` uses a standard HTML `<select>` element to provide dropdown options.

```javascript
const columnDefs = [
  {
    field: 'language',
    headerName: 'Language',
    editable: true,
    cellEditor: 'agSelectCellEditor',
    cellEditorParams: {
      values: ['English', 'Spanish', 'French', 'Portuguese', '(other)']
    }
  }
]
```

### Complete Example

```javascript
import { createGrid } from 'ag-grid-community'

const columnDefs = [
  { field: 'name', headerName: 'Name' },
  {
    field: 'country',
    headerName: 'Country',
    editable: true,
    cellEditor: 'agSelectCellEditor',
    cellEditorParams: {
      values: ['USA', 'UK', 'France', 'Germany', 'China', 'Japan']
    }
  },
  {
    field: 'status',
    headerName: 'Status',
    editable: true,
    cellEditor: 'agSelectCellEditor',
    cellEditorParams: {
      values: ['Active', 'Inactive', 'Pending', 'Suspended']
    }
  }
]

const rowData = [
  { name: 'John Doe', country: 'USA', status: 'Active' },
  { name: 'Jane Smith', country: 'UK', status: 'Pending' }
]

const gridOptions = {
  columnDefs: columnDefs,
  rowData: rowData,
  defaultColDef: {
    sortable: true,
    filter: true,
    resizable: true
  },
  onCellValueChanged: (event) => {
    console.log('Value changed:', event.data)
  }
}

const eGridDiv = document.querySelector('#myGrid')
const gridApi = createGrid(eGridDiv, gridOptions)
```

### Configuration Options

```javascript
{
  field: 'language',
  editable: true,
  cellEditor: 'agSelectCellEditor',
  cellEditorParams: {
    // Array of values to display in dropdown
    values: ['English', 'Spanish', 'French'],
    
    // Gap between displayed value and dropdown list (pixels)
    valueListGap: 0
  }
}
```

## 2. Rich Select Cell Editor (`agRichSelectCellEditor`)

The `agRichSelectCellEditor` is an Enterprise feature that provides enhanced dropdown functionality with search, filtering, and custom rendering.

### Basic Configuration

```javascript
const columnDefs = [
  {
    field: 'language',
    headerName: 'Language',
    editable: true,
    cellEditor: 'agRichSelectCellEditor',
    cellEditorParams: {
      values: ['English', 'Spanish', 'French', 'Portuguese', '(other)'],
      cellHeight: 20,
      formatValue: (value) => value.toUpperCase(),
      searchDebounceDelay: 500
    }
  }
]
```

### Advanced Configuration with Custom Renderer

```javascript
// Custom cell renderer for dropdown items
class LanguageCellRenderer {
  init(params) {
    this.eGui = document.createElement('div')
    this.eGui.style.display = 'flex'
    this.eGui.style.alignItems = 'center'
    this.eGui.style.gap = '8px'
    
    const flag = this.getFlag(params.value)
    this.eGui.innerHTML = `
      <span>${flag}</span>
      <span>${params.value}</span>
    `
  }

  getGui() {
    return this.eGui
  }

  getFlag(language) {
    const flags = {
      'English': '🇬🇧',
      'Spanish': '🇪🇸',
      'French': '🇫🇷',
      'Portuguese': '🇵🇹'
    }
    return flags[language] || '🌐'
  }

  refresh() {
    return false
  }
}

const columnDefs = [
  {
    field: 'language',
    headerName: 'Language',
    editable: true,
    cellEditor: 'agRichSelectCellEditor',
    cellEditorParams: {
      values: ['English', 'Spanish', 'French', 'Portuguese'],
      cellHeight: 30,
      cellRenderer: LanguageCellRenderer,
      formatValue: (value) => value.toUpperCase(),
      searchDebounceDelay: 300
    }
  }
]
```

### Rich Select Editor Parameters

```javascript
cellEditorParams: {
  // Array of values for dropdown
  values: ['Option 1', 'Option 2', 'Option 3'],
  
  // Height of each dropdown item (pixels)
  cellHeight: 20,
  
  // Custom formatter for displayed values
  formatValue: (value) => value.toUpperCase(),
  
  // Custom cell renderer for dropdown items
  cellRenderer: MyCellRenderer,
  
  // Debounce delay for search (milliseconds)
  searchDebounceDelay: 500,
  
  // Gap between value display and dropdown list
  valueListGap: 0
}
```

## 3. Dynamic Dropdown Options

Dropdown options can be dynamically generated based on other cell values or data.

### Function-Based Parameters

```javascript
const columnDefs = [
  {
    field: 'country',
    headerName: 'Country',
    editable: true,
    cellEditor: 'agSelectCellEditor',
    cellEditorParams: {
      values: ['USA', 'UK', 'France', 'Germany']
    }
  },
  {
    field: 'city',
    headerName: 'City',
    editable: true,
    cellEditor: 'agSelectCellEditor',
    // Dynamic parameters based on country value
    cellEditorParams: (params) => {
      const selectedCountry = params.data.country
      
      const citiesByCountry = {
        'USA': ['New York', 'Los Angeles', 'Chicago', 'Houston'],
        'UK': ['London', 'Manchester', 'Birmingham', 'Liverpool'],
        'France': ['Paris', 'Lyon', 'Marseille', 'Toulouse'],
        'Germany': ['Berlin', 'Munich', 'Hamburg', 'Cologne']
      }
      
      return {
        values: citiesByCountry[selectedCountry] || ['Other']
      }
    }
  }
]
```

### Complete Dynamic Example

```javascript
const gridOptions = {
  columnDefs: [
    {
      field: 'category',
      headerName: 'Category',
      editable: true,
      cellEditor: 'agSelectCellEditor',
      cellEditorParams: {
        values: ['Electronics', 'Clothing', 'Food', 'Books']
      }
    },
    {
      field: 'product',
      headerName: 'Product',
      editable: true,
      cellEditor: 'agSelectCellEditor',
      cellEditorParams: (params) => {
        const category = params.data.category
        
        const productsByCategory = {
          'Electronics': ['Laptop', 'Phone', 'Tablet', 'Monitor'],
          'Clothing': ['Shirt', 'Pants', 'Shoes', 'Jacket'],
          'Food': ['Apple', 'Banana', 'Orange', 'Grapes'],
          'Books': ['Fiction', 'Non-Fiction', 'Biography', 'History']
        }
        
        return {
          values: productsByCategory[category] || []
        }
      }
    }
  ],
  rowData: [
    { category: 'Electronics', product: 'Laptop' },
    { category: 'Clothing', product: 'Shirt' }
  ],
  onCellValueChanged: (event) => {
    // If category changed, reset product
    if (event.column.getColId() === 'category') {
      event.data.product = null
      event.api.refreshCells({ rowNodes: [event.node], columns: ['product'] })
    }
  }
}
```

## 4. Value Mapping and Formatting

When storing codes but displaying user-friendly values, use `valueFormatter` and `valueParser`.

### Key-Value Mapping

```javascript
// Mapping object: code -> display value
const statusMappings = {
  'A': 'Active',
  'I': 'Inactive',
  'P': 'Pending',
  'S': 'Suspended'
}

// Helper functions
const lookupValue = (mappings, key) => {
  return mappings[key] || key
}

const lookupKey = (mappings, value) => {
  const keys = Object.keys(mappings)
  for (let i = 0; i < keys.length; i++) {
    if (mappings[keys[i]] === value) {
      return keys[i]
    }
  }
  return value
}

const columnDefs = [
  {
    field: 'status',
    headerName: 'Status',
    editable: true,
    cellEditor: 'agSelectCellEditor',
    cellEditorParams: {
      // Use display values in dropdown
      values: Object.values(statusMappings)
    },
    // Display mapped value
    valueFormatter: (params) => {
      return lookupValue(statusMappings, params.value)
    },
    // Store code when editing
    valueParser: (params) => {
      return lookupKey(statusMappings, params.newValue)
    }
  }
]
```

### Complete Mapping Example

```javascript
const carMappings = {
  'tyt': 'Toyota',
  'frd': 'Ford',
  'prs': 'Porsche',
  'nss': 'Nissan'
}

const extractKeys = (mappings) => {
  return Object.keys(mappings)
}

const lookupValue = (mappings, key) => {
  return mappings[key] || key
}

const lookupKey = (mappings, name) => {
  const keys = Object.keys(mappings)
  for (let i = 0; i < keys.length; i++) {
    if (mappings[keys[i]] === name) {
      return keys[i]
    }
  }
  return name
}

const columnDefs = [
  {
    field: 'make',
    headerName: 'Car Make',
    editable: true,
    cellEditor: 'agSelectCellEditor',
    cellEditorParams: {
      // Show user-friendly names in dropdown
      values: Object.values(carMappings)
    },
    // Display user-friendly name
    valueFormatter: (params) => {
      return lookupValue(carMappings, params.value)
    },
    // Store code when selected
    valueParser: (params) => {
      return lookupKey(carMappings, params.newValue)
    }
  }
]

const rowData = [
  { make: 'tyt' }, // Stored as code
  { make: 'frd' }  // Stored as code
]
```

## 5. Popup Mode

Dropdown editors can be displayed as popups instead of inline editors.

### Select Editor as Popup

```javascript
const columnDefs = [
  {
    field: 'language',
    headerName: 'Language',
    editable: true,
    cellEditor: 'agSelectCellEditor',
    cellEditorPopup: true, // Display as popup
    cellEditorParams: {
      values: ['English', 'Spanish', 'French', 'Portuguese']
    }
  }
]
```

### Popup Positioning

```javascript
const columnDefs = [
  {
    field: 'status',
    headerName: 'Status',
    editable: true,
    cellEditor: 'agSelectCellEditor',
    cellEditorPopup: true,
    cellEditorPopupPosition: 'under', // 'over' or 'under'
    cellEditorParams: {
      values: ['Active', 'Inactive', 'Pending']
    }
  }
]
```

## 6. Custom Dropdown Cell Editor

For complete control, create a custom dropdown cell editor.

### Class-Based Custom Editor

```javascript
class CustomDropdownEditor {
  eSelect
  value
  values

  init(params) {
    this.value = params.value
    this.values = params.values || []
    
    // Create select element
    this.eSelect = document.createElement('select')
    this.eSelect.className = 'custom-dropdown-editor'
    this.eSelect.style.width = '100%'
    this.eSelect.style.height = '100%'
    
    // Add options
    this.values.forEach(value => {
      const option = document.createElement('option')
      option.value = value
      option.textContent = value
      this.eSelect.appendChild(option)
    })
    
    // Set initial value
    this.eSelect.value = this.value || ''
    
    // Handle change events
    this.eSelect.addEventListener('change', () => {
      this.value = this.eSelect.value
    })
    
    // Handle keyboard events
    this.eSelect.addEventListener('keydown', (event) => {
      if (event.key === 'Enter') {
        params.stopEditing()
      } else if (event.key === 'Escape') {
        params.stopEditing(true) // Cancel
      }
    })
  }

  getGui() {
    return this.eSelect
  }

  afterGuiAttached() {
    this.eSelect.focus()
  }

  getValue() {
    return this.eSelect.value
  }

  isCancelBeforeStart() {
    return false
  }

  isCancelAfterEnd() {
    // Validate value if needed
    return false
  }

  isPopup() {
    return false // Set to true for popup mode
  }
}

// Usage
const columnDefs = [
  {
    field: 'status',
    headerName: 'Status',
    editable: true,
    cellEditor: CustomDropdownEditor,
    cellEditorParams: {
      values: ['Active', 'Inactive', 'Pending', 'Suspended']
    }
  }
]
```

### React-Based Custom Editor

```jsx
import React, { forwardRef, useImperativeHandle, useRef, useState, useEffect } from 'react'

const CustomDropdownEditor = forwardRef((props, ref) => {
  const [value, setValue] = useState(props.value || '')
  const selectRef = useRef(null)

  useImperativeHandle(ref, () => {
    return {
      getValue() {
        return selectRef.current.value
      },
      isCancelAfterEnd() {
        // Validation logic
        return false
      }
    }
  })

  useEffect(() => {
    selectRef.current.focus()
  }, [])

  return (
    <select
      ref={selectRef}
      value={value}
      onChange={(e) => setValue(e.target.value)}
      style={{ width: '100%', height: '100%' }}
    >
      {props.values.map((option) => (
        <option key={option} value={option}>
          {option}
        </option>
      ))}
    </select>
  )
})

// Usage in column definition
const columnDefs = [
  {
    field: 'status',
    headerName: 'Status',
    editable: true,
    cellEditor: CustomDropdownEditor,
    cellEditorParams: {
      values: ['Active', 'Inactive', 'Pending']
    }
  }
]
```

### Advanced Custom Editor with Search

```javascript
class SearchableDropdownEditor {
  eContainer
  eInput
  eList
  value
  values
  filteredValues

  init(params) {
    this.value = params.value
    this.values = params.values || []
    this.filteredValues = [...this.values]
    
    // Create container
    this.eContainer = document.createElement('div')
    this.eContainer.className = 'searchable-dropdown-editor'
    this.eContainer.style.width = '200px'
    this.eContainer.style.border = '1px solid #ccc'
    this.eContainer.style.backgroundColor = 'white'
    
    // Create search input
    this.eInput = document.createElement('input')
    this.eInput.type = 'text'
    this.eInput.placeholder = 'Search...'
    this.eInput.style.width = '100%'
    this.eInput.style.padding = '4px'
    this.eInput.value = this.value || ''
    
    // Create dropdown list
    this.eList = document.createElement('div')
    this.eList.style.maxHeight = '200px'
    this.eList.style.overflowY = 'auto'
    this.eList.style.borderTop = '1px solid #ccc'
    
    this.eContainer.appendChild(this.eInput)
    this.eContainer.appendChild(this.eList)
    
    // Filter on input
    this.eInput.addEventListener('input', (e) => {
      this.filterValues(e.target.value)
    })
    
    // Handle selection
    this.eList.addEventListener('click', (e) => {
      if (e.target.classList.contains('dropdown-item')) {
        this.value = e.target.dataset.value
        this.eInput.value = e.target.textContent
        params.stopEditing()
      }
    })
    
    // Initial render
    this.renderList()
  }

  filterValues(searchTerm) {
    this.filteredValues = this.values.filter(value =>
      value.toLowerCase().includes(searchTerm.toLowerCase())
    )
    this.renderList()
  }

  renderList() {
    this.eList.innerHTML = ''
    this.filteredValues.forEach(value => {
      const item = document.createElement('div')
      item.className = 'dropdown-item'
      item.textContent = value
      item.dataset.value = value
      item.style.padding = '4px'
      item.style.cursor = 'pointer'
      item.style.backgroundColor = value === this.value ? '#e0e0e0' : 'white'
      
      item.addEventListener('mouseenter', () => {
        item.style.backgroundColor = '#f0f0f0'
      })
      item.addEventListener('mouseleave', () => {
        item.style.backgroundColor = value === this.value ? '#e0e0e0' : 'white'
      })
      
      this.eList.appendChild(item)
    })
  }

  getGui() {
    return this.eContainer
  }

  afterGuiAttached() {
    this.eInput.focus()
    this.eInput.select()
  }

  getValue() {
    return this.value
  }

  isPopup() {
    return true // Display as popup
  }
}

// Usage
const columnDefs = [
  {
    field: 'country',
    headerName: 'Country',
    editable: true,
    cellEditor: SearchableDropdownEditor,
    cellEditorParams: {
      values: [
        'United States', 'United Kingdom', 'France', 'Germany',
        'China', 'Japan', 'India', 'Brazil', 'Canada', 'Australia'
      ]
    }
  }
]
```

## 7. Cell Editor Selector

Use `cellEditorSelector` to dynamically choose which editor to use based on cell data or conditions.

### Dynamic Editor Selection

```javascript
const columnDefs = [
  {
    field: 'type',
    headerName: 'Type',
    editable: true,
    cellEditor: 'agSelectCellEditor',
    cellEditorParams: {
      values: ['Text', 'Number', 'Date', 'Select']
    }
  },
  {
    field: 'value',
    headerName: 'Value',
    editable: true,
    // Select editor based on type
    cellEditorSelector: (params) => {
      const type = params.data.type
      
      if (type === 'Select') {
        return {
          component: 'agSelectCellEditor',
          params: {
            values: ['Option 1', 'Option 2', 'Option 3']
          }
        }
      } else if (type === 'Number') {
        return {
          component: 'agNumberCellEditor'
        }
      } else if (type === 'Date') {
        return {
          component: 'agDateCellEditor'
        }
      } else {
        return {
          component: 'agTextCellEditor'
        }
      }
    }
  }
]
```

## 8. Complete Example: Multi-Column Dropdowns

```javascript
import { createGrid } from 'ag-grid-community'

const columnDefs = [
  {
    field: 'id',
    headerName: 'ID',
    width: 80
  },
  {
    field: 'category',
    headerName: 'Category',
    editable: true,
    cellEditor: 'agSelectCellEditor',
    cellEditorParams: {
      values: ['Electronics', 'Clothing', 'Food', 'Books']
    }
  },
  {
    field: 'subcategory',
    headerName: 'Subcategory',
    editable: true,
    cellEditor: 'agSelectCellEditor',
    cellEditorParams: (params) => {
      const category = params.data.category
      
      const subcategories = {
        'Electronics': ['Computers', 'Phones', 'Accessories'],
        'Clothing': ['Men', 'Women', 'Kids'],
        'Food': ['Fruits', 'Vegetables', 'Dairy'],
        'Books': ['Fiction', 'Non-Fiction', 'Reference']
      }
      
      return {
        values: subcategories[category] || []
      }
    }
  },
  {
    field: 'status',
    headerName: 'Status',
    editable: true,
    cellEditor: 'agRichSelectCellEditor',
    cellEditorParams: {
      values: ['Active', 'Inactive', 'Pending', 'Suspended'],
      cellHeight: 25,
      searchDebounceDelay: 300
    }
  }
]

const rowData = [
  { id: 1, category: 'Electronics', subcategory: 'Computers', status: 'Active' },
  { id: 2, category: 'Clothing', subcategory: 'Men', status: 'Pending' },
  { id: 3, category: 'Food', subcategory: 'Fruits', status: 'Active' }
]

const gridOptions = {
  columnDefs: columnDefs,
  rowData: rowData,
  defaultColDef: {
    sortable: true,
    filter: true,
    resizable: true
  },
  onCellValueChanged: (event) => {
    console.log('Cell value changed:', {
      field: event.column.getColId(),
      oldValue: event.oldValue,
      newValue: event.newValue,
      data: event.data
    })
    
    // Reset dependent dropdowns when parent changes
    if (event.column.getColId() === 'category') {
      event.data.subcategory = null
      event.api.refreshCells({
        rowNodes: [event.node],
        columns: ['subcategory']
      })
    }
  },
  onCellEditingStarted: (event) => {
    console.log('Started editing:', event.column.getColId())
  },
  onCellEditingStopped: (event) => {
    console.log('Stopped editing:', event.valueChanged)
  }
}

const eGridDiv = document.querySelector('#myGrid')
const gridApi = createGrid(eGridDiv, gridOptions)
```

## 9. Event Handling

### Cell Editing Events

```javascript
const gridOptions = {
  // Fired when editing starts
  onCellEditingStarted: (event) => {
    console.log('Editing started:', {
      field: event.column.getColId(),
      value: event.value,
      data: event.data
    })
  },
  
  // Fired when editing stops
  onCellEditingStopped: (event) => {
    console.log('Editing stopped:', {
      field: event.column.getColId(),
      value: event.value,
      valueChanged: event.valueChanged,
      data: event.data
    })
  },
  
  // Fired when value changes
  onCellValueChanged: (event) => {
    console.log('Value changed:', {
      field: event.column.getColId(),
      oldValue: event.oldValue,
      newValue: event.newValue,
      data: event.data
    })
    
    // Save to backend
    saveToBackend(event.data)
  }
}
```

## 10. Best Practices

### 1. Provide Default Values

```javascript
cellEditorParams: {
  values: ['Option 1', 'Option 2', 'Option 3'],
  // Ensure current value is in the list
  defaultValue: 'Option 1'
}
```

### 2. Handle Missing Values

```javascript
cellEditorParams: (params) => {
  const values = getValuesForCell(params)
  
  // Ensure current value is included
  if (params.value && !values.includes(params.value)) {
    values.unshift(params.value)
  }
  
  return { values }
}
```

### 3. Validate Selections

```javascript
class ValidatedDropdownEditor {
  init(params) {
    // ... setup code
    this.validValues = params.values
  }
  
  isCancelAfterEnd() {
    // Validate selected value
    return !this.validValues.includes(this.getValue())
  }
}
```

### 4. Performance for Large Lists

```javascript
// For large lists, use Rich Select Editor with search
cellEditor: 'agRichSelectCellEditor',
cellEditorParams: {
  values: largeArray, // Can handle thousands of items
  searchDebounceDelay: 300,
  cellHeight: 25
}
```

### 5. Accessibility

```javascript
cellEditorParams: {
  values: ['Option 1', 'Option 2', 'Option 3'],
  // Ensure proper ARIA labels
  ariaLabel: 'Select an option'
}
```

## Summary

**AG Grid provides dropdown options in cells through:**

1. **Built-in Editors**: `agSelectCellEditor` (standard) and `agRichSelectCellEditor` (Enterprise)
2. **Configuration**: `cellEditorParams` with `values` array
3. **Dynamic Options**: Function-based `cellEditorParams` for context-dependent values
4. **Value Mapping**: `valueFormatter` and `valueParser` for key-value pairs
5. **Popup Mode**: `cellEditorPopup: true` for overlay display
6. **Custom Editors**: Full control with class-based or React components
7. **Editor Selection**: `cellEditorSelector` for conditional editor choice

**Key Features:**
- **Simple setup**: Just specify `cellEditor` and `cellEditorParams.values`
- **Dynamic values**: Options can change based on other cell values
- **Value mapping**: Store codes while displaying user-friendly names
- **Search support**: Rich Select Editor provides search functionality (Enterprise)
- **Customizable**: Full control with custom cell editors

**Use Cases:**
- **Status fields**: Active/Inactive/Pending dropdowns
- **Category selection**: Hierarchical dropdowns (country → city)
- **Reference data**: Code-to-name mappings
- **Enum values**: Predefined option lists
- **Dynamic filtering**: Options filtered by other columns

This architecture provides a flexible and powerful way to add dropdown options to AG Grid cells, supporting both simple use cases and complex requirements.

