On this page

# SciChart æ˜¯å?¦æ”¯æŒ?ä¸­æ–‡

SciChart JS å®Œå…¨æ”¯æŒ?ä¸­æ–‡å­—ç¬¦æ˜¾ç¤ºï¼Œå› ä¸ºå®ƒåŸºäºŽ Unicode å­—ç¬¦ç¼–ç ?ç³»ç»Ÿæž„å»ºã€‚ä½œä¸ºä¸€ä¸ªçŽ°ä»£çš„ JavaScript å›¾è¡¨åº“ï¼ŒSciChart èƒ½å¤Ÿæ­£ç¡®æ¸²æŸ“å’Œæ˜¾ç¤ºå?„ç§?è¯­è¨€çš„å­—ç¬¦ï¼ŒåŒ…æ‹¬ä¸­æ–‡ç®€ä½“ã€?ç¹?ä½“ä»¥å?Šå…¶ä»– Unicode å­—ç¬¦é›†ã€‚

## Unicode æ”¯æŒ?åŽŸç?†<a href="#unicode-æ¯æåç" class="hash-link" aria-label="Direct link to Unicode æ¯æåç" translate="no" title="Direct link to Unicode æ¯æåç">â€‹</a>

SciChart JS åˆ©ç”¨æµ?è§ˆå™¨çš„åŽŸç”Ÿ Unicode æ”¯æŒ?æ?¥å¤„ç?†å¤šè¯­è¨€æ–‡æœ¬æ¸²æŸ“ã€‚ç”±äºŽçŽ°ä»£æµ?è§ˆå™¨éƒ½é?µå¾ª Unicode æ ‡å‡†ï¼ŒSciChart å?¯ä»¥æ— ç¼?åœ°æ˜¾ç¤ºä¸­æ–‡å­—ç¬¦ï¼Œæ— éœ€é¢?å¤–çš„é…?ç½®æˆ–æ?’ä»¶ã€‚

## 2D å›¾è¡¨ä¸­çš„ä¸­æ–‡å­—ç¬¦å®žçŽ°<a href="#2d-å¾è¡¨ä¸­çä¸­æå­ç¬¦å®ç°" class="hash-link" aria-label="Direct link to 2D å¾è¡¨ä¸­çä¸­æå­ç¬¦å®ç°" translate="no" title="Direct link to 2D å¾è¡¨ä¸­çä¸­æå­ç¬¦å®ç°">â€‹</a>

ä»¥ä¸‹ä»£ç ?ç¤ºä¾‹å±•ç¤ºäº†å¦‚ä½•åœ¨ SciChart 2D å›¾è¡¨ä¸­ä½¿ç”¨ä¸­æ–‡æ ‡ç­¾å’Œæ–‡æœ¬ï¼š

``` prism-code

SciChartDefaults.useNativeText = false;

// åå»º SciChart è¡¨é¢
const { wasmContext, sciChartSurface } = await SciChartSurface.create(divElementId, {
    theme: new SciChartJsNavyTheme(),
    title: "æ¯æ¥æ¸©åº¦ååè¶å¿å¾",
    titleStyle: {
        fontSize: 30,
    },
});

// éç½®å¸¦ä¸­ææ ç­¾ç X è½´
const xAxis = new NumericAxis(wasmContext, {
    axisTitle: "æ¶é´ï¼å°æ¶ï¼",
    growBy: new NumberRange(0.02, 0.02),
});

// éç½®å¸¦ä¸­ææ ç­¾ç Y è½´
const yAxis = new NumericAxis(wasmContext, {
    axisTitle: "æ¸©åº¦ï¼ææ°åº¦ï¼",
    growBy: new NumberRange(0.01, 0.1),
});

// æ·»å è½´å°å¾è¡¨
sciChartSurface.xAxes.add(xAxis);
sciChartSurface.yAxes.add(yAxis);

// åå»ºæ°æ®ç³»å
const dataSeries = new XyDataSeries(wasmContext, {
    dataSeriesName: "åäº¬æ¸©åº¦æ°æ®"
});

// æ·»å æ°æ®ç¹
dataSeries.appendRange([0, 1, 2, 3, 4, 5], [20, 22, 25, 28, 30, 27]);

// åå»ºçº¿ç³»å
const lineSeries = new FastLineRenderableSeries(wasmContext, {
    dataSeries,
    stroke: "#FF6600"
});

// æ·»å ç³»åå°å¾è¡¨
sciChartSurface.renderableSeries.add(lineSeries);


```

