Title: Create a Project | Tauri

Description: The cross-platform app building toolkit

Skip to content

Create a Project
================

One thing that makes Tauri so flexible is its ability to work with virtually any frontend framework. We’ve created the `create-tauri-app` utility to help you create a new Tauri project using one of the officially maintained framework templates.

`create-tauri-app` currently includes templates for vanilla (HTML, CSS and JavaScript without a framework), Vue.js, Svelte, React, SolidJS, Angular, Preact, Yew, Leptos, and Sycamore. You can also find or add your own community templates and frameworks in the Awesome Tauri repo.

Alternatively, you can add Tauri to an existing project to quickly turn your existing codebase into a Tauri app.

Using `create-tauri-app`
------------------------

Section titled “Using create-tauri-app”

To get started using `create-tauri-app` run one of the below commands in the folder you’d like to setup your project. If you’re not sure which command to use we recommend the Bash command on Linux and macOS and the PowerShell command on Windows.

*   Bash
*   PowerShell
*   Fish
*   npm
*   Yarn
*   pnpm
*   deno
*   bun
*   Cargo

```
sh <(curl https://create.tauri.app/sh)
```

```
irm https://create.tauri.app/ps | iex
```

```
sh (curl -sSL https://create.tauri.app/sh | psub)
```

```
npm create tauri-app@latest
```

```
yarn create tauri-app
```

```
pnpm create tauri-app
```

```
deno run -A npm:create-tauri-app
```

```
bun create tauri-app
```

```
cargo install create-tauri-app --lockedcargo create-tauri-app
```

Follow along with the prompts to choose your project name, frontend language, package manager, and frontend framework, and frontend framework options if applicable.

Not sure what to choose?

We recommend starting with the vanilla template (HTML, CSS, and JavaScript without a frontend framework) to get started. You can always integrate a frontend framework later.

*   Choose which language to use for your frontend: `TypeScript / JavaScript`
*   Choose your package manager: `pnpm`
*   Choose your UI template: `Vanilla`
*   Choose your UI flavor: `TypeScript`

#### Scaffold a new project

Section titled “Scaffold a new project”

1.  Choose a name and a bundle identifier (unique-id for your app):

```
? Project name (tauri-app) ›? Identifier (com.tauri-app.app) ›
```

2.  Select a flavor for your frontend. First the language:

```
? Choose which language to use for your frontend ›Rust  (cargo)TypeScript / JavaScript  (pnpm, yarn, npm, bun).NET  (dotnet)
```

3.  Select a package manager (if there are multiple available):

Options for **TypeScript / JavaScript**:

```
? Choose your package manager ›pnpmyarnnpmbun
```

4.  Select a UI Template and flavor (if there are multiple available):

Options for **Rust**:

```
? Choose your UI template ›VanillaYewLeptosSycamore
```

Options for **TypeScript / JavaScript**:

```
? Choose your UI template ›VanillaVueSvelteReactSolidAngularPreact
? Choose your UI flavor ›TypeScriptJavaScript
```

Options for **.NET**:

```
? Choose your UI template ›Blazor  (https://dotnet.microsoft.com/en-us/apps/aspnet/web-apps/blazor/)
```

Once completed, the utility reports that the template has been created and displays how to run it using the configured package manager. If it detects missing dependencies on your system, it prints a list of packages and prompts how to install them.

#### Start the development server

Section titled “Start the development server”

After `create-tauri-app` has completed, you can navigate into your project’s folder, install dependencies, and then use the Tauri CLI to start the development server:

*   npm
*   yarn
*   pnpm
*   deno
*   bun
*   cargo

```
cd tauri-appnpm installnpm run tauri dev
```

```
cd tauri-appyarn installyarn tauri dev
```

```
cd tauri-apppnpm installpnpm tauri dev
```

```
cd tauri-appdeno installdeno task tauri dev
```

```
cd tauri-appbun installbun tauri dev
```

```
cd tauri-appcargo tauri dev
```

You’ll now see a new window open with your app running.

**Congratulations!** You’ve made your Tauri app! 🚀

Manual Setup (Tauri CLI)
------------------------

Section titled “Manual Setup (Tauri CLI)”

If you already have an existing frontend or prefer to set it up yourself, you can use the Tauri CLI to initialize the backend for your project separately.

Note

The following example assumes you are creating a new project. If you’ve already initialized the frontend of your application, you can skip the first step.

1.  Create a new directory for your project and initialize the frontend. You can use plain HTML, CSS, and JavaScript, or any framework you prefer such as Next.js, Nuxt, Svelte, Yew, or Leptos. You just need a way of serving the app in your browser. Just as an example, this is how you would setup a simple Vite app:

*   npm
*   yarn
*   pnpm
*   deno
*   bun

```
mkdir tauri-appcd tauri-appnpm create vite@latest .
```

```
mkdir tauri-appcd tauri-appyarn create vite .
```

```
mkdir tauri-appcd tauri-apppnpm create vite .
```

```
mkdir tauri-appcd tauri-appdeno run -A npm:create-vite .
```

```
mkdir tauri-appcd tauri-appbun create vite
```

2.  Then, install Tauri’s CLI tool using your package manager of choice. If you are using `cargo` to install the Tauri CLI, you will have to install it globally.

*   npm
*   yarn
*   pnpm
*   deno
*   bun
*   cargo

```
npm install -D @tauri-apps/cli@latest
```

```
yarn add -D @tauri-apps/cli@latest
```

```
pnpm add -D @tauri-apps/cli@latest
```

```
deno add -D npm:@tauri-apps/cli@latest
```

```
bun add -D @tauri-apps/cli@latest
```

```
cargo install tauri-cli --version "^2.0.0" --locked
```

3.  Determine the URL of your frontend development server. This is the URL that Tauri will use to load your content. For example, if you are using Vite, the default URL is `http://localhost:5173`.

4.  In your project directory, initialize Tauri:

*   npm
*   yarn
*   pnpm
*   deno
*   bun
*   cargo

```
npx tauri init
```

```
yarn tauri init
```

```
pnpm tauri init
```

```
deno task tauri init
```

```
bun tauri init
```

```
cargo tauri init
```

After running the command it will display a prompt asking you for different options:

```
✔ What is your app name? tauri-app✔ What should the window title be? tauri-app✔ Where are your web assets located? ..✔ What is the url of your dev server? http://localhost:5173✔ What is your frontend dev command? pnpm run dev✔ What is your frontend build command? pnpm run build
```

This will create a `src-tauri` directory in your project with the necessary Tauri configuration files.

5.  Verify your Tauri app is working by running the development server:

*   npm
*   yarn
*   pnpm
*   deno
*   bun
*   cargo

```
npx tauri dev
```

```
yarn tauri dev
```

```
pnpm tauri dev
```

```
deno task tauri dev
```

```
bun tauri dev
```

```
cargo tauri dev
```

This command will compile the Rust code and open a window with your web content.

**Congratulations!** You’ve created a new Tauri project using the Tauri CLI! 🚀

Next Steps
----------

Section titled “Next Steps”

*   Add and Configure a Frontend Framework
*   Tauri Command Line Interface (CLI) Reference
*   Learn how to build your Tauri app
*   Discover additional features to extend Tauri

© 2025 Tauri Contributors. CC-BY / MIT