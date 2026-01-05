# Tauri + SvelteKit Initialization Guide

This document explains how to initialize a Tauri application with SvelteKit, based on the official Tauri documentation.

## Overview

Tauri is a framework for building desktop applications with a Rust backend and any web frontend. When using SvelteKit with Tauri, you need to configure SvelteKit to use static site generation (SSG) or single-page application (SPA) mode, as Tauri doesn't support server-side rendering (SSR).

## Prerequisites

### Required

1. **Rust** (latest stable version)
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   rustup update
   ```

2. **System Dependencies**:
   - **Linux**: `libwebkit2gtk-4.0-dev`, `build-essential`, `curl`, `wget`, `libssl-dev`, `libgtk-3-dev`, `libayatana-appindicator3-dev`, `librsvg2-dev`
   - **macOS**: Xcode Command Line Tools
   - **Windows**: Microsoft C++ Build Tools, WebView2

3. **Node.js v18+** (for SvelteKit)

4. **Package Manager**: npm, yarn, pnpm, or bun

## Method 1: Using `create-tauri-app` (Recommended)

The easiest way to create a new Tauri + SvelteKit project is using `create-tauri-app`.

### Step 1: Run create-tauri-app

```bash
# Using npm
npm create tauri-app@latest

# Using yarn
yarn create tauri-app

# Using pnpm
pnpm create tauri-app

# Using bun
bun create tauri-app

