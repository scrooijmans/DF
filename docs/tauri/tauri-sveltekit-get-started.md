# SvelteKit | Tauri Apps

This guide will walk you through creating your first Tauri app using the [SvelteKit](https://kit.svelte.dev/) frontend framework.

info

Before we continue, make sure you have completed the [prerequisites](https://v1.tauri.app/v1/guides/getting-started/prerequisites) to have a working development environment.

Tauri is a framework to build desktop applications with any frontend framework and a Rust core. Each app consists of two parts:

1.  Rust binary that creates the windows and exposes native functionality to those windows
2.  Frontend of your choice that produces the user interface inside the window

In the following, we will first scaffold the frontend, set up the Rust project, and lastly show you how to communicate between the two.

Here's a preview of what we will be building:

![Application Preview](https://v1.tauri.app/assets/images/result-888ca764c942ed1d228e2d534b2b0699.png)

## Create the Frontend[​](#create-the-frontend 'Direct link to Create the Frontend')

[SvelteKit](https://kit.svelte.dev/) is a Svelte Frontend that is primarily designed for Server-Side Rendering (SSR). To make SvelteKit work with Tauri we are going to disable SSR and use [`@sveltejs/adapter-static`](https://github.com/sveltejs/kit/tree/master/packages/adapter-static) to create a frontend based on Static-Site Generation (SSG).

SvelteKit comes with a scaffolding utility similar to [`create-tauri-app`](https://github.com/tauri-apps/create-tauri-app) that can quickly set up a new project with many customization options. For this guide, we will select the [TypeScript](https://www.typescriptlang.org/) template, with ESLint and Prettier enabled.

- npm
- Yarn
- pnpm
- Bun

1.  _Project name_  
    This will be the name of your JavaScript project. Corresponds to the name of the folder this utility will create but has otherwise no effect on your app. You can use any name you want here.
2.  _App template_  
    We will select the `Skeleton project` for a barebones template. If you want to play around with a more complete SvelteKit example you can select `SvelteKit demo app`.
3.  _Type checking_  
    Whether you want type checking via JSDoc or TypeScript in your project. For this guide, we assume you choose TypeScript.
4.  _Code linting and formatting_  
    Whether you want to start your project with ESLint for code linting and Prettier for code formatting. There won't be other mentions about this in this guide, but we recommend enabling these 2 options.
5.  _Browser testing_  
    SvelteKit offers built-in Playwright support for browser testing. Since Tauri APIs don't work in Playwright, we recommend not adding it. Check out our [WebDriver documentation](https://v1.tauri.app/v1/guides/testing/webdriver/introduction) for alternatives using Selenium or WebdriverIO instead of Playwright.

## SvelteKit in SSG mode[​](#sveltekit-in-ssg-mode 'Direct link to SvelteKit in SSG mode')

First, we need to install [`@sveltejs/adapter-static`](https://github.com/sveltejs/kit/tree/master/packages/adapter-static):

- npm
- Yarn
- pnpm
- Bun

```
npm install --save-dev @sveltejs/adapter-static

```

Then update the `adapter` import in the `svelte.config.js` file:

svelte.config.js

```
import adapter from '@sveltejs/adapter-static' // This was changed from adapter-auto
import { vitePreprocess } from '@sveltejs/vite-plugin-svelte'

/** @type {import('@sveltejs/kit').Config} */
const config = {
  // Consult https://kit.svelte.dev/docs/integrations#preprocessors
  // for more information about preprocessors
  preprocess: vitePreprocess(),

  kit: {
    adapter: adapter(),
  },
}

export default config

```

Lastly, we need to disable SSR and enable prerendering by adding a root `+layout.ts` file (or `+layout.js` if you are not using TypeScript) with these contents:

src/routes/+layout.ts

```
export const prerender = true
export const ssr = false

```

Note that static-adapter doesn't require you to disable SSR for the whole app but it makes it possible to use APIs that depend on the global `window` object (like Tauri's API) without [Client-side checks](https://kit.svelte.dev/faq#integrations-how-do-i-use-a-client-side-only-library-that-depends-on-document-or-window).

Furthermore, if you prefer a Single-Page Application (SPA) mode over SSG, you can change the adapter configurations and `+layout.ts` according to the [adapter docs](https://github.com/sveltejs/kit/tree/master/packages/adapter-static#spa-mode).

## Create the Rust Project[​](#create-the-rust-project 'Direct link to Create the Rust Project')

At the heart of every Tauri app is a Rust binary that manages windows, the webview, and calls to the operating system through a Rust crate called `tauri`. This project is managed by [Cargo](https://doc.rust-lang.org/cargo/), the official package manager and general-purpose build tool for Rust.

Our Tauri CLI uses Cargo under the hood so you rarely need to interact with it directly. Cargo has many more useful features that are not exposed through our CLI, such as testing, linting, and formatting, so please refer to their [official docs](https://doc.rust-lang.org/cargo/commands/index.html) for more.

Install Tauri CLI

If you haven't installed the Tauri CLI yet you can do so with one of the below commands. Aren't sure which to use? Check out the [FAQ entry](about:/v1/guides/faq#node-or-cargo).

- npm
- Yarn
- pnpm
- Bun
- Cargo

```
npm install --save-dev @tauri-apps/cli@">1.0.0"

```

For npm to detect Tauri correctly you need to add it to the "scripts" section in your package.json file:

package.json

```
"scripts": {
  "tauri": "tauri"
}

```

To scaffold a minimal Rust project that is pre-configured to use Tauri, open a terminal and run the following command:

- npm
- Yarn
- pnpm
- Bun
- Cargo

It will walk you through a series of questions:

1.  _What is your app name?_  
    This will be the name of your final bundle and what the OS will call your app. You can use any name you want here.

2.  _What should the window title be?_  
    This will be the title of the default main window. You can use any title you want here.

3.  _Where are your web assets (HTML/CSS/JS) located relative to the `<current dir>/src-tauri/tauri.conf.json` file that will be created?_  
    This is the path that Tauri will load your frontend assets from when building for **production**.

    Use `../build` for this value.

4.  _What is the URL of your dev server?_  
    This can be either a URL or a file path that Tauri will load during **development**.

    Use `http://localhost:5173` for this value.

5.  _What is your frontend dev command?_  
    This is the command used to start your frontend dev server.

    Use `npm run dev` (make sure to adapt this to use the package manager of your choice).

6.  _What is your frontend build command?_  
    This is the command to build your frontend files.

    Use `npm run build` (make sure to adapt this to use the package manager of your choice).

info

If you're familiar with Rust, you will notice that `tauri init` looks and works a lot like `cargo init`. You can just use `cargo init` and add the necessary Tauri dependencies if you prefer a fully manual setup.

The `tauri init` command generates a folder called `src-tauri`. It's a convention for Tauri apps to place all core-related files into this folder. Let's quickly run through the contents of this folder:

- **`Cargo.toml`**  
  Cargo's manifest file. You can declare Rust crates your app depends on, metadata about your app, and much more. For the full reference see [Cargo's Manifest Format](https://doc.rust-lang.org/cargo/reference/manifest.html).
- **`tauri.conf.json`**  
  This file lets you configure and customize aspects of your Tauri application from the name of your app to the list of allowed APIs. See [Tauri's API Configuration](https://v1.tauri.app/v1/api/config) for the full list of supported options and in-depth explanations for each.
- **`src/main.rs`**  
   This is the entry point to your Rust program and the place where we bootstrap into Tauri. You will find two sections in it:
  src/main.rs
  ```
  #![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
  ```

fn main() {
tauri::Builder::default()
.run(tauri::generate_context!())
.expect("error while running tauri application");
}

```


    The line beginning with the [`cfg! macro`](https://doc.rust-lang.org/rust-by-example/attribute/cfg.html) serves just one purpose: it disables the command prompt window that would normally pop up on Windows if you run a bundled app. If you're on Windows, try to comment it out and see what happens.

    The `main` function is the entry point and the first function that gets invoked when your program runs.

*   **`icons`**
    Chances are you want a snazzy icon for your app! To get you going quickly, we included a set of default icons. You should switch these out before publishing your application. Learn more about the various icon formats in Tauri's [icons feature guide](https://v1.tauri.app/v1/guides/features/icons).


Now that we have scaffolded our frontend and initialized the Rust project the created `tauri.conf.json` file should look like this:

src-tauri/tauri.conf.json

```

{
"build": {
// This command will execute when you run `tauri build`.
"beforeBuildCommand": "npm run build",
// This command will execute when you run `tauri dev`
"beforeDevCommand": "npm run dev",
"devPath": "http://localhost:5173",
"distDir": "../build"
},

```


And that's it! Now you can run the following command in your terminal to start a development build of your app:

*   npm
*   Yarn
*   pnpm
*   bun
*   Cargo

![Application Window](https://v1.tauri.app/assets/images/init-0279945d40196d63bffcbdb86cf218bc.png)

Invoke Commands[​](#invoke-commands "Direct link to Invoke Commands")
---------------------------------------------------------------------

Tauri lets you enhance your frontend with native capabilities. We call these [Commands](https://v1.tauri.app/v1/guides/features/command), essentially Rust functions that you can call from your frontend JavaScript. This enables you to handle heavy processing or calls to the OS in much more performant Rust code.

Let's make a simple example:

src-tauri/src/main.rs

```

#[tauri::command]
fn greet(name: &str) -> String {
format!("Hello, {}!", name)
}

```


A Command is just like any regular Rust function, with the addition of the `#[tauri::command]` attribute macro that allows your function to communicate with the JavaScript context.

Lastly, we also need to tell Tauri about our newly created command so that it can route calls accordingly. This is done with the combination of the `.invoke_handler()` function and the `generate_handler![]` macro you can see below:

src-tauri/src/main.rs

```

fn main() {
tauri::Builder::default()
.invoke_handler(tauri::generate_handler![greet])
.run(tauri::generate_context!())
.expect("error while running tauri application");
}

```


Now you're ready to call your Command from the frontend!

To call our newly created command we will use the [`@tauri-apps/api`](https://v1.tauri.app/v1/api/js/) JavaScript library. It provides access to core functionality such as windows, the filesystem, and more through convenient JavaScript abstractions. You can install it using your favorite JavaScript package manager:

*   npm
*   Yarn
*   pnpm
*   Bun

```

npm install @tauri-apps/api@1

```


With the library installed, we can now create a new Svelte component. We'll create it in `src/lib/Greet.svelte`:

src/lib/Greet.svelte

```

<script>
  import { invoke } from '@tauri-apps/api/tauri'

  let name = ''
  let greetMsg = ''

  async function greet() {
    greetMsg = await invoke('greet', { name })
  }
</script>

<div>
  <input id="greet-input" placeholder="Enter a name..." bind:value="{name}" />
  <button on:click="{greet}">Greet</button>
  <p>{greetMsg}</p>
</div>

```


You can now add this component into `src/routes/+page.svelte`:

src/routes/+page.svelte

```

<script>
  import Greet from '../lib/Greet.svelte'
</script>

<h1>Welcome to SvelteKit</h1>
<Greet />

```


You can now run this with `npm run tauri dev` and see the result:

![Application Preview](https://v1.tauri.app/assets/images/result-888ca764c942ed1d228e2d534b2b0699.png)

tip

If you want to know more about the communication between Rust and JavaScript, please read the Tauri [Inter-Process Communication](https://v1.tauri.app/v1/references/architecture/inter-process-communication/) guide.
```
