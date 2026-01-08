# Debugging D3 Chart Data Flow

## Debug Logging Points

Add these console.log statements to trace data flow issues:

### ChartSettingsPanel.svelte
```typescript
// In handleAxisChange
console.log('[ChartSettingsPanel] handleAxisChange:', { key, binding, chartType: chartConfig.type })

// In loadD3WellLogData
console.log('[ChartSettingsPanel] loadD3WellLogData called:', { curveId })
console.log('[ChartSettingsPanel] Received segmented data:', {
  hasData: !!segmentedData,
  segmentCount: segmentedData?.segments?.length ?? 0,
  depthRange: segmentedData?.depth_range,
  mnemonic: segmentedData?.mnemonic
})
console.log('[ChartSettingsPanel] Calling onSegmentedDataChange with data')
```

### PaneContainer.svelte
```typescript
$effect(() => {
  if (pane?.paneType === PaneType.D3WellLog) {
    console.log('[PaneContainer] D3WellLog data update:', {
      paneId: pane?.id,
      hasSegmentedData: !!segmentedChartData,
      segmentCount: segmentedChartData?.segments?.length ?? 0,
      depthRange: segmentedChartData?.depth_range,
      mnemonic: segmentedChartData?.mnemonic,
      hasChartConfig: !!chartConfig,
      configType: chartConfig?.type,
      curveColor: (chartConfig as D3WellLogConfig | undefined)?.curveColor
    })
  }
})
```

### dataStore.ts
```typescript
// In loadSegmentedCurveData
console.log('[dataStore] loadSegmentedCurveData called for curve:', curveId)
console.log('[dataStore] get_curve_data_segmented result:', {
  hasData: !!result,
  segmentCount: result?.segments?.length
})
```

## Expected Log Sequence

When user selects a curve, you should see this sequence:

1. `[ChartSettingsPanel] handleAxisChange` - Curve selection triggered
2. `[ChartSettingsPanel] loadD3WellLogData called` - Data loading started
3. `[dataStore] loadSegmentedCurveData called` - Backend request initiated
4. `[dataStore] get_curve_data_segmented result` - Backend response
5. `[ChartSettingsPanel] Received segmented data` - Data processed
6. `[ChartSettingsPanel] Calling onSegmentedDataChange` - Callback fired
7. `[PaneContainer] D3WellLog data update` - Component received data

## Common Issues

### Data not appearing in chart

**Symptoms:** Chart shows empty state, no curve visible

**Check:**
1. Is `segmentedChartData?.segments?.length > 0` in PaneContainer?
2. Is `onSegmentedDataChange` callback wired in ContextToolbar?
3. Does `handleSegmentedDataChange` in +page.svelte call `workspaceManager.updatePaneConfig`?
4. Is `segmentedChartData` derived from `pane?.config?.segmentedChartData`?

**Root cause:** Usually a missing callback in the chain.

### Style changes not reactive

**Symptoms:** Changing color/line width in settings doesn't update chart

**Check:**
1. Does config prop flow: ContextToolbar → ChartSettingsPanel → Style component?
2. Is `chartConfig` derived from `pane?.config?.chartConfig` in PaneContainer?
3. Is D3WellLogTrack receiving config as prop (not internal state)?

**Root cause:** Usually config stored in local state instead of flowing from props.

### Infinite loops or freezing

**Symptoms:** Browser hangs, console floods with logs

**Check:**
1. Is initialization guard using regular variable (NOT `$state`)?
2. Is D3 setup wrapped in `untrack()`?
3. Does `$effect` return a cleanup function that resets the guard?

**Root cause:** Usually `$state` used for `isInitialized` flag.

### Config is null/undefined

**Symptoms:** TypeError accessing config properties

**Check:**
1. Are all config accesses using optional chaining (`?.`)?
2. Is there a fallback with nullish coalescing (`??`)?
3. Does PaneContainer derive config with fallback: `$derived(pane?.config?.chartConfig ?? null)`?

**Root cause:** Config doesn't exist until user makes first selection.

## Debugging Checklist

When D3 chart data flow isn't working:

- [ ] Add console.log to ChartSettingsPanel.handleAxisChange
- [ ] Verify curve selection triggers loadD3WellLogData
- [ ] Check backend returns valid segmented data
- [ ] Verify onSegmentedDataChange callback fires
- [ ] Add debug $effect to PaneContainer
- [ ] Confirm segmentedChartData has segments
- [ ] Check D3 component receives non-null props

## Key Files for Data Flow

| File | Role |
|------|------|
| `ChartSettingsPanel.svelte` | Handles curve selection, loads data, calls callbacks |
| `+page.svelte` | Receives callbacks, updates workspaceManager |
| `ContextToolbar.svelte` | Wires callbacks between page and settings panel |
| `PaneContainer.svelte` | Derives data from pane config, renders D3 component |
| `dataStore.ts` | Backend API calls for curve data |
| `D3WellLogTrack.svelte` | Renders SVG from data/config props |
