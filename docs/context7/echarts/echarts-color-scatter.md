How to Color Scatter Plots in Apache ECharts
Two approaches: color by category/cluster, or by a numeric series property.

1. Color by Category/Cluster
   Use visualMap with type: 'piecewise' and a categories array. Map each category to a color.
   Example:
   option = { xAxis: {}, yAxis: {}, visualMap: { type: 'piecewise', // Define your categories/clusters categories: [ 'Cluster A', 'Cluster B', 'Cluster C', 'Cluster D', 'Cluster E' ], // Map categories to colors inRange: { color: { 'Cluster A': 'red', 'Cluster B': 'blue', 'Cluster C': 'green', '': 'gray' // Default color for unmapped categories } // OR use an array (one-to-one mapping): // color: ['red', 'blue', 'green', 'yellow', 'purple'] } }, series: [{ type: 'scatter', data: [ [10, 20, 'Cluster A'], // [x, y, category] [15, 25, 'Cluster B'], [20, 30, 'Cluster C'], [25, 35, 'Cluster A'] ] }]};
   Key points:
   Set type: 'piecewise' for categorical data
   Define categories with all possible category values
   Map colors using an object (specific mappings) or an array (one-to-one)
   The category value should be in the data array (e.g., the 3rd element)
2. Color by Series Property Value (Numeric Dimension)
   Use visualMap with dimension to select a numeric dimension for continuous color mapping.
   Example with continuous mapping:
   option = { xAxis: {}, yAxis: {}, visualMap: { type: 'continuous', // or 'piecewise' for discrete ranges min: 0, max: 100, dimension: 2, // Use the 3rd dimension (index 2) for coloring inRange: { color: [ '#313695', '#4575b4', '#74add1', '#abd9e9', '#e0f3f8', '#ffffbf', '#fee090', '#fdae61', '#f46d43', '#d73027', '#a50026' ] } }, series: [{ type: 'scatter', data: [ [10, 20, 45], // [x, y, value] - value at index 2 used for color [15, 25, 78], [20, 30, 23], [25, 35, 91] ] }]};
   Example with named dimensions (using dataset):
   option = { dataset: { source: [ ['Country', 'Population', 'Income', 'Life Expectancy'], ['USA', 300, 50000, 78], ['China', 1400, 10000, 76], ['Japan', 127, 40000, 84] ] }, xAxis: {}, yAxis: {}, visualMap: { calculable: true, max: 100, dimension: 'Life Expectancy', // Use dimension name inRange: { color: ['#313695', '#4575b4', '#74add1', '#abd9e9', '#e0f3f8', '#ffffbf', '#fee090', '#fdae61', '#f46d43', '#d73027', '#a50026'] } }, series: [{ type: 'scatter', encode: { x: 'Population', y: 'Income', tooltip: [0, 1, 2, 3] } }]};
   Example with piecewise ranges:
   visualMap: { type: 'piecewise', min: 0, max: 5000, dimension: 3, // 4th dimension (index 3) inRange: { color: ['blue', '#121122', 'red'] }, // Or define custom pieces: pieces: [ {min: 1500, color: 'red'}, {min: 900, max: 1500, color: 'orange'}, {min: 310, max: 900, color: 'yellow'}, {max: 310, color: 'green'} ]}
   Data Format
   For multi-dimensional data, each point is an array:
   data: [ [x, y, categoryOrValue], // 3 dimensions [x, y, value1, value2, category], // 5 dimensions // or as objects: { value: [x, y, categoryOrValue], itemStyle: { /_ optional override _/ } }]
   Notes
   Dimension indexing: dimension: 0 = first element, dimension: 2 = third element
   Named dimensions: with dataset, use dimension names (e.g., 'Life Expectancy')
   Multiple visualMaps: you can define multiple visualMap components in an array
   Series targeting: use seriesIndex to target specific series
   Continuous vs piecewise:
   continuous: smooth gradient for numeric data
   piecewise: discrete ranges or categories
   These methods let you color scatter plots by category/cluster or by a numeric property.
