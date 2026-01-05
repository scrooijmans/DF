# Image Labels

In additional to all the label formatting options by SciChart.js,Â it is possible to go further and override theÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/labelproviderbase2d.html#getlabeltexture" rel="noopener noreferrer" target="_blank">LabelProvider.getLabelTexture()ðŸ“˜</a> functionÂ which converts the label text that is produced byÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/labelproviderbase2d.html#formatlabel" rel="noopener noreferrer" target="_blank">LabelProvider.formatLabel()ðŸ“˜</a> into a texture that can be drawn by WebGL.Â This gives you total control over the appearance of your labels so that you can use images, complex text, or a combination.

TheÂ code belowÂ is taken from our online <a href="https://www.scichart.com/demo/javascript-image-labels" rel="noopener noreferrer" target="_blank">Image Labels example</a>. The key part is to pass an HtmlImageElement toÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/texturemanager.html#createtexturefromimage" rel="noopener noreferrer" target="_blank">TextureManager.createTextureFromImage()ðŸ“˜</a>. Everything else here is about mapping the data to the images.

![](out_scichartv4/2d-charts/axis-api/axis-labels/image-labels/index_media/0f0874621de662630fd5d7f09c89455c4ace26e6.svg)warning

Setting `useNativeText: false` on axis is needed for images to appear.

- TS

``` prism-code
const { sciChartSurface, wasmContext } = await SciChartSurface.create(divElementId, {
    theme: new SciChartJsNavyTheme()
});

const xAxis = new NumericAxis(wasmContext, {
    // Ensure there can be 1 label per item in the dataset.
    maxAutoTicks: 15,
    axisTitle: "Mobile phone manufacturer",
    // We need the data value as plain text
    labelFormat: ENumericFormat.NoFormat,
    useNativeText: false // need to be set to "false" to show images
});

const images = [
    "https://www.scichart.com/demo/images/apple.png",
    "https://www.scichart.com/demo/images/samsung.png",
    "https://www.scichart.com/demo/images/xiaomi.png",
    "https://www.scichart.com/demo/images/huawei.png",
    "https://www.scichart.com/demo/images/oppo.png",
    "https://www.scichart.com/demo/images/vivo.png",
    "https://www.scichart.com/demo/images/realme.png",
    "https://www.scichart.com/demo/images/motorola.png",
    "https://www.scichart.com/demo/images/question.png",
    "https://www.scichart.com/demo/images/lg.png",
    "https://www.scichart.com/demo/images/oneplus.png",
    "https://www.scichart.com/demo/images/tecno.png",
    "https://www.scichart.com/demo/images/infinix.png",
    "https://www.scichart.com/demo/images/google.png",
    "https://www.scichart.com/demo/images/nokia.png"
];

// SciChart utility function to create HtmlImage elements from urls
const promises = images.map(image => createImageAsync(image));
const res = await Promise.allSettled(promises);

// Override labelProvider.getLabelTexture() to return an image
const getLabelTexture = (labelText, textureManager, labelStyle) => {
    const index = parseInt(labelText);
    if (!isNaN(index)) {
        if (res[index].status === "fulfilled") {
            const emoji = res[index].value;
            return textureManager.createTextureFromImage(emoji, 40, 40);
        } else {
            console.warn(`image ${images[index]} not found`);
        }
    }
    return textureManager.createTextTexture([labelText], labelStyle);
};
xAxis.labelProvider.getLabelTexture = getLabelTexture;

// If using asyncLabels = true, override this as well
xAxis.labelProvider.getLabelTextureAsync = (labelText, textureManager, labelStyle) =>
    Promise.resolve(getLabelTexture(labelText, textureManager, labelStyle));

// Disable shared cache for this provider, otherwise other axes might pick up the emoji textures
xAxis.labelProvider.useSharedCache = false;

sciChartSurface.xAxes.add(xAxis);
```

This results in the following output:

Textures created this way are automatically cached for performance, and disposed of (deleted) when the chart is deleted.

Normally, the size of the texture returned is used as the width and height for layout purposes.Â  Depending on the shape of your images, you may also want to override the **getLabelWidth** and **getLabelHeight** methods onÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/labelproviderbase2d.html" rel="noopener noreferrer" target="_blank">LabelProviderBase2DðŸ“˜</a>.

![](out_scichartv4/2d-charts/axis-api/axis-labels/image-labels/index_media/f0173787a207cc0a720ca619ab3e16cfe886b3c7.svg)tip

For an example of how to do this with TypeScript, React and npm / webpack to import images, see ourÂ <a href="https://www.scichart.com/demo/javascript-image-labels" rel="noopener noreferrer" target="_blank">Image Labels example</a>, part of the SciChart Demo.

<a href="https://github.com/ABTSoftware/SciChart.JS.Docs/tree/dev/docusaurus/docs/2d-charts/axis-api/axis-labels/image-labels/index.md" class="theme-edit-this-page" rel="noopener noreferrer" target="_blank"><img src="out_scichartv4/2d-charts/axis-api/axis-labels/image-labels/index_media/42a87493f9569d393738cbffa7bed03324e68916.svg" class="iconEdit_Z9Sw" />Edit this page</a>
