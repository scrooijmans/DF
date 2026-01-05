# Vite/Tauri Development Server Stuck - File Lock Issue

## Problem

When running `npm run tauri dev`, the development server gets stuck with the message:

```
Blocking waiting for file lock on build directory
```

The Vite dev server starts successfully (e.g., on `http://localhost:5174/`), but the Tauri desktop application never launches.

## Root Cause

Multiple Cargo processes are trying to access the same build directory simultaneously, causing a file lock contention on `/Users/sc/Github/MudRock/target/debug/.cargo-lock`.

This typically happens when:

1. Multiple `npm run tauri dev` processes are running
2. Previous Cargo processes didn't terminate cleanly
3. Clean build (`cargo clean`) followed by multiple restart attempts

## Solution

### Step 1: Identify Conflicting Processes

```bash
ps aux | grep -E "(tauri|cargo)" | grep -v grep
```

Look for multiple Cargo processes with different PIDs.

### Step 2: Kill Conflicting Processes

```bash
# Kill all Tauri/Cargo processes
pkill -f "tauri dev"
pkill -f "npm run tauri dev"
pkill -f "cargo run"

# Or kill specific PIDs if you know them
kill -9 <PID1> <PID2>
```

### Step 3: Verify Processes Are Terminated

```bash
ps aux | grep -E "(tauri|cargo)" | grep -v grep
```

Should return no results.

### Step 4: Restart Cleanly

```bash
cd /Users/sc/Github/MudRock
npm run tauri dev
```

## Verification

After restarting, you should see:

1. ✅ Vite dev server starts (e.g., `http://localhost:5174/`)
2. ✅ Rust compilation completes without file lock errors
3. ✅ Tauri desktop application launches (check for `/Users/sc/Github/MudRock/target/debug/mudrock` process)

## Prevention

- Always wait for the current `npm run tauri dev` process to complete before starting a new one
- Use `Ctrl+C` to properly terminate the development server
- If you need to restart, ensure all processes are killed before starting fresh

## Alternative: Clean Build

If the issue persists, try a clean build:

```bash
cargo clean
rm -rf node_modules/.vite
npm run tauri dev
```

This will take longer (5-15 minutes) but ensures no cached build artifacts cause conflicts.
