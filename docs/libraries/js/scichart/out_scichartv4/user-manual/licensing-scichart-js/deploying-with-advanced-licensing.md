On this page

# Deploying SciChart.js with Advanced Licensing (OEM)

![](out_scichartv4/user-manual/licensing-scichart-js/deploying-with-advanced-licensing/index_media/af50301a53fa555616a9b2279e7e25a1d566cb1a.svg)info

To allow deployment of SciChart.js to OEM applications, apps where the runtime domain is unknown or `localhost` without a watermark and to comply with the <a href="https://www.scichart.com/scichart-eula" rel="noopener noreferrer" target="_blank">terms for commercial use</a>, you will need to setup an advanced license for your app.

Products purchased from our store that enable SciChart.js advanced licensing include:

- SciChart Bundle 2D Pro
- SciChart Bundle 2D/3D Pro
- SciChart Bundle 2D/3D Source

For to enable Advanced Licensing, you will need to talk to <a href="https://www.scichart.com/contact-us#pre-sales" rel="noopener noreferrer" target="_blank">technical sales</a> and for licensing support post-sales please <a href="https://www.scichart.com/contact-us#tech-support" rel="noopener noreferrer" target="_blank">contact technical support</a>.

Standard scichart.js licenses allow for production deployment to a fixed host name, which is not `localhost`. If you are building an application that will be deployed by third parties to hosts you do not know or control (ie OEM scenarios) or if you are building an embedded system that has to run on `localhost`, then you will need one of our **Advanced Licensing solutions**.

Advanced licensing requires a Bundle license and a commitment to maintain an active license for the lifetime of the project. For full details please see the knowlegebase article <a href="https://support.scichart.com/support/solutions/articles/101000516558-scichart-standard-advanced-licensing" rel="noopener noreferrer" target="_blank">SciChart Advanced Licensing</a>.

Once the necessary license type and agreement is in place, Advanced Licensing will be enabled for your license. This adds new functionality to the Licenses section of the <a href="https://www.scichart.com/my-account" rel="noopener noreferrer" target="_blank">scichart.com/my-account</a> page which will enable you to generate the key pairs needed.

![](out_scichartv4/user-manual/licensing-scichart-js/deploying-with-advanced-licensing/index_media/f0173787a207cc0a720ca619ab3e16cfe886b3c7.svg)tip

Before trying to implement any of these solutions we recommend <a href="https://www.scichart.com/contact-us#tech-support" rel="noopener noreferrer" target="_blank">submitting a support request</a> with details of your intended deployment, including the host requirement, the client and server tech stack and the target platform and architecture (eg windows/linux, x86/x64/arm/arm64), and we will make sure you get the correct solution.

## How Advanced Licensing Works<a href="https://www.scichart.com/documentation/js/v4/user-manual/licensing-scichart-js/deploying-with-advanced-licensing/#how-advanced-licensing-works" class="hash-link" aria-label="Direct link to How Advanced Licensing Works" translate="no" title="Direct link to How Advanced Licensing Works">â€‹</a>

You will host a license key in the server of your application, and a client key set on the client. These will communicate to unlock SciChart.js for every domain, including `localhost`. The advanced licensing works offline and does not require an internet connection.

![](out_scichartv4/user-manual/licensing-scichart-js/deploying-with-advanced-licensing/index_media/0f0874621de662630fd5d7f09c89455c4ace26e6.svg)warning

Once deployed, an advanced license will work in perpetuity, however our <a href="https://support.scichart.com/support/solutions/articles/101000516558-scichart-standard-advanced-licensing" rel="noopener noreferrer" target="_blank">advanced licensing agreement</a> requires you maintain least one active developer subscription during the lifetime of the application. This is a really low fee, and ensures ongoing maintenance of our systems & technical support for OEM use-cases.

## Generating an advanced license client key / server key pair<a href="https://www.scichart.com/documentation/js/v4/user-manual/licensing-scichart-js/deploying-with-advanced-licensing/#generating-an-advanced-license-client-key--server-key-pair" class="hash-link" aria-label="Direct link to Generating an advanced license client key / server key pair" translate="no" title="Direct link to Generating an advanced license client key / server key pair">â€‹</a>

