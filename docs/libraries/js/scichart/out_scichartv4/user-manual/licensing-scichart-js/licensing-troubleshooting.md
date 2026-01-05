On this page

# SciChart.js Licensing Troubleshooting

![](out_scichartv4/user-manual/licensing-scichart-js/licensing-troubleshooting/index_media/af50301a53fa555616a9b2279e7e25a1d566cb1a.svg)info

If you have purchased commerical licenses of SciChart.js and cannot manage to get your application working with the license applied, please check out our licensing troubleshooting steps below

## Step 1: Enabling License Debugging<a href="https://www.scichart.com/documentation/js/v4/user-manual/licensing-scichart-js/licensing-troubleshooting/#step-1-enabling-license-debugging" class="hash-link" aria-label="Direct link to Step 1: Enabling License Debugging" translate="no" title="Direct link to Step 1: Enabling License Debugging">â€‹</a>

![](out_scichartv4/user-manual/licensing-scichart-js/licensing-troubleshooting/index_media/0f0874621de662630fd5d7f09c89455c4ace26e6.svg)warning

Do this step "Enabling License Debugging" First before contacting support!

### Enable License Debugging in Browser Dev Tools<a href="https://www.scichart.com/documentation/js/v4/user-manual/licensing-scichart-js/licensing-troubleshooting/#enable-license-debugging-in-browser-dev-tools" class="hash-link" aria-label="Direct link to Enable License Debugging in Browser Dev Tools" translate="no" title="Direct link to Enable License Debugging in Browser Dev Tools">â€‹</a>

1.  In your application, open up dev tools.
2.  Find the `Application` tab and under local storage
3.  Set a flag `LICENSE_DEBUG = 1`. Or, put the following code snippet in your app. This will output license debug information to the console.

<img src="out_scichartv4/user-manual/licensing-scichart-js/licensing-troubleshooting/index_media/c056c7da9a152f4c74e0f6f2b62219d56f6040e0.png" class="img_ev3q" decoding="async" loading="lazy" width="504" height="214" alt="SciChart.js License Troubleshooting enable License Debug" />

4.  Reload the page
5.  SciChart license debug info will be dumped to the console:

<img src="out_scichartv4/user-manual/licensing-scichart-js/licensing-troubleshooting/index_media/0b7d6a8ed78f19a16771b5c4cc0b07b2b505f38a.png" class="img_ev3q" decoding="async" loading="lazy" width="632" height="631" alt="SciChart.js License Troubleshooting License Debug Output" />

### Enable License Debugging in Code<a href="https://www.scichart.com/documentation/js/v4/user-manual/licensing-scichart-js/licensing-troubleshooting/#enable-license-debugging-in-code" class="hash-link" aria-label="Direct link to Enable License Debugging in Code" translate="no" title="Direct link to Enable License Debugging in Code">â€‹</a>

It's also possible to enable license debug in code, for Electron / Tauri apps where its hard to open dev tools. Add this code snippet to your application and re-build.

``` prism-code
import { setIsDebugLicensing } from "scichart";

setIsDebugLicensing(true);
```

### Example of License Debug output & Messages<a href="https://www.scichart.com/documentation/js/v4/user-manual/licensing-scichart-js/licensing-troubleshooting/#example-of-license-debug-output--messages" class="hash-link" aria-label="Direct link to Example of License Debug output &amp; Messages" translate="no" title="Direct link to Example of License Debug output &amp; Messages">â€‹</a>

An example of a healthy license debug output can be found below:

``` prism-code
applyLicense 2D
applyLicense running
Initial license status is Community
Runtime license found
Runtime license status is Full
license ok
checkstatus: LicenseOK
SciChart Debug dump
. Is License Valid: 1
. Is Debugging Allowed: 0
. ExpiryDate: 2026-01-10T00:00:00
. License Days Remaining: 122
. License Type: Full
. Order ID: SciChart.js Demo
. Product Code: SC-JS-SDK-PRO
. Full Features:JS-2D JS-3D
. Trial Features:
. License hostnames: scichart.com;demo.scichart.com;stagingdemo.scichart.com;stagingdemo2.scichart.com;staging.scitrader.io;
. Is Server: false
. Requres Validation: false
.. Errors: 0
. RuntimeChecker:
... Is Debugger Attached: 0
... Is IDE Tool: 0
... Machine Id www.scichart.com
```

