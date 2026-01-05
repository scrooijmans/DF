How to dynamically add data to ECharts charts
Primary method: setOption()
Use setOption() to update data. ECharts merges new options with existing ones and refreshes the chart.
Basic syntax:
chart.setOption(option, notMerge, lazyUpdate);// orchart.setOption(option, { notMerge?: boolean; replaceMerge?: string | string[]; lazyUpdate?: boolean; silent?: boolean;});
Patterns for dynamic data updates

1. Asynchronous data loading
   Load data after initialization:
   var myChart = echarts.init(document.getElementById('main'));// Initial setup with empty datamyChart.setOption({ title: { text: 'Dynamic Data Example' }, tooltip: {}, legend: { data: ['Sales'] }, xAxis: { data: [] }, yAxis: {}, series: [{ name: 'Sales', type: 'bar', data: [] }]});// Load data asynchronously$.get('data.json').done(function (data) { myChart.setOption({ xAxis: { data: data.categories }, series: [{ name: 'Sales', // Find series by name data: data.data }] });});
2. Event-driven updates
   Update based on user interactions:
   myChart.on('click', function (params) { // Fetch detail data based on clicked element $.get('detail?q=' + params.name, function (detail) { myChart.setOption({ series: [{ name: 'pie', data: [detail.data] }] }); });});
3. Merge modes
   Default merge: New options merge with existing ones
   myChart.setOption({ series: [{ data: newData }] });
   Not merge: Replace all options
   myChart.setOption(option, { notMerge: true });
   Replace merge: Replace specific components
   myChart.setOption(option, { replaceMerge: ['xAxis', 'yAxis', 'series'] });
4. Performance optimization
   Lazy update: Defer updates for rapid changes
   myChart.setOption(option, { lazyUpdate: true });
   Silent mode: Suppress events during updates
   myChart.setOption(option, { silent: true });
   Best practices
   Target series by name when updating specific series
   Use lazyUpdate: true for frequent updates
   Use replaceMerge to replace entire component arrays (e.g., series, xAxis)
   ECharts animates transitions between data states by default
   Real-time data updates
   For real-time time-series data:
   Use lazyUpdate: true for high-frequency updates
   Consider dataZoom with rangeMode: 'percent' to keep the window aligned with appended data
   This approach supports dynamic, interactive charts that update smoothly as new data arrives.
