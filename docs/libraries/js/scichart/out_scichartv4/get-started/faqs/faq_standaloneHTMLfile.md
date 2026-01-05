On this page

# FAQ: Generating a Standalone SciChart HTML File

Sometimes you may want to share your **SciChart.js chart** with someone who doesnâ€™t want to install npm packages, run a dev server, or manage multiple files.  
Good news: you can package **all of SciChart.js (JavaScript + WebAssembly)** and your chart configuration into a **single HTML file** that runs offline in any modern browser ðŸš€.

This approach works by embedding the `.wasm` file as **Base64 text** instead of a binary file.  
The result is slightly larger (â‰ˆ3 MB vs â‰ˆ2.3 MB) but guarantees **only one portable file**.

------------------------------------------------------------------------

## When to Use This?<a href="https://www.scichart.com/documentation/js/v4/get-started/faqs/faq_standaloneHTMLfile/#when-to-use-this" class="hash-link" aria-label="Direct link to When to Use This?" translate="no" title="Direct link to When to Use This?">â€‹</a>

- âœ… Sharing a chart via email or file transfer
- âœ… Embedding SciChart inside offline docs or reports
- âœ… Creating demos for clients with no external dependencies
- â?Œ Not recommended for production deployment (prefer CDNs or proper bundling)

------------------------------------------------------------------------

## Step 1: Clone repo & Install dependencies<a href="https://www.scichart.com/documentation/js/v4/get-started/faqs/faq_standaloneHTMLfile/#step-1-clone-repo--install-dependencies" class="hash-link" aria-label="Direct link to Step 1: Clone repo &amp; Install dependencies" translate="no" title="Direct link to Step 1: Clone repo &amp; Install dependencies">â€‹</a>

``` prism-code
# Clone this repo:
git clone https://github.com/ABTSoftware/scichart.js-standalone_html_embeder

# and install dependencies:
npm install
```

------------------------------------------------------------------------

## Step 2: Configure your chart<a href="https://www.scichart.com/documentation/js/v4/get-started/faqs/faq_standaloneHTMLfile/#step-2-configure-your-chart" class="hash-link" aria-label="Direct link to Step 2: Configure your chart" translate="no" title="Direct link to Step 2: Configure your chart">â€‹</a>

The repo includes a **playground file** `drawExample.js` where you place your chart configuration.

We recommend starting in CodePen (e.g. <a href="https://codepen.io/vasculandrei/pen/gbOqXGE" rel="noopener noreferrer" target="_blank">Base Template</a>) to quickly iterate. Once youâ€™re happy, copy your chart setup code into `drawExample.js`.

------------------------------------------------------------------------

## Step 3: Build the offline HTML file<a href="https://www.scichart.com/documentation/js/v4/get-started/faqs/faq_standaloneHTMLfile/#step-3-build-the-offline-html-file" class="hash-link" aria-label="Direct link to Step 3: Build the offline HTML file" translate="no" title="Direct link to Step 3: Build the offline HTML file">â€‹</a>

Run the build script:

``` prism-code
npm run build
```

For **3D charts**, run:

``` prism-code
npm run build3d
```

------------------------------------------------------------------------

## Step 4: Done! ðŸŽ‰<a href="https://www.scichart.com/documentation/js/v4/get-started/faqs/faq_standaloneHTMLfile/#step-4-done-" class="hash-link" aria-label="Direct link to Step 4: Done! ð" translate="no" title="Direct link to Step 4: Done! ð">â€‹</a>

The script will generate:

``` prism-code
output.html
```

This single file includes:

- Your chart config (`drawExample.js`)
- SciChart library (`index.min.js`)
- WebAssembly runtime (Base64 encoded)

You can now open `output.html` in Chrome, Firefox, Edge, or Safari **without internet**. Itâ€™s completely self-contained and portable.

------------------------------------------------------------------------

![](out_scichartv4/get-started/faqs/faq_standaloneHTMLfile/index_media/5a72c3ee6a9cc14296f901057d6eabf2b9b712b4.svg)note

- **Copy-paste workflow:** Start in CodePen â†’ copy your code â†’ paste into `drawExample.js`.
- **Offline ready:** No npm, no web server, no extra files required.
- **3D support:** Works the same way, just run `npm run build3d`.
- **Customization:** You can style `output.html` like any normal webpage

<a href="https://github.com/ABTSoftware/SciChart.JS.Docs/tree/dev/docusaurus/docs/get-started/faqs/faq_standaloneHTMLfile/readme.md" class="theme-edit-this-page" rel="noopener noreferrer" target="_blank"><img src="out_scichartv4/get-started/faqs/faq_standaloneHTMLfile/index_media/42a87493f9569d393738cbffa7bed03324e68916.svg" class="iconEdit_Z9Sw" />Edit this page</a>
