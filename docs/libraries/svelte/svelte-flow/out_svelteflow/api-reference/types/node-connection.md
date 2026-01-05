# NodeConnection

<a href="https://github.com/xyflow/xyflow/blob/main/packages/system/src/types/general.ts/#L36-L37" class="x:focus-visible:nextra-focus x:text-primary-600 x:underline x:hover:no-underline x:decoration-from-font x:[text-underline-position:from-font]" rel="noreferrer" target="_blank">Source on GitHub <img src="out_svelteflow/api-reference/types/node-connection/index_media/926b7c4e31213c3a8240847d8251d75a921b6a21.svg" class="x:inline x:align-baseline x:shrink-0" /></a>

The `NodeConnection` type is an extension of a basic <a href="https://svelteflow.dev/api-reference/types/connection" class="x:focus-visible:nextra-focus x:text-primary-600 x:underline x:hover:no-underline x:decoration-from-font x:[text-underline-position:from-font]">Connection</a> that includes the `edgeId`.

## Fields<a href="https://svelteflow.dev/api-reference/types/node-connection#fields" class="x:focus-visible:nextra-focus subheading-anchor" aria-label="Permalink for this section"></a>

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
<tr id="source" class="x:rounded-xl nextra-border x:hover:bg-primary-50 x:dark:hover:bg-primary-500/10 x:group x:mb-5 x:max-lg:block x:max-lg:border x:lg:border-b x:lg:not-target:[&amp;&gt;td&gt;a]:opacity-0">
<td class="x:relative x:max-lg:block x:py-3 x:max-lg:px-3"><a href="https://svelteflow.dev/api-reference/types/node-connection#source" class="x:absolute x:top-0 x:right-0 x:text-lg x:font-black x:before:content-[&quot;#&quot;] x:hover:text-black x:dark:hover:text-white x:px-3 x:py-[min(1%,12px)] x:group-hover:opacity-100! x:lg:top-1/2 x:lg:right-full x:lg:-translate-y-1/2"></a><code class="nextra-code x:max-md:break-all" dir="ltr">source</code></td>
<td class="x:p-3 x:max-lg:block x:max-lg:before:content-[&quot;Type:_&quot;]"><code class="nextra-code" dir="ltr">string</code>
<p>The id of the node this connection originates from.</p></td>
<td class="x:max-lg:block x:lg:after:content-[&quot;–&quot;]"></td>
</tr>
<tr id="target" class="x:rounded-xl nextra-border x:hover:bg-primary-50 x:dark:hover:bg-primary-500/10 x:group x:mb-5 x:max-lg:block x:max-lg:border x:lg:border-b x:lg:not-target:[&amp;&gt;td&gt;a]:opacity-0">
<td class="x:relative x:max-lg:block x:py-3 x:max-lg:px-3"><a href="https://svelteflow.dev/api-reference/types/node-connection#target" class="x:absolute x:top-0 x:right-0 x:text-lg x:font-black x:before:content-[&quot;#&quot;] x:hover:text-black x:dark:hover:text-white x:px-3 x:py-[min(1%,12px)] x:group-hover:opacity-100! x:lg:top-1/2 x:lg:right-full x:lg:-translate-y-1/2"></a><code class="nextra-code x:max-md:break-all" dir="ltr">target</code></td>
<td class="x:p-3 x:max-lg:block x:max-lg:before:content-[&quot;Type:_&quot;]"><code class="nextra-code" dir="ltr">string</code>
<p>The id of the node this connection terminates at.</p></td>
<td class="x:max-lg:block x:lg:after:content-[&quot;–&quot;]"></td>
</tr>
<tr id="sourcehandle" class="x:rounded-xl nextra-border x:hover:bg-primary-50 x:dark:hover:bg-primary-500/10 x:group x:mb-5 x:max-lg:block x:max-lg:border x:lg:border-b x:lg:not-target:[&amp;&gt;td&gt;a]:opacity-0">
<td class="x:relative x:max-lg:block x:py-3 x:max-lg:px-3"><a href="https://svelteflow.dev/api-reference/types/node-connection#sourcehandle" class="x:absolute x:top-0 x:right-0 x:text-lg x:font-black x:before:content-[&quot;#&quot;] x:hover:text-black x:dark:hover:text-white x:px-3 x:py-[min(1%,12px)] x:group-hover:opacity-100! x:lg:top-1/2 x:lg:right-full x:lg:-translate-y-1/2"></a><code class="nextra-code x:max-md:break-all" dir="ltr">sourceHandle</code></td>
<td class="x:p-3 x:max-lg:block x:max-lg:before:content-[&quot;Type:_&quot;]"><code class="nextra-code" dir="ltr">string | null</code>
<p>When not <code class="nextra-code" dir="ltr">null</code>, the id of the handle on the source node that this connection originates from.</p></td>
<td class="x:max-lg:block x:lg:after:content-[&quot;–&quot;]"></td>
</tr>
<tr id="targethandle" class="x:rounded-xl nextra-border x:hover:bg-primary-50 x:dark:hover:bg-primary-500/10 x:group x:mb-5 x:max-lg:block x:max-lg:border x:lg:border-b x:lg:not-target:[&amp;&gt;td&gt;a]:opacity-0">
<td class="x:relative x:max-lg:block x:py-3 x:max-lg:px-3"><a href="https://svelteflow.dev/api-reference/types/node-connection#targethandle" class="x:absolute x:top-0 x:right-0 x:text-lg x:font-black x:before:content-[&quot;#&quot;] x:hover:text-black x:dark:hover:text-white x:px-3 x:py-[min(1%,12px)] x:group-hover:opacity-100! x:lg:top-1/2 x:lg:right-full x:lg:-translate-y-1/2"></a><code class="nextra-code x:max-md:break-all" dir="ltr">targetHandle</code></td>
<td class="x:p-3 x:max-lg:block x:max-lg:before:content-[&quot;Type:_&quot;]"><code class="nextra-code" dir="ltr">string | null</code>
<p>When not <code class="nextra-code" dir="ltr">null</code>, the id of the handle on the target node that this connection terminates at.</p></td>
<td class="x:max-lg:block x:lg:after:content-[&quot;–&quot;]"></td>
</tr>
<tr id="edgeid" class="x:rounded-xl nextra-border x:hover:bg-primary-50 x:dark:hover:bg-primary-500/10 x:group x:mb-5 x:max-lg:block x:max-lg:border x:lg:border-b x:lg:not-target:[&amp;&gt;td&gt;a]:opacity-0">
<td class="x:relative x:max-lg:block x:py-3 x:max-lg:px-3"><a href="https://svelteflow.dev/api-reference/types/node-connection#edgeid" class="x:absolute x:top-0 x:right-0 x:text-lg x:font-black x:before:content-[&quot;#&quot;] x:hover:text-black x:dark:hover:text-white x:px-3 x:py-[min(1%,12px)] x:group-hover:opacity-100! x:lg:top-1/2 x:lg:right-full x:lg:-translate-y-1/2"></a><code class="nextra-code x:max-md:break-all" dir="ltr">edgeId</code></td>
<td class="x:p-3 x:max-lg:block x:max-lg:before:content-[&quot;Type:_&quot;]"><code class="nextra-code" dir="ltr">string</code></td>
<td class="x:max-lg:block x:lg:after:content-[&quot;–&quot;]"></td>
</tr>
</tbody>
</table>
