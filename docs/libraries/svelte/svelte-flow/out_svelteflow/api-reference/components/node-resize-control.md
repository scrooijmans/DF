# \<NodeResizeControl /\>

<a href="https://github.com/xyflow/xyflow/blob/487b13c9ad8903789f56c6fcfd8222f9cb74b812/packages/svelte/src/lib/plugins/NodeResizer/ResizeControl.svelte" class="x:focus-visible:nextra-focus x:text-primary-600 x:underline x:hover:no-underline x:decoration-from-font x:[text-underline-position:from-font]" rel="noreferrer" target="_blank">Source on GitHub <img src="out_svelteflow/api-reference/components/node-resize-control/index_media/926b7c4e31213c3a8240847d8251d75a921b6a21.svg" class="x:inline x:align-baseline x:shrink-0" /></a>

To create your own resizing UI, you can use the `NodeResizeControl` component where you can pass children (such as icons).

## Props<a href="https://svelteflow.dev/api-reference/components/node-resize-control#props" class="x:focus-visible:nextra-focus subheading-anchor" aria-label="Permalink for this section"></a>

The type for props of `<NodeResizeControl />` component is exported as `NodeResizeControlProps`. Additionally, it extends the props of `<div />`.

<table class="x:my-8 x:w-full x:text-sm">
<colgroup>
<col style="width: 33%" />
<col style="width: 33%" />
<col style="width: 33%" />
</colgroup>
<thead class="nextra-border x:border-b x:text-left x:max-lg:hidden">
<tr>
<th class="x:py-1.5">Name</th>
<th class="x:p-1.5 x:px-3">Type</th>
<th class="x:py-1.5">Default</th>
</tr>
</thead>
<tbody>
<tr id="nodeid" class="x:rounded-xl nextra-border x:hover:bg-primary-50 x:dark:hover:bg-primary-500/10 x:group x:mb-5 x:max-lg:block x:max-lg:border x:lg:border-b x:lg:not-target:[&amp;&gt;td&gt;a]:opacity-0">
<td class="x:relative x:max-lg:block x:py-3 x:max-lg:px-3"><a href="https://svelteflow.dev/api-reference/components/node-resize-control#nodeid" class="x:absolute x:top-0 x:right-0 x:text-lg x:font-black x:before:content-[&quot;#&quot;] x:hover:text-black x:dark:hover:text-white x:px-3 x:py-[min(1%,12px)] x:group-hover:opacity-100! x:lg:top-1/2 x:lg:right-full x:lg:-translate-y-1/2"></a><code class="nextra-code x:max-md:break-all x:after:content-[&quot;?&quot;]" dir="ltr">nodeId</code></td>
<td class="x:p-3 x:max-lg:block x:max-lg:before:content-[&quot;Type:_&quot;]"><code class="nextra-code" dir="ltr">string</code>
<p>Id of the node it is resizing</p></td>
<td class="x:max-lg:block x:lg:after:content-[&quot;–&quot;]"></td>
</tr>
<tr id="minwidth" class="x:rounded-xl nextra-border x:hover:bg-primary-50 x:dark:hover:bg-primary-500/10 x:group x:mb-5 x:max-lg:block x:max-lg:border x:lg:border-b x:lg:not-target:[&amp;&gt;td&gt;a]:opacity-0">
<td class="x:relative x:max-lg:block x:py-3 x:max-lg:px-3"><a href="https://svelteflow.dev/api-reference/components/node-resize-control#minwidth" class="x:absolute x:top-0 x:right-0 x:text-lg x:font-black x:before:content-[&quot;#&quot;] x:hover:text-black x:dark:hover:text-white x:px-3 x:py-[min(1%,12px)] x:group-hover:opacity-100! x:lg:top-1/2 x:lg:right-full x:lg:-translate-y-1/2"></a><code class="nextra-code x:max-md:break-all x:after:content-[&quot;?&quot;]" dir="ltr">minWidth</code></td>
<td class="x:p-3 x:max-lg:block x:max-lg:before:content-[&quot;Type:_&quot;]"><code class="nextra-code" dir="ltr">number</code>
<p>Minimum width of node</p></td>
<td class="x:max-lg:block x:lg:after:content-[&quot;–&quot;]"></td>
</tr>
<tr id="minheight" class="x:rounded-xl nextra-border x:hover:bg-primary-50 x:dark:hover:bg-primary-500/10 x:group x:mb-5 x:max-lg:block x:max-lg:border x:lg:border-b x:lg:not-target:[&amp;&gt;td&gt;a]:opacity-0">
<td class="x:relative x:max-lg:block x:py-3 x:max-lg:px-3"><a href="https://svelteflow.dev/api-reference/components/node-resize-control#minheight" class="x:absolute x:top-0 x:right-0 x:text-lg x:font-black x:before:content-[&quot;#&quot;] x:hover:text-black x:dark:hover:text-white x:px-3 x:py-[min(1%,12px)] x:group-hover:opacity-100! x:lg:top-1/2 x:lg:right-full x:lg:-translate-y-1/2"></a><code class="nextra-code x:max-md:break-all x:after:content-[&quot;?&quot;]" dir="ltr">minHeight</code></td>
<td class="x:p-3 x:max-lg:block x:max-lg:before:content-[&quot;Type:_&quot;]"><code class="nextra-code" dir="ltr">number</code>
<p>Minimum height of node</p></td>
<td class="x:max-lg:block x:lg:after:content-[&quot;–&quot;]"></td>
</tr>
<tr id="maxwidth" class="x:rounded-xl nextra-border x:hover:bg-primary-50 x:dark:hover:bg-primary-500/10 x:group x:mb-5 x:max-lg:block x:max-lg:border x:lg:border-b x:lg:not-target:[&amp;&gt;td&gt;a]:opacity-0">
<td class="x:relative x:max-lg:block x:py-3 x:max-lg:px-3"><a href="https://svelteflow.dev/api-reference/components/node-resize-control#maxwidth" class="x:absolute x:top-0 x:right-0 x:text-lg x:font-black x:before:content-[&quot;#&quot;] x:hover:text-black x:dark:hover:text-white x:px-3 x:py-[min(1%,12px)] x:group-hover:opacity-100! x:lg:top-1/2 x:lg:right-full x:lg:-translate-y-1/2"></a><code class="nextra-code x:max-md:break-all x:after:content-[&quot;?&quot;]" dir="ltr">maxWidth</code></td>
<td class="x:p-3 x:max-lg:block x:max-lg:before:content-[&quot;Type:_&quot;]"><code class="nextra-code" dir="ltr">number</code>
<p>Maximum width of node</p></td>
<td class="x:max-lg:block x:lg:after:content-[&quot;–&quot;]"></td>
</tr>
<tr id="maxheight" class="x:rounded-xl nextra-border x:hover:bg-primary-50 x:dark:hover:bg-primary-500/10 x:group x:mb-5 x:max-lg:block x:max-lg:border x:lg:border-b x:lg:not-target:[&amp;&gt;td&gt;a]:opacity-0">
<td class="x:relative x:max-lg:block x:py-3 x:max-lg:px-3"><a href="https://svelteflow.dev/api-reference/components/node-resize-control#maxheight" class="x:absolute x:top-0 x:right-0 x:text-lg x:font-black x:before:content-[&quot;#&quot;] x:hover:text-black x:dark:hover:text-white x:px-3 x:py-[min(1%,12px)] x:group-hover:opacity-100! x:lg:top-1/2 x:lg:right-full x:lg:-translate-y-1/2"></a><code class="nextra-code x:max-md:break-all x:after:content-[&quot;?&quot;]" dir="ltr">maxHeight</code></td>
<td class="x:p-3 x:max-lg:block x:max-lg:before:content-[&quot;Type:_&quot;]"><code class="nextra-code" dir="ltr">number</code>
<p>Maximum height of node</p></td>
<td class="x:max-lg:block x:lg:after:content-[&quot;–&quot;]"></td>
</tr>
<tr id="keepaspectratio" class="x:rounded-xl nextra-border x:hover:bg-primary-50 x:dark:hover:bg-primary-500/10 x:group x:mb-5 x:max-lg:block x:max-lg:border x:lg:border-b x:lg:not-target:[&amp;&gt;td&gt;a]:opacity-0">
<td class="x:relative x:max-lg:block x:py-3 x:max-lg:px-3"><a href="https://svelteflow.dev/api-reference/components/node-resize-control#keepaspectratio" class="x:absolute x:top-0 x:right-0 x:text-lg x:font-black x:before:content-[&quot;#&quot;] x:hover:text-black x:dark:hover:text-white x:px-3 x:py-[min(1%,12px)] x:group-hover:opacity-100! x:lg:top-1/2 x:lg:right-full x:lg:-translate-y-1/2"></a><code class="nextra-code x:max-md:break-all x:after:content-[&quot;?&quot;]" dir="ltr">keepAspectRatio</code></td>
<td class="x:p-3 x:max-lg:block x:max-lg:before:content-[&quot;Type:_&quot;]"><code class="nextra-code" dir="ltr">boolean</code>
<p>Keep aspect ratio when resizing</p></td>
<td class="x:max-lg:block x:lg:after:content-[&quot;–&quot;]"></td>
</tr>
<tr id="autoscale" class="x:rounded-xl nextra-border x:hover:bg-primary-50 x:dark:hover:bg-primary-500/10 x:group x:mb-5 x:max-lg:block x:max-lg:border x:lg:border-b x:lg:not-target:[&amp;&gt;td&gt;a]:opacity-0">
<td class="x:relative x:max-lg:block x:py-3 x:max-lg:px-3"><a href="https://svelteflow.dev/api-reference/components/node-resize-control#autoscale" class="x:absolute x:top-0 x:right-0 x:text-lg x:font-black x:before:content-[&quot;#&quot;] x:hover:text-black x:dark:hover:text-white x:px-3 x:py-[min(1%,12px)] x:group-hover:opacity-100! x:lg:top-1/2 x:lg:right-full x:lg:-translate-y-1/2"></a><code class="nextra-code x:max-md:break-all x:after:content-[&quot;?&quot;]" dir="ltr">autoScale</code></td>
<td class="x:p-3 x:max-lg:block x:max-lg:before:content-[&quot;Type:_&quot;]"><code class="nextra-code" dir="ltr">boolean</code>
<p>Automatically scale the node when resizing</p></td>
<td class="x:max-lg:block x:lg:after:content-[&quot;–&quot;]"></td>
</tr>
<tr id="shouldresize" class="x:rounded-xl nextra-border x:hover:bg-primary-50 x:dark:hover:bg-primary-500/10 x:group x:mb-5 x:max-lg:block x:max-lg:border x:lg:border-b x:lg:not-target:[&amp;&gt;td&gt;a]:opacity-0">
<td class="x:relative x:max-lg:block x:py-3 x:max-lg:px-3"><a href="https://svelteflow.dev/api-reference/components/node-resize-control#shouldresize" class="x:absolute x:top-0 x:right-0 x:text-lg x:font-black x:before:content-[&quot;#&quot;] x:hover:text-black x:dark:hover:text-white x:px-3 x:py-[min(1%,12px)] x:group-hover:opacity-100! x:lg:top-1/2 x:lg:right-full x:lg:-translate-y-1/2"></a><code class="nextra-code x:max-md:break-all x:after:content-[&quot;?&quot;]" dir="ltr">shouldResize</code></td>
<td class="x:p-3 x:max-lg:block x:max-lg:before:content-[&quot;Type:_&quot;]"><code class="nextra-code" dir="ltr">(event: ResizeDragEvent, params: ResizeParamsWithDirection) =&gt; boolean</code>
<p>Callback to determine if node should resize</p></td>
<td class="x:max-lg:block x:lg:after:content-[&quot;–&quot;]"></td>
</tr>
<tr id="onresizestart" class="x:rounded-xl nextra-border x:hover:bg-primary-50 x:dark:hover:bg-primary-500/10 x:group x:mb-5 x:max-lg:block x:max-lg:border x:lg:border-b x:lg:not-target:[&amp;&gt;td&gt;a]:opacity-0">
<td class="x:relative x:max-lg:block x:py-3 x:max-lg:px-3"><a href="https://svelteflow.dev/api-reference/components/node-resize-control#onresizestart" class="x:absolute x:top-0 x:right-0 x:text-lg x:font-black x:before:content-[&quot;#&quot;] x:hover:text-black x:dark:hover:text-white x:px-3 x:py-[min(1%,12px)] x:group-hover:opacity-100! x:lg:top-1/2 x:lg:right-full x:lg:-translate-y-1/2"></a><code class="nextra-code x:max-md:break-all x:after:content-[&quot;?&quot;]" dir="ltr">onResizeStart</code></td>
<td class="x:p-3 x:max-lg:block x:max-lg:before:content-[&quot;Type:_&quot;]"><code class="nextra-code" dir="ltr">OnResizeStart</code>
<p>Callback called when resizing starts</p></td>
<td class="x:max-lg:block x:lg:after:content-[&quot;–&quot;]"></td>
</tr>
<tr id="onresize" class="x:rounded-xl nextra-border x:hover:bg-primary-50 x:dark:hover:bg-primary-500/10 x:group x:mb-5 x:max-lg:block x:max-lg:border x:lg:border-b x:lg:not-target:[&amp;&gt;td&gt;a]:opacity-0">
<td class="x:relative x:max-lg:block x:py-3 x:max-lg:px-3"><a href="https://svelteflow.dev/api-reference/components/node-resize-control#onresize" class="x:absolute x:top-0 x:right-0 x:text-lg x:font-black x:before:content-[&quot;#&quot;] x:hover:text-black x:dark:hover:text-white x:px-3 x:py-[min(1%,12px)] x:group-hover:opacity-100! x:lg:top-1/2 x:lg:right-full x:lg:-translate-y-1/2"></a><code class="nextra-code x:max-md:break-all x:after:content-[&quot;?&quot;]" dir="ltr">onResize</code></td>
<td class="x:p-3 x:max-lg:block x:max-lg:before:content-[&quot;Type:_&quot;]"><code class="nextra-code" dir="ltr">OnResize</code>
<p>Callback called when resizing</p></td>
<td class="x:max-lg:block x:lg:after:content-[&quot;–&quot;]"></td>
</tr>
<tr id="onresizeend" class="x:rounded-xl nextra-border x:hover:bg-primary-50 x:dark:hover:bg-primary-500/10 x:group x:mb-5 x:max-lg:block x:max-lg:border x:lg:border-b x:lg:not-target:[&amp;&gt;td&gt;a]:opacity-0">
<td class="x:relative x:max-lg:block x:py-3 x:max-lg:px-3"><a href="https://svelteflow.dev/api-reference/components/node-resize-control#onresizeend" class="x:absolute x:top-0 x:right-0 x:text-lg x:font-black x:before:content-[&quot;#&quot;] x:hover:text-black x:dark:hover:text-white x:px-3 x:py-[min(1%,12px)] x:group-hover:opacity-100! x:lg:top-1/2 x:lg:right-full x:lg:-translate-y-1/2"></a><code class="nextra-code x:max-md:break-all x:after:content-[&quot;?&quot;]" dir="ltr">onResizeEnd</code></td>
<td class="x:p-3 x:max-lg:block x:max-lg:before:content-[&quot;Type:_&quot;]"><code class="nextra-code" dir="ltr">OnResizeEnd</code>
<p>Callback called when resizing ends</p></td>
<td class="x:max-lg:block x:lg:after:content-[&quot;–&quot;]"></td>
</tr>
<tr id="resizedirection" class="x:rounded-xl nextra-border x:hover:bg-primary-50 x:dark:hover:bg-primary-500/10 x:group x:mb-5 x:max-lg:block x:max-lg:border x:lg:border-b x:lg:not-target:[&amp;&gt;td&gt;a]:opacity-0">
<td class="x:relative x:max-lg:block x:py-3 x:max-lg:px-3"><a href="https://svelteflow.dev/api-reference/components/node-resize-control#resizedirection" class="x:absolute x:top-0 x:right-0 x:text-lg x:font-black x:before:content-[&quot;#&quot;] x:hover:text-black x:dark:hover:text-white x:px-3 x:py-[min(1%,12px)] x:group-hover:opacity-100! x:lg:top-1/2 x:lg:right-full x:lg:-translate-y-1/2"></a><code class="nextra-code x:max-md:break-all x:after:content-[&quot;?&quot;]" dir="ltr">resizeDirection</code></td>
<td class="x:p-3 x:max-lg:block x:max-lg:before:content-[&quot;Type:_&quot;]"><code class="nextra-code" dir="ltr">'horizontal' | 'vertical'</code>
<p>The direction the user can resize the node</p></td>
<td class="x:max-lg:block x:lg:after:content-[&quot;–&quot;]"></td>
</tr>
<tr id="position" class="x:rounded-xl nextra-border x:hover:bg-primary-50 x:dark:hover:bg-primary-500/10 x:group x:mb-5 x:max-lg:block x:max-lg:border x:lg:border-b x:lg:not-target:[&amp;&gt;td&gt;a]:opacity-0">
<td class="x:relative x:max-lg:block x:py-3 x:max-lg:px-3"><a href="https://svelteflow.dev/api-reference/components/node-resize-control#position" class="x:absolute x:top-0 x:right-0 x:text-lg x:font-black x:before:content-[&quot;#&quot;] x:hover:text-black x:dark:hover:text-white x:px-3 x:py-[min(1%,12px)] x:group-hover:opacity-100! x:lg:top-1/2 x:lg:right-full x:lg:-translate-y-1/2"></a><code class="nextra-code x:max-md:break-all x:after:content-[&quot;?&quot;]" dir="ltr">position</code></td>
<td class="x:p-3 x:max-lg:block x:max-lg:before:content-[&quot;Type:_&quot;]"><code class="nextra-code" dir="ltr">ControlLinePosition | 'top-left' | 'top-right' | 'bottom-left' | 'bottom-right'</code>
<p>Position of control</p></td>
<td class="x:max-lg:block x:lg:after:content-[&quot;–&quot;]"></td>
</tr>
<tr id="variant" class="x:rounded-xl nextra-border x:hover:bg-primary-50 x:dark:hover:bg-primary-500/10 x:group x:mb-5 x:max-lg:block x:max-lg:border x:lg:border-b x:lg:not-target:[&amp;&gt;td&gt;a]:opacity-0">
<td class="x:relative x:max-lg:block x:py-3 x:max-lg:px-3"><a href="https://svelteflow.dev/api-reference/components/node-resize-control#variant" class="x:absolute x:top-0 x:right-0 x:text-lg x:font-black x:before:content-[&quot;#&quot;] x:hover:text-black x:dark:hover:text-white x:px-3 x:py-[min(1%,12px)] x:group-hover:opacity-100! x:lg:top-1/2 x:lg:right-full x:lg:-translate-y-1/2"></a><code class="nextra-code x:max-md:break-all x:after:content-[&quot;?&quot;]" dir="ltr">variant</code></td>
<td class="x:p-3 x:max-lg:block x:max-lg:before:content-[&quot;Type:_&quot;]"><code class="nextra-code" dir="ltr">ResizeControlVariant</code>
<p>Variant of control</p></td>
<td class="x:max-lg:block x:lg:after:content-[&quot;–&quot;]"></td>
</tr>
<tr id="props" class="x:rounded-xl nextra-border x:hover:bg-primary-50 x:dark:hover:bg-primary-500/10 x:group x:mb-5 x:max-lg:block x:max-lg:border x:lg:border-b x:lg:not-target:[&amp;&gt;td&gt;a]:opacity-0">
<td class="x:relative x:max-lg:block x:py-3 x:max-lg:px-3"><a href="https://svelteflow.dev/api-reference/components/node-resize-control#props" class="x:absolute x:top-0 x:right-0 x:text-lg x:font-black x:before:content-[&quot;#&quot;] x:hover:text-black x:dark:hover:text-white x:px-3 x:py-[min(1%,12px)] x:group-hover:opacity-100! x:lg:top-1/2 x:lg:right-full x:lg:-translate-y-1/2"></a><code class="nextra-code x:max-md:break-all" dir="ltr">...props</code></td>
<td class="x:p-3 x:max-lg:block x:max-lg:before:content-[&quot;Type:_&quot;]"><code class="nextra-code" dir="ltr">HTMLAttributes&lt;HTMLDivElement&gt;</code></td>
<td class="x:max-lg:block x:lg:after:content-[&quot;–&quot;]"></td>
</tr>
</tbody>
</table>
