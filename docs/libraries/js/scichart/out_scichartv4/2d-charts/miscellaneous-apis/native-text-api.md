On this page

# Native Text Api

The Native Text api is new in SciChart v3.Â  It uses industry standard font libraries compiled into webassembly to render text directly using webGL, supporting all types of fonts including right to left text, shaped fonts such as Arabic and Chinese.Â  This first version of the api is not complete and we welcome your feedback to shape it going forward.

## Font Loading<a href="https://www.scichart.com/documentation/js/v4/2d-charts/miscellaneous-apis/native-text-api/#font-loading" class="hash-link" aria-label="Direct link to Font Loading" translate="no" title="Direct link to Font Loading">â€‹</a>

Only Arial is included in the webassembly data as standard.Â  Other fonts must either be hosted on your server, or registered if coming from a remote location.Â  In either case, fonts are only downloaded once, and are then cached in the browser (in indexdb).

### Hosting fonts on your server<a href="https://www.scichart.com/documentation/js/v4/2d-charts/miscellaneous-apis/native-text-api/#hosting-fonts-on-your-server" class="hash-link" aria-label="Direct link to Hosting fonts on your server" translate="no" title="Direct link to Hosting fonts on your server">â€‹</a>

If you simply specify a font other than arialÂ for native axis labels, dataLabels or NativeTextAnnotation, SciChart will look for a file with that name .ttf on your server.Â  For example, the following annotation will cause SciChart toÂ request /jokerman.ttf

``` prism-code
// Hosted font

const nativeTextHostedFont = new NativeTextAnnotation({
        x1: 1,
        y1: 1,
        font: "jokerman",
        text: "This text uses a hosted font",
        fontSize: 18
     });
```

Â To serve fonts using webpack dev server you need a rule for .ttf files which specifies the correct mimetype, and you need to copy the font file itself to the root of the output location:

``` prism-code
// webpack.config.js

const path = require("path");
const CopyPlugin = require("copy-webpack-plugin");
const webpack = require("webpack");
module.exports = {
    mode: "development",
    entry: "./src/index.js",
    module: {
        rules: [
            // Serve .ttf files
            {
                test: /\.ttf$/,
                use: {
                    loader: "url-loader",
                    options: { mimetype: "application/font-ttf" }
                }
            }
        ]
    },
    resolve: {
        extensions: [".js"]
    },
    output: {
        filename: "bundle.js",
        path: path.resolve(__dirname, "build")
    },
    plugins: [
        new CopyPlugin({
            patterns: [
                { from: "src/index.html", to: "" },
                // Copy the font to the root of the output location
                { from: "src/jokerman.ttf", to: "" },
                { from: "node_modules/scichart/_wasm/scichart2d.data", to: "" },
                { from: "node_modules/scichart/_wasm/scichart2d.wasm", to: "" }
            ]
        }),
    ]
};
```

### Registering Remote Fonts<a href="https://www.scichart.com/documentation/js/v4/2d-charts/miscellaneous-apis/native-text-api/#registering-remote-fonts" class="hash-link" aria-label="Direct link to Registering Remote Fonts" translate="no" title="Direct link to Registering Remote Fonts">â€‹</a>

UseÂ `sciChartSurface.registerFont` to provide a remote url to load a font file from.Â  Note that this requires a sciChartSurface instance - it is not a static method.Â  The method returns a promise which resolves once the file is downloaded.Â  If you do not await this method, the text will render using Arial until the font is available.Â  There is a timeout (set by SciChartDefaults.nativeFontTimeout, default 2000ms) after which SciChart will fall back to Arial and stop trying to load the custom font.Â  You might need to increase this if you need to load fonts over a slow connection, but in general it is better to await the registerFont method.

![](out_scichartv4/2d-charts/miscellaneous-apis/native-text-api/index_media/0f0874621de662630fd5d7f09c89455c4ace26e6.svg)warning

