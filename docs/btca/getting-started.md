Description: btca: CLI for asking questions about codebases.

Title: Better Context

Getting started Install, configure, and use btca effectively.

# Getting started with btca

Install `btca`, add it to your agent rules, and start asking questions

## Install

Install globally with Bun, then run `btca --help`.

```
bun add -g btca
btca
```

Ships with these repos by default

- `svelte` `main` https://github.com/sveltejs/svelte.dev

- `tailwindcss` `main` https://github.com/tailwindlabs/tailwindcss.com

- `nextjs` `canary` https://github.com/vercel/next.js

## Add it to a project

Paste this into your project's `AGENTS.md` so your agent knows when to use btca. Make sure you update the list of technologies to match the ones you have added to btca config and need in this project.

## Using btca

Most of the time you'll use `ask`. Use `chat` for an interactive session, and `serve` when you want an HTTP API.

Ask

Answer a single question

```
btca ask -t svelte -q "How do stores work in Svelte 5?"
```

Chat

Open an interactive session

```
btca chat -t svelte
```

Serve

Expose an HTTP endpoint for questions.

```
btca serve -p 8081
```

POST `/question` with `{"tech","question"}`.

## Add tech to btca

Click a technology to copy the command that adds it to your btca config.

Effect

btca config repos add -n effect -u https://github.com/Effect-TS/effect -b main

Copy

React

btca config repos add -n react -u https://github.com/facebook/react -b main

Copy

Vue

btca config repos add -n vue -u https://github.com/vuejs/core -b main

Copy

Daytona

btca config repos add -n daytona -u https://github.com/daytonaio/daytona -b main

Copy

OpenCode

btca config repos add -n opencode -u https://github.com/opencode-ai/opencode -b main

Copy

neverthrow

btca config repos add -n neverthrow -u https://github.com/supermacro/neverthrow -b master

Copy

Runed

btca config repos add -n runed -u https://github.com/svelte-plugins/runed -b main

Copy

## Set the model

You can set provider + model via `btca config model`. I recommend using Haiku for fast, cheap answers.

```
btca config model -p anthropic -m claude-haiku-4-5
```

## Full config

The config lives at `~/.config/btca/btca.json`. You can print the path by running `btca config`.

```
{
"promptsDirectory": "~/.config/btca/prompts",
"reposDirectory": "~/.config/btca/repos",
"port": 3420,
"maxInstances": 5,
"repos": [
{
"name": "svelte",
"url": "https://github.com/sveltejs/svelte.dev",
"branch": "main",
"specialNotes": "This is the svelte docs website repo, not the actual svelte repo. Use the docs to answer questions about svelte."
},
{
"name": "tailwindcss",
"url": "https://github.com/tailwindlabs/tailwindcss.com",
"branch": "main",
"specialNotes": "This is the tailwindcss docs website repo, not the actual tailwindcss repo. Use the docs to answer questions about tailwindcss."
},
{
"name": "nextjs",
"url": "https://github.com/vercel/next.js",
"branch": "canary"
}
],
"model": "claude-haiku-4-5",
"provider": "anthropic"
}
```