If any errors or warnings are found in the `LICENSE_DEBUG` output, this can guide you to what the problem is and how to solve it. Pass this license debug output to <a href="https://www.scichart.com/contact-us/#tech-support" rel="noopener noreferrer" target="_blank">tech support</a> so we can assist.

![](out_scichartv4/user-manual/licensing-scichart-js/licensing-troubleshooting/index_media/af50301a53fa555616a9b2279e7e25a1d566cb1a.svg)info

The LICENSE_DEBUG output will tell you the support expiry date, the order ID, the product code, features and hostnames (domains) enabled for this license key. This will help you debug & troubleshoot further.

## Step 2: Debugging Common Licensing Issues<a href="https://www.scichart.com/documentation/js/v4/user-manual/licensing-scichart-js/licensing-troubleshooting/#step-2-debugging-common-licensing-issues" class="hash-link" aria-label="Direct link to Step 2: Debugging Common Licensing Issues" translate="no" title="Direct link to Step 2: Debugging Common Licensing Issues">â€‹</a>

### I see a trial / community watermark in local development<a href="https://www.scichart.com/documentation/js/v4/user-manual/licensing-scichart-js/licensing-troubleshooting/#i-see-a-trial--community-watermark-in-local-development" class="hash-link" aria-label="Direct link to I see a trial / community watermark in local development" translate="no" title="Direct link to I see a trial / community watermark in local development">â€‹</a>

When `Is IDE Tool: 1` in the license debug output, SciChart.js has detected you are running on a local development machine (`localhost`) aka you are developing an application locally.

In this case you will need to:

