Installation

javascript logoJavaScript
AG Grid is available for download from NPM, CDN or as a direct download, however, we strongly recommend using either NPM or CDN. When using NPM, you need to import the package and register the modules you want to use.

NPM Installation
Copy Link
Install the ag-grid-community package:

npm install ag-grid-community
To use AG Grid Enterprise features, install the ag-grid-enterprise package:

npm install ag-grid-enterprise
You can test AG Grid Enterprise locally without a licence. To test in production, access support, and remove the watermark and console warnings, request a trial licence.

Registering Modules
Copy Link
Register the AllCommunityModule to access all Community features:

import { AllCommunityModule, ModuleRegistry } from 'ag-grid-community';

// Register all Community features
ModuleRegistry.registerModules([AllCommunityModule]);
Register the AllEnterpriseBundle to access all Community and Enterprise features:

import { ModuleRegistry } from 'ag-grid-community';
import { AllEnterpriseModule } from 'ag-grid-enterprise';

// Register all Community and Enterprise features
ModuleRegistry.registerModules([AllEnterpriseModule]);
To minimize bundle size, only register the modules you want to use. See the Selecting Modules docs for more information.

Importing
Copy Link
Import createGrid from the ag-grid-community package to create a new data grid:

import { createGrid } from 'ag-grid-community';
CDN Installation
Copy Link
To install AG Grid Community, include the following script tags in your HTML file:

<script src="https://cdn.jsdelivr.net/npm/ag-grid-community@34.2.0/dist/ag-grid-community.min.js"></script>

To install AG Grid Enterprise, include the following script tags in your HTML file:

<script src="https://cdn.jsdelivr.net/npm/ag-grid-enterprise@34.2.0/dist/ag-grid-enterprise.min.js"></script>

When installing AG Grid via a CDN, all modules are automatically registered for you. You can then access AG Grid via the agGrid global variable. See the Quick Start for more information.

Download
Copy Link
If your project does not use package manager and you don't want to refer AG Grid from CDN, you can download AG Grid's source files and keep them in your project structure.

Downloading AG Grid makes upgrading more complex and prone to errors. We recommend using AG Grid from an NPM package or from CDN.

Community
Enterprise
You can download AG Grid Community from GitHub Repository.

Once unpacked you'll see four bundle files in within the dist folder:

dist/ag-grid-community.js — standard bundle containing JavaScript and CSS
dist/ag-grid-community.min.js — minified bundle containing JavaScript and CSS
dist/ag-grid-community.noStyle.js — standard bundle containing JavaScript without CSS
dist/ag-grid-community.min.noStyle.js — minified bundle containing JavaScript without CSS
Should you decide to use the noStyle versions of the bundle, the style sheet files are present in the styles folder.

After downloading the bundles, you can refer to the files in the same way as you would from CDN. See the Quick Start for more information.
