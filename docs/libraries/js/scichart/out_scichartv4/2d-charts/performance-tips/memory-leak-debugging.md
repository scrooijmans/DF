On this page

# Memory Leak Debugging

In SciChart.js v3.2 we've introduced a set of Memory Leak debugging tools. Read this guide to find out how to enable Memory Leak Debugging in your app in Dev mode.Â 

![](out_scichartv4/2d-charts/performance-tips/memory-leak-debugging/index_media/af50301a53fa555616a9b2279e7e25a1d566cb1a.svg)info

Read the section inÂ [Memory Best Practices - Deletable Entities](https://www.scichart.com/documentation/js/v4/2d-charts/performance-tips/memory-best-practices/#deletable-entities-in-scichartjs) for background info on which types need to be explictly deleted in SciChart.js and how failure to do this can cause a memory leak.

## Enabling Memory Debugging tools<a href="https://www.scichart.com/documentation/js/v4/2d-charts/performance-tips/memory-leak-debugging/#enabling-memory-debugging-tools" class="hash-link" aria-label="Direct link to Enabling Memory Debugging tools" translate="no" title="Direct link to Enabling Memory Debugging tools">â€‹</a>

Enabling memory leak debugging is possible by setting the static propertyÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/memoryusagehelper.html#ismemoryusagedebugenabled" rel="noopener noreferrer" target="_blank">MemoryUsageHelper.isMemoryUsageDebugEnabledðŸ“˜</a> = true.

``` prism-code
import { MemoryUsageHelper } from "scichart";

MemoryUsageHelper.isMemoryUsageDebugEnabled = true;
```

After enabling memory usage helper you should see a message in the console output like this:

<img src="out_scichartv4/2d-charts/performance-tips/memory-leak-debugging/index_media/b72c10841e556190887860e07ef2875226639868.png" style="display:block;margin-left:auto;margin-right:auto;object-fit:contain;height:auto;width:85%;margin:0 auto" />

![](out_scichartv4/2d-charts/performance-tips/memory-leak-debugging/index_media/af50301a53fa555616a9b2279e7e25a1d566cb1a.svg)info

Note this will only work in development mode: the environment variable process.env.NODE_ENV must not equal "prod" or "production"

## Tracking Undeleted objects<a href="https://www.scichart.com/documentation/js/v4/2d-charts/performance-tips/memory-leak-debugging/#tracking-undeleted-objects" class="hash-link" aria-label="Direct link to Tracking Undeleted objects" translate="no" title="Direct link to Tracking Undeleted objects">â€‹</a>

OnceÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/memoryusagehelper.html" rel="noopener noreferrer" target="_blank">MemoryUsageHelperðŸ“˜</a> is enabled, the memory usage debugging tool has two features:

- It adds helpful warnings to the JS Console when executing actions which may result in a potential memory leak or undesirable behaviour.
- It wraps objects implementingÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/interfaces/ideletable.html" rel="noopener noreferrer" target="_blank">IDeletableðŸ“˜</a> interface and native entites created in WebAssembly into a proxy - and will keep records of them in anÂ <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/objectregistry.html" rel="noopener noreferrer" target="_blank">ObjectRegistryðŸ“˜</a> instance -Â <a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/memoryusagehelper.html#objectregistry" rel="noopener noreferrer" target="_blank">MemoryUsageHelper.objectRegistryðŸ“˜</a>.

### Examining the ObjectRegistry State<a href="https://www.scichart.com/documentation/js/v4/2d-charts/performance-tips/memory-leak-debugging/#examining-the-objectregistry-state" class="hash-link" aria-label="Direct link to Examining the ObjectRegistry State" translate="no" title="Direct link to Examining the ObjectRegistry State">â€‹</a>

The registry state can be examined at any time by calling

``` prism-code
import { MemoryUsageHelper } from "scichart";

// Log out current objectRegistry state
MemoryUsageHelper.objectRegistry.log();
```

This will output several collections and their contents to the JS console. From the output we can define a lifecycle state of an object and it's type.

<img src="out_scichartv4/2d-charts/performance-tips/memory-leak-debugging/index_media/3c1d1cd06ddd01a6e56bddcd3774cce766ffc9d7.png" style="display:block;margin-left:auto;margin-right:auto;object-fit:contain;height:auto;width:85%;margin:0 auto" />

- **undeletedObjectsMap** - objects that were created but .delete() was not called.
- **uncollectedObjectsMap** = objects that are not yet garbage collected by the JavaScript Garbage Collector
- **collectedNotDeleted** - objects that have been garbage collected but .delete() was omitted
- **deletedNotCollected** - objects that had .delete() called by were not collected by the JavaScript Garbage Collector
- **weakMap** - another collection that could be used to see if an object was GC'd. Additionally tracks referenes to proxies of deleted objects.

## What to do if you find a Leak<a href="https://www.scichart.com/documentation/js/v4/2d-charts/performance-tips/memory-leak-debugging/#what-to-do-if-you-find-a-leak" class="hash-link" aria-label="Direct link to What to do if you find a Leak" translate="no" title="Direct link to What to do if you find a Leak">â€‹</a>

- **deletedNotCollected**: your application code called **.delete()** on an object but there is still a reference to it somewhere in JavaScriptÂ preventing it from being GC'd.Â Use the Chrome memory tools to identify the GC roots to determine the cause of the leak (<a href="https://developer.chrome.com/docs/devtools/memory-problems/" rel="noopener noreferrer" target="_blank">find out how</a>).
- **collectedNotDeleted**: your application code GC'd the object but you forgot to call **.delete()**. This can cause a WebAssembly memory leak. Ensure this object has **.delete()** called before it falls out of scope.

## Tracking Arbitrary objects<a href="https://www.scichart.com/documentation/js/v4/2d-charts/performance-tips/memory-leak-debugging/#tracking-arbitrary-objects" class="hash-link" aria-label="Direct link to Tracking Arbitrary objects" translate="no" title="Direct link to Tracking Arbitrary objects">â€‹</a>

**<a href="https://www.scichart.com/documentation/js/v4/typedoc/classes/memoryusagehelper.html" rel="noopener noreferrer" target="_blank">MemoryUsageHelperðŸ“˜</a>** automatically tracks all objects created within SciChart itself. If you want to use our tools to track abitrary objects in your application. you can use the following functions:

``` prism-code
// Register an arbitrary object
MemoryUsageHelper.register(yourObject, "identifier");

// Unregister an arbitrary object
MemoryUsageHelper.unregister("identifier");
```

#### See Also<a href="https://www.scichart.com/documentation/js/v4/2d-charts/performance-tips/memory-leak-debugging/#see-also" class="hash-link" aria-label="Direct link to See Also" translate="no" title="Direct link to See Also">â€‹</a>

- [Performance Tips & Tricks](https://www.scichart.com/documentation/js/v4/2d-charts/performance-tips/performance-tips-and-tricks/)
- [Memory Best Practices](https://www.scichart.com/documentation/js/v4/2d-charts/performance-tips/memory-best-practices/)

<a href="https://github.com/ABTSoftware/SciChart.JS.Docs/tree/dev/docusaurus/docs/2d-charts/performance-tips/memory-leak-debugging/index.md" class="theme-edit-this-page" rel="noopener noreferrer" target="_blank"><img src="out_scichartv4/2d-charts/performance-tips/memory-leak-debugging/index_media/42a87493f9569d393738cbffa7bed03324e68916.svg" class="iconEdit_Z9Sw" />Edit this page</a>