There is currently a limitation in that the font fetching from webassembly will not follow a http 302 redirection, so you need to pass the url to the actual file.Â  For instance, when downloading from github,Â <a href="https://github.com/google/fonts/blob/main/ofl/braahone/BraahOne-Regular.ttf" rel="noopener noreferrer" target="_blank">https://github.com/google/fonts/blob/main/ofl/braahone/BraahOne-Regular.ttf</a>Â redirects toÂ <a href="https://raw.githubusercontent.com/google/fonts/main/ofl/braahone/BraahOne-Regular.ttf" rel="noopener noreferrer" target="_blank">https://raw.githubusercontent.com/google/fonts/main/ofl/braahone/BraahOne-Regular.ttf</a> so you need to use the githubusercontent.com link.

``` prism-code
// Registering a font

await sciChartSurface.registerFont(
      "braahone",
      "https://raw.githubusercontent.com/google/fonts/main/ofl/braahone/BraahOne-Regular.ttf"
);
const nativeTextRemote = new NativeTextAnnotation({
        x1: 3,
        y1: 7,
        text: "This text uses a font from the internet",
        fontFamily: "braahone",
        fontSize: 24
});
```

## Native Text API<a href="https://www.scichart.com/documentation/js/v4/2d-charts/miscellaneous-apis/native-text-api/#native-text-api-1" class="hash-link" aria-label="Direct link to Native Text API" translate="no" title="Direct link to Native Text API">â€‹</a>

The following sections describe some of the native text api methods and concepts which you may need if you want to develop custom annotations, dataLabels or series using native text.Â  In summary, using the native text api goes like this:

1.  Call renderContext.getFont to get a font instance.Â  Fonts are cached and shared within webassembly, so there is no need to cache them in JS.
2.  If necessary call getTextBounds and pass it to font.CalculateStringBounds to get information on the size of your text so you can adjust drawing coordinates.
3.  Call font.DrawString, or font.DrawStringAdvanced

### getFont<a href="https://www.scichart.com/documentation/js/v4/2d-charts/miscellaneous-apis/native-text-api/#getfont" class="hash-link" aria-label="Direct link to getFont" translate="no" title="Direct link to getFont">â€‹</a>

getFont is a method on webGLRenderContext2D which is passed to the drawing methods (eg toÂ RenderContextAnnotationBase.drawWithContext) as renderContext. If you plan to use rotation or scaling,Â set theÂ advanced parameter true.Â  Requesting an advanced font actually meansÂ SciChart will generate a Signed Distance Field font which gives much better rendering for rotated and scaled text, and in the futureÂ will allow for more advanced text effects.Â  However, if you don't need this, normal fonts use less memory and are slightly faster to first frame.Â 

There is no need to call font.Begin() - this is done by getFont.Â  Set the drawEarly parameterÂ trueÂ if you are planning to call font.End() early so other elementsÂ can draw over the text.Â  This is not stricly required, but it causes SciChart to give you a separate font instance so you don't mess with other text that might be drawing with the same font.

Do not call nativeContext.AquireFont directly.Â  There is no need to delete the font to free memory.

### TextBounds<a href="https://www.scichart.com/documentation/js/v4/2d-charts/miscellaneous-apis/native-text-api/#textbounds" class="hash-link" aria-label="Direct link to TextBounds" translate="no" title="Direct link to TextBounds">â€‹</a>

Call getTextBounds from scichart/Charting/Visuals/Helpers/NativeObject, to get a TSRTextBounds instance.Â  Each call to this method returns the same cached instance.Â  Do not call delete on it.

CallÂ font.CalculateStringBounds which populates the TSRTextBounds with the size of your desired text.Â  The image below shows how the properties on textBounds relate to the text.Â  Text is anchored at the left on the baseline.Â  The origin is top, left (for consistency with canvas coordinates) so to have the text anchored at the top, you need to addÂ Â textBounds.GetLineBounds(0).m_fHeight to your y coordinate.

<img src="out_scichartv4/2d-charts/miscellaneous-apis/native-text-api/index_media/fcf6cc1a9379c9b0f3a4829f75c1f3c1595f1d4a.png" style="display:block;margin-left:auto;margin-right:auto;object-fit:contain;height:auto;width:85%;margin:0 auto" />

