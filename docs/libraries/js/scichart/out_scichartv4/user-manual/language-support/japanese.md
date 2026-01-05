On this page

# SciChartã?¯æ—¥æœ¬èªžã‚’ã‚µãƒ?ãƒ¼ãƒˆã?—ã?¦ã?„ã?¾ã?™ã?‹ï¼Ÿ

ã?¯ã?„ã€?SciChartã?¯æ—¥æœ¬èªžã‚’å®Œå…¨ã?«ã‚µãƒ?ãƒ¼ãƒˆã?—ã?¦ã?„ã?¾ã?™ã€‚SciChartãƒ©ã‚¤ãƒ–ãƒ©ãƒªã?¯Unicodeã‚¨ãƒ³ã‚³ãƒ¼ãƒ‡ã‚£ãƒ³ã‚°ã‚’ä½¿ç”¨ã?—ã?¦ã?„ã‚‹ã?Ÿã‚?ã€?æ—¥æœ¬èªžã?®æ–‡å­—ï¼ˆã?²ã‚‰ã?Œã?ªã€?ã‚«ã‚¿ã‚«ãƒŠã€?æ¼¢å­—ï¼‰ã‚’å?«ã‚€ã?™ã?¹ã?¦ã?®Unicodeæ–‡å­—ã‚»ãƒƒãƒˆã‚’è¡¨ç¤ºã?§ã??ã?¾ã?™ã€‚

## Unicodeæ–‡å­—ã‚µãƒ?ãƒ¼ãƒˆ<a href="#unicodeæå­ãµãã¼ã" class="hash-link" aria-label="Direct link to Unicodeæå­ãµãã¼ã" translate="no" title="Direct link to Unicodeæå­ãµãã¼ã">â€‹</a>

SciChartã?¯ä»¥ä¸‹ã?®ç?†ç”±ã?§æ—¥æœ¬èªžæ–‡å­—ã‚’é?©åˆ‡ã?«å‡¦ç?†ã?§ã??ã?¾ã?™ï¼š

- å®Œå…¨ã?ªUnicodeã‚µãƒ?ãƒ¼ãƒˆ: UTF-8ã?Šã‚ˆã?³UTF-16ã‚¨ãƒ³ã‚³ãƒ¼ãƒ‡ã‚£ãƒ³ã‚°ã?«å¯¾å¿œ

- å›½éš›åŒ–å¯¾å¿œ: å¤šè¨€èªžãƒ†ã‚­ã‚¹ãƒˆãƒ¬ãƒ³ãƒ€ãƒªãƒ³ã‚°ã‚¨ãƒ³ã‚¸ãƒ³å†…è”µ

- ãƒ•ã‚©ãƒ³ãƒˆäº’æ?›æ€§: ã‚·ã‚¹ãƒ†ãƒ ãƒ•ã‚©ãƒ³ãƒˆã?Šã‚ˆã?³Webãƒ•ã‚©ãƒ³ãƒˆã?¨ã?®å®Œå…¨ã?ªäº’æ?›æ€§

## 2Dãƒ?ãƒ£ãƒ¼ãƒˆã?§ã?®æ—¥æœ¬èªžå®Ÿè£…<a href="#2dãã£ã¼ãã§ã®æ¥æ¬èªå®è£" class="hash-link" aria-label="Direct link to 2Dãã£ã¼ãã§ã®æ¥æ¬èªå®è£" translate="no" title="Direct link to 2Dãã£ã¼ãã§ã®æ¥æ¬èªå®è£">â€‹</a>

åŸºæœ¬çš„ã?ªæ—¥æœ¬èªžãƒ†ã‚­ã‚¹ãƒˆã?®è¨­å®š