# Using bash (Linux/macOS)
sh <(curl https://create.tauri.app/sh)

# Using PowerShell (Windows)
irm https://create.tauri.app/ps | iex
```

### Step 2: Answer Configuration Questions

The CLI will prompt you with several questions:

1. **Project name**: Choose a name for your project
2. **Frontend language**: Select `TypeScript / JavaScript`
3. **Package manager**: Choose `npm`, `yarn`, `pnpm`, or `bun`
4. **UI template**: Select `SvelteKit`
5. **UI flavor**: Select `TypeScript` (recommended) or `JavaScript`

### Step 3: Install Dependencies

```bash
cd your-project-name
npm install  # or yarn/pnpm/bun install
```

### Step 4: Run Development Server

```bash
npm run tauri dev
```

This will:
- Start the SvelteKit dev server
- Compile the Rust backend
- Open a window with your application

**Congratulations!** You've created a Tauri + SvelteKit app! 🚀

## Method 2: Manual Setup

If you prefer to set up the project manually or add Tauri to an existing SvelteKit project:

### Step 1: Create SvelteKit Project

```bash
npm create svelte@latest my-tauri-app
cd my-tauri-app
npm install
```

**Configuration Options**:
- **App template**: `Skeleton project` (barebones) or `SvelteKit demo app`
- **Type checking**: `TypeScript` (recommended) or `JSDoc`
- **Code linting and formatting**: Enable ESLint and Prettier (recommended)
- **Browser testing**: Skip Playwright (Tauri APIs don't work in Playwright)

### Step 2: Install Static Adapter

SvelteKit needs to be configured for static site generation (SSG) or SPA mode:

```bash
npm install --save-dev @sveltejs/adapter-static
```

### Step 3: Configure SvelteKit

Update `svelte.config.js`:

```javascript
import adapter from "@sveltejs/adapter-static";
import { vitePreprocess } from "@sveltejs/vite-plugin-svelte";

/** @type {import('@sveltejs/kit').Config} */
const config = {
  preprocess: vitePreprocess(),

  kit: {
    adapter: adapter({
      fallback: "index.html", // Required for SPA mode
      strict: false, // Don't error on dynamic routes
    }),
  },
};

export default config;
```

**For SSG Mode** (Static Site Generation):

```javascript
import adapter from "@sveltejs/adapter-static";
import { vitePreprocess } from "@sveltejs/vite-plugin-svelte";

/** @type {import('@sveltejs/kit').Config} */
const config = {
  preprocess: vitePreprocess(),

  kit: {
    adapter: adapter(),
  },
};

export default config;
```

### Step 4: Disable SSR

Create `src/routes/+layout.ts` (or `+layout.js`):

```typescript
// src/routes/+layout.ts
export const prerender = true; // Enable prerendering for SSG
export const ssr = false; // Disable SSR (required for Tauri APIs)
```

**For SPA Mode**:

```typescript
// src/routes/+layout.ts
export const ssr = false; // Disable SSR
// Don't set prerender for SPA mode
```

**Why disable SSR?**
- Tauri APIs rely on browser-specific `window` objects
- SSR runs on the server where these APIs don't exist
- Disabling SSR allows you to use Tauri APIs without client-side checks

### Step 5: Install Tauri CLI

```bash
npm install --save-dev @tauri-apps/cli
```

Add to `package.json` scripts:

```json
{
  "scripts": {
    "tauri": "tauri"
  }
}
```

### Step 6: Initialize Tauri

Run the Tauri initialization command:

```bash
npm run tauri init
```

**Answer the prompts**:

1. **What is your app name?**
   - This will be the name of your final bundle and what the OS will call your app
   - Example: `my-tauri-app`

2. **What should the window title be?**
   - The title of the default main window
   - Example: `My Tauri App`

3. **Where are your web assets located?**
   - Path relative to `src-tauri/tauri.conf.json` for **production** builds
   - Use: `../build` (for SSG) or `../dist` (for SPA)

4. **What is the URL of your dev server?**
   - URL or file path Tauri will load during **development**
   - Use: `http://localhost:5173` (default SvelteKit dev server port)

5. **What is your frontend dev command?**
   - Command to start your frontend dev server
   - Use: `npm run dev` (or `yarn dev`, `pnpm dev`, etc.)

6. **What is your frontend build command?**
   - Command to build your frontend files
   - Use: `npm run build` (or `yarn build`, `pnpm build`, etc.)

### Step 7: Verify Configuration

After initialization, check `src-tauri/tauri.conf.json`:

```json
{
  "build": {
    "beforeDevCommand": "npm run dev",
    "beforeBuildCommand": "npm run build",
    "devUrl": "http://localhost:5173",
    "frontendDist": "../build"
  }
}
```

**Note**: For SPA mode, `frontendDist` should be `../dist` instead of `../build`.

### Step 8: Run Development Server

```bash
npm run tauri dev
```

This will:
1. Start the SvelteKit dev server
2. Compile the Rust backend
3. Open a window with your application

## Project Structure

After initialization, your project structure should look like:

```
my-tauri-app/
├── src/                    # SvelteKit source files
│   ├── routes/
│   │   ├── +layout.ts      # Layout with SSR disabled
│   │   └── +page.svelte    # Home page
│   └── lib/                # Shared components/utilities
├── src-tauri/              # Tauri backend
│   ├── src/
│   │   └── main.rs         # Rust entry point
│   ├── Cargo.toml          # Rust dependencies
│   ├── tauri.conf.json     # Tauri configuration
│   └── icons/              # App icons
├── svelte.config.js        # SvelteKit configuration
├── package.json            # Node.js dependencies
└── vite.config.js          # Vite configuration
```

## Key Configuration Files

### `svelte.config.js`

```javascript
import adapter from "@sveltejs/adapter-static";
import { vitePreprocess } from "@sveltejs/vite-plugin-svelte";

/** @type {import('@sveltejs/kit').Config} */
const config = {
  preprocess: vitePreprocess(),
  kit: {
    adapter: adapter({
      fallback: "index.html", // For SPA mode
      strict: false,
    }),
  },
};

export default config;
```

### `src/routes/+layout.ts`

```typescript
export const prerender = true; // For SSG
export const ssr = false; // Required for Tauri
```

### `src-tauri/tauri.conf.json`

```json
{
  "build": {
    "beforeDevCommand": "npm run dev",
    "beforeBuildCommand": "npm run build",
    "devUrl": "http://localhost:5173",
    "frontendDist": "../build"
  },
  "app": {
    "windows": [
      {
        "title": "My Tauri App",
        "width": 1200,
        "height": 800
      }
    ]
  }
}
```

## Creating Your First Tauri Command

Tauri commands are Rust functions that can be called from JavaScript. Here's a simple example:

### Step 1: Create a Command in Rust

Edit `src-tauri/src/main.rs`:

```rust
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```

### Step 2: Install Tauri API

```bash
npm install @tauri-apps/api
```

### Step 3: Call Command from Svelte

Create `src/lib/Greet.svelte`:

```svelte
<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri";

  let name = "";
  let greetMsg = "";

  async function greet() {
    greetMsg = await invoke("greet", { name });
  }
</script>

<div>
  <input
    id="greet-input"
    placeholder="Enter a name..."
    bind:value={name}
  />
  <button on:click={greet}>Greet</button>
  <p>{greetMsg}</p>
</div>
```

### Step 4: Use Component in Page

Edit `src/routes/+page.svelte`:

```svelte
<script lang="ts">
  import Greet from "$lib/Greet.svelte";
</script>

<h1>Welcome to SvelteKit + Tauri</h1>
<Greet />
```

## Building for Production

To create a production build:

```bash
npm run tauri build
```

This will:
1. Build the SvelteKit frontend
2. Compile the Rust backend
3. Create platform-specific installers in `src-tauri/target/release/bundle/`

**Output locations**:
- **Windows**: `.msi` installer
- **macOS**: `.dmg` and `.app` bundle
- **Linux**: `.deb`, `.AppImage`, and `.rpm` packages

## Common Issues and Solutions

### Issue: "Cannot find module '@tauri-apps/api'"

**Solution**: Install the Tauri API package:

```bash
npm install @tauri-apps/api
```

### Issue: "window is not defined" or Tauri APIs not available

**Solution**: Ensure SSR is disabled in `src/routes/+layout.ts`:

```typescript
export const ssr = false;
```

### Issue: Routes not working in production build

**Solution**: For SPA mode, ensure `fallback: "index.html"` is set in `svelte.config.js`:

```javascript
adapter: adapter({
  fallback: "index.html",
  strict: false,
})
```

### Issue: Build directory mismatch

**Solution**: Check `src-tauri/tauri.conf.json`:
- For SSG: `frontendDist: "../build"`
- For SPA: `frontendDist: "../dist"`

### Issue: Dev server port conflict

**Solution**: If port 5173 is in use, either:
1. Change SvelteKit port in `vite.config.js`:
   ```javascript
   export default defineConfig({
     server: { port: 3000 }
   });
   ```
2. Update `devUrl` in `tauri.conf.json`:
   ```json
   {
     "build": {
       "devUrl": "http://localhost:3000"
     }
   }
   ```

## Next Steps

1. **Learn Tauri Commands**: Create Rust functions callable from JavaScript
2. **Add Plugins**: Extend functionality with Tauri plugins
3. **Configure Window**: Customize window appearance and behavior
4. **Add Icons**: Replace default icons with your app icons
5. **Build and Distribute**: Create installers for your target platforms

## Additional Resources

- [Tauri Documentation](https://tauri.app/)
- [SvelteKit Documentation](https://kit.svelte.dev/)
- [Tauri + SvelteKit Guide](https://tauri.app/v1/guides/frontend/sveltekit)
- [Tauri API Reference](https://tauri.app/v1/api/js/)

## Summary

Initializing Tauri with SvelteKit involves:

1. ✅ **Create SvelteKit project** (or use existing)
2. ✅ **Install static adapter**: `@sveltejs/adapter-static`
3. ✅ **Configure SvelteKit**: Update `svelte.config.js` with static adapter
4. ✅ **Disable SSR**: Create `+layout.ts` with `ssr = false`
5. ✅ **Install Tauri CLI**: `@tauri-apps/cli`
6. ✅ **Initialize Tauri**: Run `npm run tauri init`
7. ✅ **Configure paths**: Set correct `frontendDist` in `tauri.conf.json`
8. ✅ **Run dev server**: `npm run tauri dev`

The process is straightforward, and both `create-tauri-app` and manual setup methods work well. Choose the method that best fits your workflow!