For multi line text, m_fHeight is the height of the entire text block, but text is still anchored at the baseline of the first line.

### Drawing Text<a href="https://www.scichart.com/documentation/js/v4/2d-charts/miscellaneous-apis/native-text-api/#drawing-text" class="hash-link" aria-label="Direct link to Drawing Text" translate="no" title="Direct link to Drawing Text">â€‹</a>

Call font.DrawString, or font.DrawStringAdvanced.Â  DrawString is just text, colour, x, y whereas DrawStringAdvanced also allows youÂ to specify rotation, multiline alignment and spacing.The only difference is the options available.Â  You do not have to have created the font with advanced: true to use DrawStringAdvanced if you are just doing multiline, but for rotated text you will get much nicer rendering with advanced: true.

Note that text is not actually drawn immediately.Â  This happens when font.End() is called.Â 

SciChart automatically calls font.End on all fonts at the end of the render cycle.Â  If you want the native text to draw earlier so other chart elements can draw over it, you can call font.End yourself, but for optimum performance you want to do this as little as possible.

### Rotation<a href="https://www.scichart.com/documentation/js/v4/2d-charts/miscellaneous-apis/native-text-api/#rotation" class="hash-link" aria-label="Direct link to Rotation" translate="no" title="Direct link to Rotation">â€‹</a>

To get a rotation vector use the following code:

``` prism-code
// Rotation vector

import { getVector4 } from "scichart"

const rotationVector = getVector4(
    webAssemblyContext2D,
    rotationCenterX,
    rotationCenterY,
    rotationInRadians,
    0
);
```

Like textBounds this returns a single shared instance so you do not need to delete it.

### Scaling<a href="https://www.scichart.com/documentation/js/v4/2d-charts/miscellaneous-apis/native-text-api/#scaling" class="hash-link" aria-label="Direct link to Scaling" translate="no" title="Direct link to Scaling">â€‹</a>

You can adjust the size of text by calling font.SetScale, which will multiply the font size by the valueÂ you set.Â  This only applies to subsequentÂ DrawString/DrawStringAdvanced calls.

#### See Also<a href="https://www.scichart.com/documentation/js/v4/2d-charts/miscellaneous-apis/native-text-api/#see-also" class="hash-link" aria-label="Direct link to See Also" translate="no" title="Direct link to See Also">â€‹</a>

##### Axis Labels<a href="https://www.scichart.com/documentation/js/v4/2d-charts/miscellaneous-apis/native-text-api/#axis-labels" class="hash-link" aria-label="Direct link to Axis Labels" translate="no" title="Direct link to Axis Labels">â€‹</a>

[Performance Considerations - Native Text Axis Labels](https://www.scichart.com/documentation/js/v4/2d-charts/axis-api/axis-labels/performance-considerations-native-text-axis-abels/)

##### Annotations API<a href="https://www.scichart.com/documentation/js/v4/2d-charts/miscellaneous-apis/native-text-api/#annotations-api" class="hash-link" aria-label="Direct link to Annotations API" translate="no" title="Direct link to Annotations API">â€‹</a>

[NativeTextAnnotation](https://www.scichart.com/documentation/js/v4/2d-charts/annotations-api/native-text-annotation/)

##### DataLabels API<a href="https://www.scichart.com/documentation/js/v4/2d-charts/miscellaneous-apis/native-text-api/#datalabels-api" class="hash-link" aria-label="Direct link to DataLabels API" translate="no" title="Direct link to DataLabels API">â€‹</a>

[Adding DataLabels to a Chart Series](https://www.scichart.com/documentation/js/v4/2d-charts/chart-types/data-point-labels/data-labels-api-overview/)

<a href="https://github.com/ABTSoftware/SciChart.JS.Docs/tree/dev/docusaurus/docs/2d-charts/miscellaneous-apis/native-text-api/index.md" class="theme-edit-this-page" rel="noopener noreferrer" target="_blank"><img src="out_scichartv4/2d-charts/miscellaneous-apis/native-text-api/index_media/42a87493f9569d393738cbffa7bed03324e68916.svg" class="iconEdit_Z9Sw" />Edit this page</a>
