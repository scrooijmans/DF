On this page

# Deploying SciChart.js to Domains

![](out_scichartv4/user-manual/licensing-scichart-js/deploying-to-live-sites/index_media/af50301a53fa555616a9b2279e7e25a1d566cb1a.svg)info

To allow website (known domain) deployment of SciChart.js applications without a watermark and to comply with the <a href="https://www.scichart.com/scichart-eula" rel="noopener noreferrer" target="_blank">terms for commercial use</a>, you will need to setup a runtime license key with valid domains for your app.

Products purchased from our store that enable SciChart.js website domain licensing include:

- SciChart JS 2D
- SciChart JS 2D & 3D
- SciChart Bundle 2D Pro
- SciChart Bundle 2D/3D Pro
- SciChart Bundle 2D/3D Source

For purchasing, please visit <a href="https://www.scichart.com/shop" rel="noopener noreferrer" target="_blank">scichart.com/shop</a>. For pre-sales enquiries <a href="https://www.scichart.com/contact-us#pre-sales" rel="noopener noreferrer" target="_blank">contact technical sales</a> and for licensing support post-sales please <a href="https://www.scichart.com/contact-us#tech-support" rel="noopener noreferrer" target="_blank">contact technical support</a>.

![](out_scichartv4/user-manual/licensing-scichart-js/deploying-to-live-sites/index_media/f0173787a207cc0a720ca619ab3e16cfe886b3c7.svg)tip

If you are deploying to embedded devices (ie the site will run on `localhost`) or you are an OEM and the application will be deployed by your customers to domains that you do not control, then we have alternative licensing mechanisms.

