On this page

# Setting a Runtime License on a SciChartSurface

Another static method which allows you to license SciChart (apply a trial or paid production or test license) is SciChartSurface.setRuntimeLicenseKey()

All of our instructions for licensing can be found at the pageÂ <a href="https://www.scichart.com/licensing-scichart-js" rel="noopener noreferrer" target="_blank">scichart.com/licensing-scichart-js</a>. A quick code sample is below. Ensure that you call this function once before any SciChartSurface is shown with a valid runtime key.

``` prism-code
import { SciChartSurface } from "scichart";

// Set a runtime key in JavaScript once before any SciChartSurface is created
SciChartSurface.setRuntimeLicenseKey("YOUR_RUNTIME_KEY_HERE");
```

### Notes on Licensing<a href="https://www.scichart.com/documentation/js/v4/2d-charts/surface/runtime-license/#notes-on-licensing" class="hash-link" aria-label="Direct link to Notes on Licensing" translate="no" title="Direct link to Notes on Licensing">â€‹</a>

1.  **SciChart licensing is two-step.** We have a developer license for localhost and a runtime key for production or staging sites.
2.  The Runtime Key controls how your app works on a website (with encoded domain in the key). This applies to production and staging (test) sites.
3.  Staging (test) sites will have a watermark. This is expected & by design. Production sites will not have a watermark.
4.  Development activity carried out on your local PC will require an activated developer license.

![](out_scichartv4/2d-charts/surface/runtime-license/index_media/f0173787a207cc0a720ca619ab3e16cfe886b3c7.svg)tip

Full instructions how to activate developer licenses, how to add production & test domains to your account and how to include Runtime keysÂ can be found atÂ <a href="https://www.scichart.com/licensing-scichart-js" rel="noopener noreferrer" target="_blank">scichart.com/licensing-scichart-js</a>.

### Resolving Wasm errors on load<a href="https://www.scichart.com/documentation/js/v4/2d-charts/surface/runtime-license/#resolving-wasm-errors-on-load" class="hash-link" aria-label="Direct link to Resolving Wasm errors on load" translate="no" title="Direct link to Resolving Wasm errors on load">â€‹</a>

If you get an error when loading a SciChartSurface as follows:

![](out_scichartv4/2d-charts/surface/runtime-license/index_media/0f0874621de662630fd5d7f09c89455c4ace26e6.svg)warning

**Error**: Could not load SciChart WebAssembly module. Check your build process and ensure that your scichart2d.wasm and scichart2d.js files are from the same version

Please see our related articleÂ [Deploying Wasm or WebAssembly Data Files with your app](https://www.scichart.com/documentation/js/v4/2d-charts/surface/deploying-wasm)

#### See Also<a href="https://www.scichart.com/documentation/js/v4/2d-charts/surface/runtime-license/#see-also" class="hash-link" aria-label="Direct link to See Also" translate="no" title="Direct link to See Also">â€‹</a>

- [The SciChartSurface Type](https://www.scichart.com/documentation/js/v4/2d-charts/surface/scichart-surface-type-overview)
- [Creating a new SciChartSurface and loading Wasm](https://www.scichart.com/documentation/js/v4/2d-charts/surface/new-scichart-surface)
- [Deploying Wasm (WebAssembly) and Data Files with your app](https://www.scichart.com/documentation/js/v4/2d-charts/surface/deploying-wasm)

<a href="https://github.com/ABTSoftware/SciChart.JS.Docs/tree/dev/docusaurus/docs/2d-charts/surface/runtime-license/index.md" class="theme-edit-this-page" rel="noopener noreferrer" target="_blank"><img src="out_scichartv4/2d-charts/surface/runtime-license/index_media/42a87493f9569d393738cbffa7bed03324e68916.svg" class="iconEdit_Z9Sw" />Edit this page</a>
