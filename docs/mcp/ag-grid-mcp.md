AG Grid Model Context Protocol (MCP) Server

javascript logoJavaScript
AG Grid's Model Context Protocol (MCP) server provides AI Agents with framework and version specific knowledge to help developers integrate and maintain their AG Grid code.


Overview
Copy Link
The core feature of the AG MCP Server is an LLM optimized search tool which will provide additional context, (e.g. version specific documentation, examples, and references), in a condensed markdown format, minimising the amount of context used.

More specifically, the AG MCP Server is made up of Tools, Prompts and Resources:

Tools - Schema defined interfaces that enable AI models to perform actions, including searching the docs and detecting & setting the current AG Grid version.
Prompts - Pre-configured actions that allow you to perform common actions, such as creating new grids, or migrating to later versions.
Resources - Documentation, examples, and API definitions/interfaces in condensed markdown.
Each of these components are available to your LLM whenever it needs more information on how to implement AG Grid features, and can also be accessed directly.

Architecture
Copy Link
At a high-level, the AG MCP Server works as follows:

Prompt submitted to LLM client, e.g. Claude, Cursor, CoPilot, ChatGPT, etc...
LLM accesses ag-mcp server and requests additional context, via a tool call, pre-built prompt, or by requesting specific resources.
ag-mcp uses an LLM optimised search tool to return version-specific information, such as docs, examples, and api refs in condensed markdown.
LLM uses this additional context to provide more accurate responses.
AG MCP Architecture
Installation
Copy Link
To install and use ag-mcp with your LLM client, provide the npx ag-mcp command in the normal process for adding an MCP Server.

Cursor
VS Code (CoPilot)
Claude
Click Install in Cursor to open Cursor and automatically add the ag-mcp server

To install manually, create a mcp.json file in the root of your project and add the following:

{
  "mcpServers": {
    "ag-mcp": {
      "command": "npx",
      "args": ["ag-mcp"]
    }
  }
}
To learn more, see the Cursor MCP documentation

Getting Started
Copy Link
Once the MCP is installed, your LLM will automatically leverage the AG MCP Server to access additional context, based on your prompt. Tools, Prompts and Resources can also be accessed directly, as demonstrated below.

Refer to your LLM documentation for specific instructions on accessing MCP features.

Prompts
Copy Link
Prompts are pre-configured actions that allow you to perform common actions, such as creating a new grid, or migrating to a later version.

AG MCP currently provides two prompts:

quick-start - Get started with AG Grid in any framework
upgrade-grid - Migrate to a newer version of AG Grid
Create a New Grid
Copy Link
The quick-start prompt can be triggered directly from your LLM client. This creates a list of instructions for your LLM to follow when creating a new AG Grid project or adding AG Grid to an existing project.

You can pass additional context, requirements or instructions to your LLM as arguments to this prompt to fine tune the type of data grid you want the LLM to create.

Cursor
VS Code (CoPilot)
Claude
To trigger the quick-start prompt, enter the following command in your LLM prompt:

/ag-mcp:quick-start {additional-context}
Migrate to a New Version
Copy Link
The upgrade-grid prompt creates a step by step plan to help migrate from your current version to the provided version. This is given to the LLM to execute, calling back to the MCP server as needed. It takes a version by version approach, making sure each version is correct before continuing.

To trigger the upgrade-grid prompt, enter the following command in your LLM prompt:

Cursor
VS Code (CoPilot)
Claude
/ag-mcp:upgrade-grid {additional-context}
Resources (Docs, API & Example Search)
Copy Link
Resources allow servers to share data that provides context to language models, such as files, database schemas, or application-specific information.

There are currently three sets of resources available:

articles - Access to the full AG Grid documentation at the correct version.
definitions - API definitions and interfaces
examples - A library of AG Grid implementation examples in your framework.
Your LLM can access these resources whenever it needs more information on how to implement AG Grid features.

To directly reference articles, definitions or examples, use the following steps depending on your LLM provider of choice:

VS Code (CoPilot)
Claude
To add a resource from an MCP server to your chat prompt:

In the Chat view, select Add Context > MCP Resources

Select a resource type from the list and provide optional resource input parameters.

VS Code Docs

Tools
Copy Link
Tools are schema-defined interfaces that enable AI models to perform actions. Each tool defines a specific operation with typed inputs and outputs, and the model automatically requests tool execution based on context.

Tools can be called manually by entering the tool name as a prompt into your LLM, and passing the relevant params. Refer to your LLM documentation for specific instructions.

AG-MCP currently provides four tools:

search_docs - Search the documents for the currently installed version of AG Grid.
detect_version - Infers the version and framework of AG Grid installed in your repo.
set_version - Manually set the version of your repo (useful in monorepos).
list_versions - List all available AG Grid versions from the API to see what versions are available for migration or reference.
Search Docs
Copy Link
Search AG Grid documentation for the detected or latest version. Use this to find details on features, APIs, configurations, and troubleshooting. Supports natural language queries.

Parameters Copy Link
Name	Type	Required	Description
query	string	Yes	Search term ("column sorting", "cell renderers", "data grid performance", etc...)
version	string	No	Override the detected AG Grid version
framework	string	No	Override the detected framework
Detect Version
Copy Link
Detect the AG Grid version and framework in the current project by analyzing package.json and dependencies. Use this to understand the project setup.

Parameters Copy Link
Name	Type	Required	Description
path	string	No	Path to the project directory (defaults to current workspace)
Set Version
Copy Link
Set the AG Grid version and framework to use for documentation searches and resources. Use this when working with a specific version or framework combination.

Parameters Copy Link
Name	Type	Required	Description
version	string	Yes	AG Grid version to use (e.g., "34.1.0")
framework	string	Yes	Framework to use for documentation (react, angular, vue, vanilla)
Config
Copy Link
Config, such as project roots and versions, is stored in your cache folder. For example, in MacOS it will be stored in ~/Library/Preferences/ag-mcp.