## æ–‡æœ¬æ³¨é‡Šå’Œæ ‡ç­¾<a href="#ææ¬æ³¨éåæ%C2%A0ç­¾" class="hash-link" aria-label="Direct link to ææ¬æ³¨éåæ ç­¾" translate="no" title="Direct link to ææ¬æ³¨éåæ ç­¾">â€‹</a>

SciChart è¿˜æ”¯æŒ?åœ¨å›¾è¡¨ä¸Šæ·»åŠ ä¸­æ–‡æ–‡æœ¬æ³¨é‡Šï¼š

``` prism-code

// åå»ºä¸­æææ¬æ³¨é
const textAnnotation = new TextAnnotation({
    text: "æé«æ¸©åº¦ç¹",
    x1: 4,
    y1: 30,
    fontSize: 14,
    textColor: "#FF0000"
});

// æ·»å æ³¨éå°å¾è¡¨
sciChartSurface.annotations.add(textAnnotation);
```

## 3D å›¾è¡¨ä¸­çš„ä¸­æ–‡å­—ç¬¦å®žçŽ°<a href="#3d-å¾è¡¨ä¸­çä¸­æå­ç¬¦å®ç°" class="hash-link" aria-label="Direct link to 3D å¾è¡¨ä¸­çä¸­æå­ç¬¦å®ç°" translate="no" title="Direct link to 3D å¾è¡¨ä¸­çä¸­æå­ç¬¦å®ç°">â€‹</a>

è¦?åœ¨ Scichart 3d ä¸­ä½¿ç”¨ä¸­æ–‡å­—ç¬¦ï¼Œè¯·å?‚é˜…æœ¬æ–‡ - <a href="https://www.scichart.com/documentation/js/v4/2d-charts/miscellaneous-apis/native-text-api/" rel="noopener noreferrer" target="_blank">Native Text ApiðŸ“˜</a>

## æœ€ä½³å®žè·µå»ºè®®<a href="#æä½³å®è·µå»ºè®®" class="hash-link" aria-label="Direct link to æä½³å®è·µå»ºè®®" translate="no" title="Direct link to æä½³å®è·µå»ºè®®">â€‹</a>

- å­—ä½“é€‰æ‹©ï¼šä½¿ç”¨ç³»ç»Ÿé»˜è®¤çš„ä¸­æ–‡å­—ä½“ï¼Œå¦‚ Microsoft YaHeiï¼ˆå¾®è½¯é›…é»‘ï¼‰æˆ– SimHeiï¼ˆé»‘ä½“ï¼‰

- ç¼–ç ?è®¾ç½®ï¼šç¡®ä¿? HTML é¡µé?¢ä½¿ç”¨ UTF-8 ç¼–ç ?

- å­—ä½“å¤§å°?ï¼šæ ¹æ?®å›¾è¡¨å¤§å°?è°ƒæ•´å?ˆé€‚çš„å­—ä½“å¤§å°?ï¼Œç¡®ä¿?ä¸­æ–‡å­—ç¬¦æ¸…æ™°å?¯è¯»

- æµ?è§ˆå™¨å…¼å®¹æ€§ï¼šæµ‹è¯•ä¸?å?Œæµ?è§ˆå™¨ä¸­çš„ä¸­æ–‡æ˜¾ç¤ºæ•ˆæžœ

é€šè¿‡ä»¥ä¸Šé…?ç½®å’Œä»£ç ?ç¤ºä¾‹ï¼Œæ‚¨å?¯ä»¥åœ¨ SciChart JS åº”ç”¨ç¨‹åº?ä¸­å®Œç¾Žåœ°æ˜¾ç¤ºå’Œä½¿ç”¨ä¸­æ–‡å­—ç¬¦ï¼Œåˆ›å»ºæœ¬åœ°åŒ–çš„å›¾è¡¨ç•Œé?¢ã€‚

<a href="https://github.com/ABTSoftware/SciChart.JS.Docs/tree/dev/docusaurus/docs/user-manual/language-support/chinese/index.md" class="theme-edit-this-page" rel="noopener noreferrer" target="_blank"><img src="out_scichartv4/user-manual/language-support/chinese/index_media/42a87493f9569d393738cbffa7bed03324e68916.svg" class="iconEdit_Z9Sw" />Edit this page</a>