1.  Head over to <a href="https://www.scichart.com/my-account/" rel="noopener noreferrer" target="_blank">scichart.com/my-account</a> to administer your license keys (Need help? See [location and management of license keys](https://www.scichart.com/documentation/js/v4/user-manual/licensing-scichart-js/where-are-my-license-keys/))
2.  In the section **Orders & Keys** - **Manage Licenses** - **Hostnames** set a server assembly name or app name, with the drop-down value "OEM or Embedded License"

![](out_scichartv4/user-manual/licensing-scichart-js/deploying-with-advanced-licensing/index_media/af50301a53fa555616a9b2279e7e25a1d566cb1a.svg)info

For dotnet server, this must be the actual assembly name of your .net server application. Else, it can be any application name or ID

<img src="out_scichartv4/user-manual/licensing-scichart-js/deploying-with-advanced-licensing/index_media/6cb318784f37a7001c45986597d4990642e5d618.png" class="img_ev3q" decoding="async" loading="lazy" width="1363" height="1247" alt="Set server assembly name or app name for advanced licensing - scichart.js" />

3.  In the section **Orders & Keys** - **Manage Licenses** - **Runtime License Key** you can now generate a client/server key pair for your app.

<img src="out_scichartv4/user-manual/licensing-scichart-js/deploying-with-advanced-licensing/index_media/3977116c102469c28adf7b36a90f5688c88b4024.png" class="img_ev3q" decoding="async" loading="lazy" width="805" height="944" alt="Generate an advanced license client-server key pair - scichart.js" />

## Including advanced license keys in your app<a href="https://www.scichart.com/documentation/js/v4/user-manual/licensing-scichart-js/deploying-with-advanced-licensing/#including-advanced-license-keys-in-your-app" class="hash-link" aria-label="Direct link to Including advanced license keys in your app" translate="no" title="Direct link to Including advanced license keys in your app">â€‹</a>

The actual implementation depends on your tech stack, however we have helpful examples for dotnet server, nodejs server or a self-hosted licensing server over at <a href="https://github.com/ABTSoftware/SciChart.JS.Examples/tree/master/AdvancedLicensing" rel="noopener noreferrer" target="_blank">our Github</a>

![](out_scichartv4/user-manual/licensing-scichart-js/deploying-with-advanced-licensing/index_media/af50301a53fa555616a9b2279e7e25a1d566cb1a.svg)info

The git repo <a href="https://github.com/ABTSoftware/SciChart.JS.Examples/tree/master/AdvancedLicensing" rel="noopener noreferrer" target="_blank">github.com/ABTSoftware/SciChart.JS.Examples/tree/master/AdvancedLicensing</a> contains the code you need to setup and enable an advanced license in your system

### Advanced licensing for .net (dotnet) server<a href="https://www.scichart.com/documentation/js/v4/user-manual/licensing-scichart-js/deploying-with-advanced-licensing/#advanced-licensing-for-net-dotnet-server" class="hash-link" aria-label="Direct link to Advanced licensing for .net (dotnet) server" translate="no" title="Direct link to Advanced licensing for .net (dotnet) server">â€‹</a>

We have a folder in our <a href="https://github.com/ABTSoftware/SciChart.JS.Examples/tree/master/AdvancedLicensing" rel="noopener noreferrer" target="_blank">Github Repository here</a> called `dotnet-server-licensing` with a detailed `Readme.md` and example test app that you can use to test out your Advanced Licensing keys.

For the dotnet server example, the client/server key pair must be generated using the server entry assembly name as an OEM or Embedded License App name. For this demo that would be `DotnetServerLicensing`.

### Advanced licensing for nodejs server<a href="https://www.scichart.com/documentation/js/v4/user-manual/licensing-scichart-js/deploying-with-advanced-licensing/#advanced-licensing-for-nodejs-server" class="hash-link" aria-label="Direct link to Advanced licensing for nodejs server" translate="no" title="Direct link to Advanced licensing for nodejs server">â€‹</a>

We have a folder in our <a href="https://github.com/ABTSoftware/SciChart.JS.Examples/tree/master/AdvancedLicensing" rel="noopener noreferrer" target="_blank">Github Repository here</a> called `nodejs-server-licensing` with a detailed `Readme.md` and example test app that you can use to test out your Advanced Licensing keys.

For the nodejs example, the client/server key pair must be generated using the `APP_NAME`, e.g. in this demo that would be `scichart-nodejs-server-licensing`.

### Advanced licensing for any server environment<a href="https://www.scichart.com/documentation/js/v4/user-manual/licensing-scichart-js/deploying-with-advanced-licensing/#advanced-licensing-for-any-server-environment" class="hash-link" aria-label="Direct link to Advanced licensing for any server environment" translate="no" title="Direct link to Advanced licensing for any server environment">â€‹</a>

We have a folder in our <a href="https://github.com/ABTSoftware/SciChart.JS.Examples/tree/master/AdvancedLicensing" rel="noopener noreferrer" target="_blank">Github Repository here</a> called `SciChartLicenseServer` with a detailed `Readme.md` and C++ assemblies that you can include in any server environment, e.g. Java, Python, PhP etc.

For the C++ License server assemblies, set any desired App Name as an OEM or Embedded License App name when generating client/server key pairs in My-Account. After that, call `SetAssemblyName()` on the server with the same string, call `SetRuntimeLicenseKey()` on the server passing the server key.

The first time a chart is created on the client, a validation challenge is generated using the client key and this is sent to the server (by default to `/api/license?orderid={orderId}&challenge={challenge}`). The server needs to pass the challenge to the SciChart native library which has the server key set, and return the response to the client. The result is the application can be deployed to any domain, including `localhost`.

![](out_scichartv4/user-manual/licensing-scichart-js/deploying-with-advanced-licensing/index_media/af50301a53fa555616a9b2279e7e25a1d566cb1a.svg)info

Note, for advanced licensing the communication is only between the client and its originating server. It does not require outside internet access. The validation result is stored in a cookie on the client, so this validation only needs to occur once per week per client.

<a href="https://github.com/ABTSoftware/SciChart.JS.Docs/tree/dev/docusaurus/docs/user-manual/licensing-scichart-js/deploying-with-advanced-licensing/index.md" class="theme-edit-this-page" rel="noopener noreferrer" target="_blank"><img src="out_scichartv4/user-manual/licensing-scichart-js/deploying-with-advanced-licensing/index_media/42a87493f9569d393738cbffa7bed03324e68916.svg" class="iconEdit_Z9Sw" />Edit this page</a>