1.  **Activate a developer license** on your development PC following the steps from [here](https://www.scichart.com/documentation/js/v4/user-manual/licensing-scichart-js/activating-paid-licenses/)
2.  **Ensure your support expiry date** (see `ExpiryDate` in License Debug) is later than the release date of the library version you are using.
3.  **Keep the Licensing Wizard application open** or minimise to system tray (for periodic license checks)

![](out_scichartv4/user-manual/licensing-scichart-js/licensing-troubleshooting/index_media/af50301a53fa555616a9b2279e7e25a1d566cb1a.svg)info

Once a developer license is activated, it will work in perpetuity (does not require an internet connection). However, periodically SciChart.js will ping the local licensing wizard in development mode only, which has a cached downloaded version of the developer license, hence leave it open or minimise to system tray.

![](out_scichartv4/user-manual/licensing-scichart-js/licensing-troubleshooting/index_media/0f0874621de662630fd5d7f09c89455c4ace26e6.svg)warning

If you update SciChart.js in your app to a version that was released after your support-expiry date, the application will default to community license in local dev.

To resolve this, either roll-back SciChart.js to a version released before your support-expiry, or renew a developer subscription.

Once renewed, then re-activate your developer license following steps from [here](https://www.scichart.com/documentation/js/v4/user-manual/licensing-scichart-js/activating-paid-licenses/) before updating SciChart.js in local dev.

### I see a trial / community watermark in my live deployed app<a href="https://www.scichart.com/documentation/js/v4/user-manual/licensing-scichart-js/licensing-troubleshooting/#i-see-a-trial--community-watermark-in-my-live-deployed-app" class="hash-link" aria-label="Direct link to I see a trial / community watermark in my live deployed app" translate="no" title="Direct link to I see a trial / community watermark in my live deployed app">â€‹</a>

When `Is IDE Tool: 0` in the license debug output, SciChart is running on a live domain. You may see the error in `LICENSE_DEBUG` as follows:

``` prism-code
Runtime license is invalid: License is not valid for this domain.
Expected: somedomain.com, Actual: anotherdomain.com
```

In this case you will need to:

1.  **Generate a runtime key with the correct domains** by following steps from [here](https://www.scichart.com/documentation/js/v4/user-manual/licensing-scichart-js/deploying-to-live-sites/).
2.  **Set the updated runtime license key** in your application.
3.  **Redeploy** your application.

![](out_scichartv4/user-manual/licensing-scichart-js/licensing-troubleshooting/index_media/af50301a53fa555616a9b2279e7e25a1d566cb1a.svg)info

Once a runtime key is deployed correctly, it will work in perpetuity (does not require an internet connection and won't ever fallback to community / trial version).

When deployed to a domain, SciChart.js checks the local runtime key set by calling `sciChartSurface.setRuntimeLicenseKey("--Your-Key--");`. No licensing wizard or developer license is needed on deployed servers or machines.

![](out_scichartv4/user-manual/licensing-scichart-js/licensing-troubleshooting/index_media/0f0874621de662630fd5d7f09c89455c4ace26e6.svg)warning

If you update SciChart.js in your app and redeploy, ensure the version of SciChart.js was released before your support expiry date (`ExpiryDate` in License Debug).

If you renew a SciChart.js subscription, regenerate runtime keys when you next apply a SciChart.js library version update.

Otherwise, the deployed application may default to community license

### I see a trial / community watermark in my advanced licensing (OEM / Embedded) app<a href="https://www.scichart.com/documentation/js/v4/user-manual/licensing-scichart-js/licensing-troubleshooting/#i-see-a-trial--community-watermark-in-my-advanced-licensing-oem--embedded-app" class="hash-link" aria-label="Direct link to I see a trial / community watermark in my advanced licensing (OEM / Embedded) app" translate="no" title="Direct link to I see a trial / community watermark in my advanced licensing (OEM / Embedded) app">â€‹</a>

For advanced licensing, the field `Is Server: true` will be present in the `LICENSE_DEBUG` output.

1.  Ensure the client/server key pair was generated ([see steps](https://www.scichart.com/documentation/js/v4/user-manual/licensing-scichart-js/deploying-with-advanced-licensing/#generating-an-advanced-license-client-key--server-key-pair)) using the Assembly name or App name you expect
2.  You can view the License hostnames in the debug output when `LICENSE_DEBUG = 1` ([see steps](https://www.scichart.com/documentation/js/v4/user-manual/licensing-scichart-js/licensing-troubleshooting/#step-1-enabling-license-debugging))
3.  Check the network tab of devtools to see if the client is making the validation request to the server, if the endpoint is what you expect, and what the response is.

If you still experience problems, contact tech support with your entire license debug output.

### I see a trial / community watermark in my test domain<a href="https://www.scichart.com/documentation/js/v4/user-manual/licensing-scichart-js/licensing-troubleshooting/#i-see-a-trial--community-watermark-in-my-test-domain" class="hash-link" aria-label="Direct link to I see a trial / community watermark in my test domain" translate="no" title="Direct link to I see a trial / community watermark in my test domain">â€‹</a>

Test domains added to the runtime key display a watermark by design. See the page on [domain licensing](https://www.scichart.com/documentation/js/v4/user-manual/licensing-scichart-js/deploying-to-live-sites/) to find out more.

## Step 3: Where to get further help & support<a href="https://www.scichart.com/documentation/js/v4/user-manual/licensing-scichart-js/licensing-troubleshooting/#step-3-where-to-get-further-help--support" class="hash-link" aria-label="Direct link to Step 3: Where to get further help &amp; support" translate="no" title="Direct link to Step 3: Where to get further help &amp; support">â€‹</a>

If you require further urgent assistance with SciChart.js licensing, <a href="https://www.scichart.com/contact-us/#tech-support" rel="noopener noreferrer" target="_blank">contact tech support</a> sending us the following info:

![](out_scichartv4/user-manual/licensing-scichart-js/licensing-troubleshooting/index_media/f0173787a207cc0a720ca619ab3e16cfe886b3c7.svg)tip

Send the following info to <a href="https://www.scichart.com/contact-us/#tech-support" rel="noopener noreferrer" target="_blank">tech support</a> to resolve issues with licensing.

1.  Entire `LICENSE_DEBUG` output from `applyLicense2D` onwards ([see steps](https://www.scichart.com/documentation/js/v4/user-manual/licensing-scichart-js/licensing-troubleshooting/#step-1-enabling-license-debugging))
2.  Your Order ID
3.  Your runtime key (or client/server key pair for OEM/Advanced Licensing)
4.  The version of SciChart.js you are using

<a href="https://github.com/ABTSoftware/SciChart.JS.Docs/tree/dev/docusaurus/docs/user-manual/licensing-scichart-js/licensing-troubleshooting/index.md" class="theme-edit-this-page" rel="noopener noreferrer" target="_blank"><img src="out_scichartv4/user-manual/licensing-scichart-js/licensing-troubleshooting/index_media/42a87493f9569d393738cbffa7bed03324e68916.svg" class="iconEdit_Z9Sw" />Edit this page</a>