Please see [deploying with advanced licensing](https://www.scichart.com/documentation/js/v4/user-manual/licensing-scichart-js/deploying-with-advanced-licensing/) for more info.

When you have a paid SciChart.js developer license, to deploy an application to a domain you need to register that domain with your account, generate and insert the runtime key into your app. Please find the instructions below.

## Adding / removing production domains to your scichart.js license key<a href="https://www.scichart.com/documentation/js/v4/user-manual/licensing-scichart-js/deploying-to-live-sites/#adding--removing-production-domains-to-your-scichartjs-license-key" class="hash-link" aria-label="Direct link to Adding / removing production domains to your scichart.js license key" translate="no" title="Direct link to Adding / removing production domains to your scichart.js license key">â€‹</a>

1.  Head over to <a href="https://www.scichart.com/my-account/" rel="noopener noreferrer" target="_blank">scichart.com/my-account</a> to adminster your license keys (Need help? See [location and management of license keys](https://www.scichart.com/documentation/js/v4/user-manual/licensing-scichart-js/where-are-my-license-keys/))
2.  In the section **Orders & Keys** - **Manage Licenses** - **Hostnames** you can add/remove hostnames for your license
3.  Enter the domain then select from the dropdown "Production" to add a production domain, or "Test" to add a test domain
4.  Click "Submit"

<img src="out_scichartv4/user-manual/licensing-scichart-js/deploying-to-live-sites/index_media/32c0bf73cb3780c1c73b153df946cdd0791685bc.png" class="img_ev3q" decoding="async" loading="lazy" width="1363" height="1199" alt="Adding Production Domains to your license key - scichart.js" />

5.  Check the hostname was added

<img src="out_scichartv4/user-manual/licensing-scichart-js/deploying-to-live-sites/index_media/b19102c7fd887cc389a23a5aba1045935cdf6b6b.png" class="img_ev3q" decoding="async" loading="lazy" width="805" height="818" alt="Check domain hostname was added to your license key - scichart.js" />

6.  Once the license key has been added, now click "Runtime License Key" to regenerate the runtime key

<img src="out_scichartv4/user-manual/licensing-scichart-js/deploying-to-live-sites/index_media/c9e71b22e87d1884c736ecb2705da5669bae23fd.png" class="img_ev3q" decoding="async" loading="lazy" width="805" height="944" alt="Generate a runtime key for your domain - scichart.js" />

## Deploying a runtime Key in your app<a href="https://www.scichart.com/documentation/js/v4/user-manual/licensing-scichart-js/deploying-to-live-sites/#deploying-a-runtime-key-in-your-app" class="hash-link" aria-label="Direct link to Deploying a runtime Key in your app" translate="no" title="Direct link to Deploying a runtime Key in your app">â€‹</a>

Once you have generated a runtime key following the steps above, add the runtime key to your application by copying the line of code:

``` prism-code
SciChartSurface.setRuntimeLicenseKey("--your-runtime-key-here--");
```

![](out_scichartv4/user-manual/licensing-scichart-js/deploying-to-live-sites/index_media/f0173787a207cc0a720ca619ab3e16cfe886b3c7.svg)tip

The Runtime Key must be placed in your application once before any `SciChartSurface` is shown or instantiated, for example in a Root component in a React App.

![](out_scichartv4/user-manual/licensing-scichart-js/deploying-to-live-sites/index_media/af50301a53fa555616a9b2279e7e25a1d566cb1a.svg)info

Once a runtime key is deployed correctly, it will work in perpetuity (does not require an internet connection and won't ever fallback to community / trial version).

When deployed to a domain, SciChart.js checks the local runtime key set by calling `sciChartSurface.setRuntimeLicenseKey("--Your-Key--")`. No licensing wizard or developer license is needed on deployed servers or machines.

![](out_scichartv4/user-manual/licensing-scichart-js/deploying-to-live-sites/index_media/0f0874621de662630fd5d7f09c89455c4ace26e6.svg)warning

If you update SciChart.js in your app and redeploy, ensure the version of SciChart.js was released before your support expiry date (`ExpiryDate` in [License Debug](https://www.scichart.com/documentation/js/v4/user-manual/licensing-scichart-js/licensing-troubleshooting/#step-1-enabling-license-debugging)).

If you renew a SciChart.js subscription, regenerate runtime keys when you next apply a SciChart.js library version update.

Otherwise, the deployed application may default to community license

## Testing & debugging of runtime license keys<a href="https://www.scichart.com/documentation/js/v4/user-manual/licensing-scichart-js/deploying-to-live-sites/#testing--debugging-of-runtime-license-keys" class="hash-link" aria-label="Direct link to Testing &amp; debugging of runtime license keys" translate="no" title="Direct link to Testing &amp; debugging of runtime license keys">â€‹</a>

To test and debug your runtime key, it is strongly recommended to follow the steps to [enable license debug](https://www.scichart.com/documentation/js/v4/user-manual/licensing-scichart-js/licensing-troubleshooting/#step-1-enabling-license-debugging) and view the console debug output before deploying your app.

If the domain name is found correctly, go ahead and deploy your app to your domain.

## FAQs on Domain Licensing Deployment<a href="https://www.scichart.com/documentation/js/v4/user-manual/licensing-scichart-js/deploying-to-live-sites/#faqs-on-domain-licensing-deployment" class="hash-link" aria-label="Direct link to FAQs on Domain Licensing Deployment" translate="no" title="Direct link to FAQs on Domain Licensing Deployment">â€‹</a>

### How can I remove a domain from my license key?<a href="https://www.scichart.com/documentation/js/v4/user-manual/licensing-scichart-js/deploying-to-live-sites/#how-can-i-remove-a-domain-from-my-license-key" class="hash-link" aria-label="Direct link to How can I remove a domain from my license key?" translate="no" title="Direct link to How can I remove a domain from my license key?">â€‹</a>

Follow the steps above to add/remove domains from your license key, regenerate the runtime key and deploy.

### I see a trial watermark in my test domain?<a href="https://www.scichart.com/documentation/js/v4/user-manual/licensing-scichart-js/deploying-to-live-sites/#i-see-a-trial-watermark-in-my-test-domain" class="hash-link" aria-label="Direct link to I see a trial watermark in my test domain?" translate="no" title="Direct link to I see a trial watermark in my test domain?">â€‹</a>

Test domains added to the runtime key display a watermark by design.

### I see a powered by SciChart watermark in my live deployed application<a href="https://www.scichart.com/documentation/js/v4/user-manual/licensing-scichart-js/deploying-to-live-sites/#i-see-a-powered-by-scichart-watermark-in-my-live-deployed-application" class="hash-link" aria-label="Direct link to I see a powered by SciChart watermark in my live deployed application" translate="no" title="Direct link to I see a powered by SciChart watermark in my live deployed application">â€‹</a>

Follow the troubleshooting steps from [here](https://www.scichart.com/documentation/js/v4/user-manual/licensing-scichart-js/licensing-troubleshooting/#i-see-a-trial--community-watermark-in-my-live-deployed-app)

### How many domains can I register with my license key?<a href="https://www.scichart.com/documentation/js/v4/user-manual/licensing-scichart-js/deploying-to-live-sites/#how-many-domains-can-i-register-with-my-license-key" class="hash-link" aria-label="Direct link to How many domains can I register with my license key?" translate="no" title="Direct link to How many domains can I register with my license key?">â€‹</a>

This depends on the license type, but SciChart.js licenses have at least 5 production domains per license. If you hit a limit or need help, <a href="https://www.scichart.com/contact-us#tech-support" rel="noopener noreferrer" target="_blank">contact tech support</a>

<a href="https://github.com/ABTSoftware/SciChart.JS.Docs/tree/dev/docusaurus/docs/user-manual/licensing-scichart-js/deploying-to-live-sites/index.md" class="theme-edit-this-page" rel="noopener noreferrer" target="_blank"><img src="out_scichartv4/user-manual/licensing-scichart-js/deploying-to-live-sites/index_media/42a87493f9569d393738cbffa7bed03324e68916.svg" class="iconEdit_Z9Sw" />Edit this page</a>