``` prism-code

SciChartDefaults.useNativeText = false;

// 2Dãã£ã¼ãã§æ¥æ¬èªã¿ã¤ãã«ã¨ã©ãã«ãè¨­å®
const { wasmContext, sciChartSurface } = await SciChartSurface.create(divElementId, {
    theme: new SciChartJsNavyTheme(),
    title: "å£²ä¸ã°ã©ã", // æ¥æ¬èªã¿ã¤ãã«
    titleStyle: {
        fontSize: 30
    }
});

// Xè»¸ã®æ¥æ¬èªã©ãã«
const xAxis = new NumericAxis(wasmContext, {
    axisTitle: "æ", // æ¥æ¬èªè»¸ã¿ã¤ãã«
    growBy: new NumberRange(0.02, 0.02)
});

// Yè»¸ã®æ¥æ¬èªã©ãã«
const yAxis = new NumericAxis(wasmContext, {
    axisTitle: "å£²ä¸ï¼ä¸åï¼", // æ¥æ¬èªè»¸ã¿ã¤ãã«
    growBy: new NumberRange(0.01, 0.1)
});

sciChartSurface.xAxes.add(xAxis);
sciChartSurface.yAxes.add(yAxis);

const dataSeries = new XyDataSeries(wasmContext, {
    dataSeriesName: "æ¥æ¬èªã·ãªã¼ãºå"
});

dataSeries.appendRange([0, 1, 2, 3, 4, 5], [20, 22, 25, 28, 30, 27]);

const lineSeries = new FastLineRenderableSeries(wasmContext, {
    dataSeries,
    stroke: "#FF6600"
});

sciChartSurface.renderableSeries.add(lineSeries);

const textAnnotation = new TextAnnotation({
    text: "éè¦ãªãã¤ã³ãï¼å£²ä¸ãæ¥ä¸æ", // æ¥æ¬èªæ³¨é
    x1: 4,
    y1: 30,
    fontSize: 14,
    textColor: "#FF0000"
});

sciChartSurface.annotations.add(textAnnotation);
```

## 3Dãƒ?ãƒ£ãƒ¼ãƒˆã?§ã?®æ—¥æœ¬èªžå®Ÿè£…<a href="#3dãã£ã¼ãã§ã®æ¥æ¬èªå®è£" class="hash-link" aria-label="Direct link to 3Dãã£ã¼ãã§ã®æ¥æ¬èªå®è£" translate="no" title="Direct link to 3Dãã£ã¼ãã§ã®æ¥æ¬èªå®è£">â€‹</a>

Scichart 3dã?§æ—¥æœ¬èªžæ–‡å­—ã‚’ä½¿ç”¨ã?™ã‚‹ã?«ã?¯ã€?ã?“ã?®è¨˜äº‹ã‚’å?‚ç…§ã?—ã?¦ã??ã? ã?•ã?„ã€‚ - <a href="https://www.scichart.com/documentation/js/v4/2d-charts/miscellaneous-apis/native-text-api/" rel="noopener noreferrer" target="_blank">Native Text ApiðŸ“˜</a>

## é‡?è¦?ã?ªæ³¨æ„?äº‹é …<a href="#éè¦ãªæ³¨æäºé%C2%A0" class="hash-link" aria-label="Direct link to éè¦ãªæ³¨æäºé " translate="no" title="Direct link to éè¦ãªæ³¨æäºé ">â€‹</a>

- ã‚¨ãƒ³ã‚³ãƒ¼ãƒ‡ã‚£ãƒ³ã‚°: HTMLãƒ•ã‚¡ã‚¤ãƒ«ã?¨JavaScriptãƒ•ã‚¡ã‚¤ãƒ«ã?¯UTF-8ã‚¨ãƒ³ã‚³ãƒ¼ãƒ‡ã‚£ãƒ³ã‚°ã?§ä¿?å­˜ã?—ã?¦ã??ã? ã?•ã?„

- ãƒ–ãƒ©ã‚¦ã‚¶äº’æ?›æ€§: ã?™ã?¹ã?¦ã?®ä¸»è¦?ãƒ–ãƒ©ã‚¦ã‚¶ã?§æ—¥æœ¬èªžè¡¨ç¤ºã?Œã‚µãƒ?ãƒ¼ãƒˆã?•ã‚Œã?¦ã?„ã?¾ã?™

SciChartã?®Unicodeã‚µãƒ?ãƒ¼ãƒˆã?«ã‚ˆã‚Šã€?æ—¥æœ¬èªžã‚’å?«ã‚€å¤šè¨€èªžãƒ?ãƒ£ãƒ¼ãƒˆã‚¢ãƒ—ãƒªã‚±ãƒ¼ã‚·ãƒ§ãƒ³ã?®é–‹ç™ºã?Œå?¯èƒ½ã?§ã?™ã€‚

<a href="https://github.com/ABTSoftware/SciChart.JS.Docs/tree/dev/docusaurus/docs/user-manual/language-support/japanese/index.md" class="theme-edit-this-page" rel="noopener noreferrer" target="_blank"><img src="out_scichartv4/user-manual/language-support/japanese/index_media/42a87493f9569d393738cbffa7bed03324e68916.svg" class="iconEdit_Z9Sw" />Edit this page</a>
