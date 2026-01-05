![](out_runed/utilities/finite-state-machine/index_media/412c1b99df431530f07813ed77ff123e20b0035f.svg) Toggle Sidebar

<img src="out_runed/utilities/finite-state-machine/index_media/2ef71b57cc18afcd4ed82644358ede113ed6d592.svg" class="size-5" />Search Docs ... ⌘ K

<a href="https://github.com/svecosystem/runed" class="inline-flex select-none items-center justify-center gap-2 whitespace-nowrap text-sm font-semibold tracking-wide active:scale-[99%] disabled:pointer-events-none disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:size-4 [&amp;_svg]:shrink-0 text-foreground hover:dark:bg-foreground/5 hover:dark:ring-foreground/5 hover:bg-foreground/2 hover:ring-foreground/2 bg-transparent ring ring-transparent h-10 w-10 rounded-md [&amp;&gt;svg]:size-5" aria-label="View this project on GitHub" rel="noopener noreferrer" target="_blank"><img src="out_runed/utilities/finite-state-machine/index_media/1d22f722c87a133f2381965da1acf7a54aefc45a.svg" /></a>

<img src="out_runed/utilities/finite-state-machine/index_media/b2f1f2409c2d7a3eaec75643beb67d5092b71fb2.svg" class="size-6" />

# FiniteStateMachine

Defines a strongly-typed finite state machine.

## <a href="https://runed.dev/docs/utilities/finite-state-machine#demo" class="text-brand-link hover:text-brand-link-hover leading-7" aria-hidden="true" tabindex="-1"></a>Demo

# Now: `disabled`

Toggle the switch to enable.

![](out_runed/utilities/finite-state-machine/index_media/6ff2106ab77bb4fa5ddad76193ac26e165bb072f.svg) Run

<figure data-rehype-pretty-code-figure="">
<pre class="ring" data-language="ts" data-theme="github-light github-dark" tabindex="0"><code>
        import { FiniteStateMachine } from &quot;runed&quot;;
type MyStates = &quot;disabled&quot; | &quot;idle&quot; | &quot;running&quot;;
type MyEvents = &quot;toggleEnabled&quot; | &quot;start&quot; | &quot;stop&quot;;
 
const f = new FiniteStateMachine&lt;MyStates, MyEvents&gt;(&quot;disabled&quot;, {
    disabled: {
        toggleEnabled: &quot;idle&quot;
    },
    idle: {
        toggleEnabled: &quot;disabled&quot;,
        start: &quot;running&quot;
    },
    running: {
        _enter: () =&gt; {
            f.debounce(2000, &quot;stop&quot;);
        },
        stop: &quot;idle&quot;,
        toggleEnabled: &quot;disabled&quot;
    }
});
    </code></pre>
<img src="out_runed/utilities/finite-state-machine/index_media/4e76ed27ee39219530d490611486e3c005c77476.svg" />
</figure>

## <a href="https://runed.dev/docs/utilities/finite-state-machine#usage" class="text-brand-link hover:text-brand-link-hover leading-7" aria-hidden="true" tabindex="-1"></a>Usage

Finite state machines (often abbreviated as "FSMs") are useful for tracking and manipulating something that could be in one of many different states. It centralizes the definition of every possible *state* and the *events* that might trigger a transition from one state to another. Here is a state machine describing a simple toggle switch:

<figure data-rehype-pretty-code-figure="">
<pre class="ring" data-language="ts" data-theme="github-light github-dark" tabindex="0"><code>
        import { FiniteStateMachine } from &quot;runed&quot;;
type MyStates = &quot;on&quot; | &quot;off&quot;;
type MyEvents = &quot;toggle&quot;;
 
const f = new FiniteStateMachine&lt;MyStates, MyEvents&gt;(&quot;off&quot;, {
    off: {
        toggle: &quot;on&quot;
    },
    on: {
        toggle: &quot;off&quot;
    }
});
    </code></pre>
<img src="out_runed/utilities/finite-state-machine/index_media/4e76ed27ee39219530d490611486e3c005c77476.svg" />
</figure>

The first argument to the `FiniteStateMachine` constructor is the initial state. The second argument is an object with one key for each state. Each state then describes which events are valid for that state, and which state that event should lead to.

In the above example of a simple switch, there are two states (`on` and `off`). The `toggle` event in either state leads to the other state.

You send events to the FSM using `f.send`. To send the `toggle` event, invoke `f.send('toggle')`.

### <a href="https://runed.dev/docs/utilities/finite-state-machine#actions" class="text-brand-link hover:text-brand-link-hover leading-7" aria-hidden="true" tabindex="-1"></a>Actions

Maybe you want fancier logic for an event handler, or you want to conditionally transition into another state. Instead of strings, you can use *actions*.

An action is a function that returns a state. An action can receive parameters, and it can use those parameters to dynamically choose which state should come next. It can also prevent a state transition by returning nothing.

<figure data-rehype-pretty-code-figure="">
<pre class="ring" data-language="ts" data-theme="github-light github-dark" tabindex="0"><code>
        import { FiniteStateMachine } from &quot;runed&quot;;
type MyStates = &quot;on&quot; | &quot;off&quot; | &quot;cooldown&quot;;
type MyEvents = &quot;toggle&quot;;
 
const f = new FiniteStateMachine&lt;MyStates, MyEvents&gt;(&quot;off&quot;, {
    off: {
        toggle: () =&gt; {
            if (isTuesday) {
                // Switch can only turn on during Tuesdays
                return &quot;on&quot;;
            }
            // All other days, nothing is returned and state is unchanged.
        }
    },
    on: {
        toggle: (heldMillis: number) =&gt; {
            // You can also dynamically return the next state!
            // Only turn off if switch is depressed for 3 seconds
            if (heldMillis &gt; 3000) {
                return &quot;off&quot;;
            }
        }
    }
});
    </code></pre>
<img src="out_runed/utilities/finite-state-machine/index_media/4e76ed27ee39219530d490611486e3c005c77476.svg" />
</figure>

### <a href="https://runed.dev/docs/utilities/finite-state-machine#lifecycle-methods" class="text-brand-link hover:text-brand-link-hover leading-7" aria-hidden="true" tabindex="-1"></a>Lifecycle methods

You can define special handlers that are invoked whenever a state is entered or exited:

<figure data-rehype-pretty-code-figure="">
<pre class="ring" data-language="ts" data-theme="github-light github-dark" tabindex="0"><code>
        import { FiniteStateMachine } from &quot;runed&quot;;
type MyStates = &quot;on&quot; | &quot;off&quot;;
type MyEvents = &quot;toggle&quot;;
 
const f = new FiniteStateMachine&lt;MyStates, MyEvents&gt;(&quot;off&quot;, {
    off: {
        toggle: &quot;on&quot;,
        _enter: (meta) =&gt; {
            console.log(&quot;switch is off&quot;);
        },
        _exit: (meta) =&gt; {
            console.log(&quot;switch is no longer off&quot;);
        }
    },
    on: {
        toggle: &quot;off&quot;,
        _enter: (meta) =&gt; {
            console.log(&quot;switch is on&quot;);
        },
        _exit: (meta) =&gt; {
            console.log(&quot;switch is no longer on&quot;);
        }
    }
});
    </code></pre>
<img src="out_runed/utilities/finite-state-machine/index_media/4e76ed27ee39219530d490611486e3c005c77476.svg" />
</figure>

The lifecycle methods are invoked with a metadata object containing some useful information:

- `from`: the name of the event that is being exited
- `to`: the name of the event that is being entered
- `event`: the name of the event which has triggered the transition
- `args`: (optional) you may pass additional metadata when invoking an action with `f.send('theAction', additional, params, as, args)`

The `_enter` handler for the initial state is called upon creation of the FSM. It is invoked with both the `from` and `event` fields set to `null`.

### <a href="https://runed.dev/docs/utilities/finite-state-machine#wildcard-handlers" class="text-brand-link hover:text-brand-link-hover leading-7" aria-hidden="true" tabindex="-1"></a>Wildcard handlers

There is one special state used as a fallback: `*`. If you have the fallback state, and you attempt to `send()` an event that is not handled by the current state, then it will try to find a handler for that event on the `*` state before discarding the event:

<figure data-rehype-pretty-code-figure="">
<pre class="ring" data-language="ts" data-theme="github-light github-dark" tabindex="0"><code>
        import { FiniteStateMachine } from &quot;runed&quot;;
type MyStates = &quot;on&quot; | &quot;off&quot;;
type MyEvents = &quot;toggle&quot; | &quot;emergency&quot;;
 
const f = new FiniteStateMachine&lt;MyStates, MyEvents&gt;(&quot;off&quot;, {
    off: {
        toggle: &quot;on&quot;
    },
    on: {
        toggle: &quot;off&quot;
    },
    &quot;*&quot;: {
        emergency: &quot;off&quot;
    }
});
 
// will always result in the switch turning off.
f.send(&quot;emergency&quot;);
    </code></pre>
<img src="out_runed/utilities/finite-state-machine/index_media/4e76ed27ee39219530d490611486e3c005c77476.svg" />
</figure>

### <a href="https://runed.dev/docs/utilities/finite-state-machine#debouncing" class="text-brand-link hover:text-brand-link-hover leading-7" aria-hidden="true" tabindex="-1"></a>Debouncing

Frequently, you want to transition to another state after some time has elapsed. To do this, use the `debounce` method:

<figure data-rehype-pretty-code-figure="">
<pre class="ring" data-language="ts" data-theme="github-light github-dark" tabindex="0"><code>
        f.send(&quot;toggle&quot;); // turn on immediately
f.debounce(5000, &quot;toggle&quot;); // turn off in 5000 milliseconds
    </code></pre>
<img src="out_runed/utilities/finite-state-machine/index_media/4e76ed27ee39219530d490611486e3c005c77476.svg" />
</figure>

If you re-invoke debounce with the same event, it will cancel the existing timer and start the countdown over:

<figure data-rehype-pretty-code-figure="">
<pre class="ring" data-language="ts" data-theme="github-light github-dark" tabindex="0"><code>
        // schedule a toggle in five seconds
f.debounce(5000, &quot;toggle&quot;);
// ... less than 5000ms elapses ...
f.debounce(5000, &quot;toggle&quot;);
// The second call cancels the original timer, and starts a new one
    </code></pre>
<img src="out_runed/utilities/finite-state-machine/index_media/4e76ed27ee39219530d490611486e3c005c77476.svg" />
</figure>

You can also use `debounce` in both actions and lifecycle methods. In both of the following examples, the lightswitch will turn itself off five seconds after it was turned on:

<figure data-rehype-pretty-code-figure="">
<pre class="ring" data-language="ts" data-theme="github-light github-dark" tabindex="0"><code>
        const f = new FiniteStateMachine&lt;MyStates, MyEvents&gt;(&quot;off&quot;, {
    off: {
        toggle: () =&gt; {
            f.debounce(5000, &quot;toggle&quot;);
            return &quot;on&quot;;
        }
    },
    on: {
        toggle: &quot;off&quot;
    }
});
    </code></pre>
<img src="out_runed/utilities/finite-state-machine/index_media/4e76ed27ee39219530d490611486e3c005c77476.svg" />
</figure>

<figure data-rehype-pretty-code-figure="">
<pre class="ring" data-language="ts" data-theme="github-light github-dark" tabindex="0"><code>
        const f = new FiniteStateMachine&lt;MyStates, MyEvents&gt;(&quot;off&quot;, {
    off: {
        toggle: &quot;on&quot;
    },
    on: {
        toggle: &quot;off&quot;,
        _enter: () =&gt; {
            f.debounce(5000, &quot;toggle&quot;);
        }
    }
});
    </code></pre>
<img src="out_runed/utilities/finite-state-machine/index_media/4e76ed27ee39219530d490611486e3c005c77476.svg" />
</figure>

## <a href="https://runed.dev/docs/utilities/finite-state-machine#notes" class="text-brand-link hover:text-brand-link-hover leading-7" aria-hidden="true" tabindex="-1"></a>Notes

`FiniteStateMachine` is a loving rewrite of <a href="https://github.com/kenkunz/svelte-fsm" class="text-brand-link hover:text-brand-link-hover leading-7" rel="noopener noreferrer" target="_blank">kenkunz/svelte-fsm</a>.

FSMs are ideal for representing many different kinds of systems and interaction patterns. `FiniteStateMachine` is an intentionally minimalistic implementation. If you're looking for a more powerful FSM library, <a href="https://github.com/statelyai/xstate" class="text-brand-link hover:text-brand-link-hover leading-7" rel="noopener noreferrer" target="_blank">statelyai/xstate</a> is an excellent library with more features — and a steeper learning curve.

## Contributors

<a href="https://github.com/huntabyte" class="group flex items-center gap-2 rounded-lg px-2 py-1.5" target="_blank"></a>

<img src="out_runed/utilities/finite-state-machine/index_media/75bcb1d595b8d37466d561f08b890ffc8f1d5425.jpg" id="bits-556" class="rounded-full" style="display: block;" data-avatar-image="" data-status="loaded" onerror="this.__e=event" onload="this.__e=event" alt="Hunter Johnston" />

Hunter Johnston<a href="https://github.com/43081j" class="group flex items-center gap-2 rounded-lg px-2 py-1.5" target="_blank"></a>

<img src="out_runed/utilities/finite-state-machine/index_media/06f4edda8a9beea51070ce54837509dcdbea04f6.png" id="bits-558" class="rounded-full" style="display: block;" data-avatar-image="" data-status="loaded" onerror="this.__e=event" onload="this.__e=event" alt="James Garbutt" />

James Garbutt<a href="https://github.com/sukeshpabolu" class="group flex items-center gap-2 rounded-lg px-2 py-1.5" target="_blank"></a>

<img src="out_runed/utilities/finite-state-machine/index_media/5ac1e8f2d1039c697836f74df59010f5f5f05535.png" id="bits-560" class="rounded-full" style="display: block;" data-avatar-image="" data-status="loaded" onerror="this.__e=event" onload="this.__e=event" alt="sukeshpabolu" />

sukeshpabolu<a href="https://github.com/idan" class="group flex items-center gap-2 rounded-lg px-2 py-1.5" target="_blank"></a>

<img src="out_runed/utilities/finite-state-machine/index_media/db1212ff3c46e6aed5a9d308317a79ffc2a776a6.jpg" id="bits-562" class="rounded-full" style="display: block;" data-avatar-image="" data-status="loaded" onerror="this.__e=event" onload="this.__e=event" alt="Idan Gazit" />

Idan Gazit

© 2025 <a href="https://github.com/svecosystem" class="underline-offset-2 hover:underline" target="_blank">Svecosystem</a>

<a href="https://github.com/svecosystem/runed" class="inline-flex select-none items-center justify-center gap-2 whitespace-nowrap text-sm font-semibold tracking-wide active:scale-[99%] disabled:pointer-events-none disabled:opacity-50 [&amp;_svg]:pointer-events-none [&amp;_svg]:size-4 [&amp;_svg]:shrink-0 text-foreground hover:dark:bg-foreground/5 hover:dark:ring-foreground/5 hover:bg-foreground/2 hover:ring-foreground/2 bg-transparent ring ring-transparent h-10 w-10 rounded-md [&amp;&gt;svg]:size-5" aria-label="View this project on GitHub" rel="noopener noreferrer" target="_blank"><img src="out_runed/utilities/finite-state-machine/index_media/1d22f722c87a133f2381965da1acf7a54aefc45a.svg" /></a>

<img src="out_runed/utilities/finite-state-machine/index_media/b2f1f2409c2d7a3eaec75643beb67d5092b71fb2.svg" class="size-6" />

# FiniteStateMachine

Defines a strongly-typed finite state machine.

## <a href="https://runed.dev/docs/utilities/finite-state-machine#demo" class="text-brand-link hover:text-brand-link-hover leading-7" aria-hidden="true" tabindex="-1"></a>Demo

# Now: `disabled`

Toggle the switch to enable.

![](out_runed/utilities/finite-state-machine/index_media/6ff2106ab77bb4fa5ddad76193ac26e165bb072f.svg) Run

<figure data-rehype-pretty-code-figure="">
<pre class="ring" data-language="ts" data-theme="github-light github-dark" tabindex="0"><code>
        import { FiniteStateMachine } from &quot;runed&quot;;
type MyStates = &quot;disabled&quot; | &quot;idle&quot; | &quot;running&quot;;
type MyEvents = &quot;toggleEnabled&quot; | &quot;start&quot; | &quot;stop&quot;;
 
const f = new FiniteStateMachine&lt;MyStates, MyEvents&gt;(&quot;disabled&quot;, {
    disabled: {
        toggleEnabled: &quot;idle&quot;
    },
    idle: {
        toggleEnabled: &quot;disabled&quot;,
        start: &quot;running&quot;
    },
    running: {
        _enter: () =&gt; {
            f.debounce(2000, &quot;stop&quot;);
        },
        stop: &quot;idle&quot;,
        toggleEnabled: &quot;disabled&quot;
    }
});
    </code></pre>
<img src="out_runed/utilities/finite-state-machine/index_media/4e76ed27ee39219530d490611486e3c005c77476.svg" />
</figure>

## <a href="https://runed.dev/docs/utilities/finite-state-machine#usage" class="text-brand-link hover:text-brand-link-hover leading-7" aria-hidden="true" tabindex="-1"></a>Usage

Finite state machines (often abbreviated as "FSMs") are useful for tracking and manipulating something that could be in one of many different states. It centralizes the definition of every possible *state* and the *events* that might trigger a transition from one state to another. Here is a state machine describing a simple toggle switch:

<figure data-rehype-pretty-code-figure="">
<pre class="ring" data-language="ts" data-theme="github-light github-dark" tabindex="0"><code>
        import { FiniteStateMachine } from &quot;runed&quot;;
type MyStates = &quot;on&quot; | &quot;off&quot;;
type MyEvents = &quot;toggle&quot;;
 
const f = new FiniteStateMachine&lt;MyStates, MyEvents&gt;(&quot;off&quot;, {
    off: {
        toggle: &quot;on&quot;
    },
    on: {
        toggle: &quot;off&quot;
    }
});
    </code></pre>
<img src="out_runed/utilities/finite-state-machine/index_media/4e76ed27ee39219530d490611486e3c005c77476.svg" />
</figure>

The first argument to the `FiniteStateMachine` constructor is the initial state. The second argument is an object with one key for each state. Each state then describes which events are valid for that state, and which state that event should lead to.

In the above example of a simple switch, there are two states (`on` and `off`). The `toggle` event in either state leads to the other state.

You send events to the FSM using `f.send`. To send the `toggle` event, invoke `f.send('toggle')`.

### <a href="https://runed.dev/docs/utilities/finite-state-machine#actions" class="text-brand-link hover:text-brand-link-hover leading-7" aria-hidden="true" tabindex="-1"></a>Actions

Maybe you want fancier logic for an event handler, or you want to conditionally transition into another state. Instead of strings, you can use *actions*.

An action is a function that returns a state. An action can receive parameters, and it can use those parameters to dynamically choose which state should come next. It can also prevent a state transition by returning nothing.

<figure data-rehype-pretty-code-figure="">
<pre class="ring" data-language="ts" data-theme="github-light github-dark" tabindex="0"><code>
        import { FiniteStateMachine } from &quot;runed&quot;;
type MyStates = &quot;on&quot; | &quot;off&quot; | &quot;cooldown&quot;;
type MyEvents = &quot;toggle&quot;;
 
const f = new FiniteStateMachine&lt;MyStates, MyEvents&gt;(&quot;off&quot;, {
    off: {
        toggle: () =&gt; {
            if (isTuesday) {
                // Switch can only turn on during Tuesdays
                return &quot;on&quot;;
            }
            // All other days, nothing is returned and state is unchanged.
        }
    },
    on: {
        toggle: (heldMillis: number) =&gt; {
            // You can also dynamically return the next state!
            // Only turn off if switch is depressed for 3 seconds
            if (heldMillis &gt; 3000) {
                return &quot;off&quot;;
            }
        }
    }
});
    </code></pre>
<img src="out_runed/utilities/finite-state-machine/index_media/4e76ed27ee39219530d490611486e3c005c77476.svg" />
</figure>

### <a href="https://runed.dev/docs/utilities/finite-state-machine#lifecycle-methods" class="text-brand-link hover:text-brand-link-hover leading-7" aria-hidden="true" tabindex="-1"></a>Lifecycle methods

You can define special handlers that are invoked whenever a state is entered or exited:

<figure data-rehype-pretty-code-figure="">
<pre class="ring" data-language="ts" data-theme="github-light github-dark" tabindex="0"><code>
        import { FiniteStateMachine } from &quot;runed&quot;;
type MyStates = &quot;on&quot; | &quot;off&quot;;
type MyEvents = &quot;toggle&quot;;
 
const f = new FiniteStateMachine&lt;MyStates, MyEvents&gt;(&quot;off&quot;, {
    off: {
        toggle: &quot;on&quot;,
        _enter: (meta) =&gt; {
            console.log(&quot;switch is off&quot;);
        },
        _exit: (meta) =&gt; {
            console.log(&quot;switch is no longer off&quot;);
        }
    },
    on: {
        toggle: &quot;off&quot;,
        _enter: (meta) =&gt; {
            console.log(&quot;switch is on&quot;);
        },
        _exit: (meta) =&gt; {
            console.log(&quot;switch is no longer on&quot;);
        }
    }
});
    </code></pre>
<img src="out_runed/utilities/finite-state-machine/index_media/4e76ed27ee39219530d490611486e3c005c77476.svg" />
</figure>

The lifecycle methods are invoked with a metadata object containing some useful information:

- `from`: the name of the event that is being exited
- `to`: the name of the event that is being entered
- `event`: the name of the event which has triggered the transition
- `args`: (optional) you may pass additional metadata when invoking an action with `f.send('theAction', additional, params, as, args)`

The `_enter` handler for the initial state is called upon creation of the FSM. It is invoked with both the `from` and `event` fields set to `null`.

### <a href="https://runed.dev/docs/utilities/finite-state-machine#wildcard-handlers" class="text-brand-link hover:text-brand-link-hover leading-7" aria-hidden="true" tabindex="-1"></a>Wildcard handlers

There is one special state used as a fallback: `*`. If you have the fallback state, and you attempt to `send()` an event that is not handled by the current state, then it will try to find a handler for that event on the `*` state before discarding the event:

<figure data-rehype-pretty-code-figure="">
<pre class="ring" data-language="ts" data-theme="github-light github-dark" tabindex="0"><code>
        import { FiniteStateMachine } from &quot;runed&quot;;
type MyStates = &quot;on&quot; | &quot;off&quot;;
type MyEvents = &quot;toggle&quot; | &quot;emergency&quot;;
 
const f = new FiniteStateMachine&lt;MyStates, MyEvents&gt;(&quot;off&quot;, {
    off: {
        toggle: &quot;on&quot;
    },
    on: {
        toggle: &quot;off&quot;
    },
    &quot;*&quot;: {
        emergency: &quot;off&quot;
    }
});
 
// will always result in the switch turning off.
f.send(&quot;emergency&quot;);
    </code></pre>
<img src="out_runed/utilities/finite-state-machine/index_media/4e76ed27ee39219530d490611486e3c005c77476.svg" />
</figure>

### <a href="https://runed.dev/docs/utilities/finite-state-machine#debouncing" class="text-brand-link hover:text-brand-link-hover leading-7" aria-hidden="true" tabindex="-1"></a>Debouncing

Frequently, you want to transition to another state after some time has elapsed. To do this, use the `debounce` method:

<figure data-rehype-pretty-code-figure="">
<pre class="ring" data-language="ts" data-theme="github-light github-dark" tabindex="0"><code>
        f.send(&quot;toggle&quot;); // turn on immediately
f.debounce(5000, &quot;toggle&quot;); // turn off in 5000 milliseconds
    </code></pre>
<img src="out_runed/utilities/finite-state-machine/index_media/4e76ed27ee39219530d490611486e3c005c77476.svg" />
</figure>

If you re-invoke debounce with the same event, it will cancel the existing timer and start the countdown over:

<figure data-rehype-pretty-code-figure="">
<pre class="ring" data-language="ts" data-theme="github-light github-dark" tabindex="0"><code>
        // schedule a toggle in five seconds
f.debounce(5000, &quot;toggle&quot;);
// ... less than 5000ms elapses ...
f.debounce(5000, &quot;toggle&quot;);
// The second call cancels the original timer, and starts a new one
    </code></pre>
<img src="out_runed/utilities/finite-state-machine/index_media/4e76ed27ee39219530d490611486e3c005c77476.svg" />
</figure>

You can also use `debounce` in both actions and lifecycle methods. In both of the following examples, the lightswitch will turn itself off five seconds after it was turned on:

<figure data-rehype-pretty-code-figure="">
<pre class="ring" data-language="ts" data-theme="github-light github-dark" tabindex="0"><code>
        const f = new FiniteStateMachine&lt;MyStates, MyEvents&gt;(&quot;off&quot;, {
    off: {
        toggle: () =&gt; {
            f.debounce(5000, &quot;toggle&quot;);
            return &quot;on&quot;;
        }
    },
    on: {
        toggle: &quot;off&quot;
    }
});
    </code></pre>
<img src="out_runed/utilities/finite-state-machine/index_media/4e76ed27ee39219530d490611486e3c005c77476.svg" />
</figure>

<figure data-rehype-pretty-code-figure="">
<pre class="ring" data-language="ts" data-theme="github-light github-dark" tabindex="0"><code>
        const f = new FiniteStateMachine&lt;MyStates, MyEvents&gt;(&quot;off&quot;, {
    off: {
        toggle: &quot;on&quot;
    },
    on: {
        toggle: &quot;off&quot;,
        _enter: () =&gt; {
            f.debounce(5000, &quot;toggle&quot;);
        }
    }
});
    </code></pre>
<img src="out_runed/utilities/finite-state-machine/index_media/4e76ed27ee39219530d490611486e3c005c77476.svg" />
</figure>

## <a href="https://runed.dev/docs/utilities/finite-state-machine#notes" class="text-brand-link hover:text-brand-link-hover leading-7" aria-hidden="true" tabindex="-1"></a>Notes

`FiniteStateMachine` is a loving rewrite of <a href="https://github.com/kenkunz/svelte-fsm" class="text-brand-link hover:text-brand-link-hover leading-7" rel="noopener noreferrer" target="_blank">kenkunz/svelte-fsm</a>.

FSMs are ideal for representing many different kinds of systems and interaction patterns. `FiniteStateMachine` is an intentionally minimalistic implementation. If you're looking for a more powerful FSM library, <a href="https://github.com/statelyai/xstate" class="text-brand-link hover:text-brand-link-hover leading-7" rel="noopener noreferrer" target="_blank">statelyai/xstate</a> is an excellent library with more features — and a steeper learning curve.

## Contributors

<a href="https://github.com/huntabyte" class="group flex items-center gap-2 rounded-lg px-2 py-1.5" target="_blank"></a>

<img src="out_runed/utilities/finite-state-machine/index_media/75bcb1d595b8d37466d561f08b890ffc8f1d5425.jpg" id="bits-556" class="rounded-full" style="display: block;" data-avatar-image="" data-status="loaded" onerror="this.__e=event" onload="this.__e=event" alt="Hunter Johnston" />

Hunter Johnston<a href="https://github.com/43081j" class="group flex items-center gap-2 rounded-lg px-2 py-1.5" target="_blank"></a>

<img src="out_runed/utilities/finite-state-machine/index_media/06f4edda8a9beea51070ce54837509dcdbea04f6.png" id="bits-558" class="rounded-full" style="display: block;" data-avatar-image="" data-status="loaded" onerror="this.__e=event" onload="this.__e=event" alt="James Garbutt" />

James Garbutt<a href="https://github.com/sukeshpabolu" class="group flex items-center gap-2 rounded-lg px-2 py-1.5" target="_blank"></a>

<img src="out_runed/utilities/finite-state-machine/index_media/5ac1e8f2d1039c697836f74df59010f5f5f05535.png" id="bits-560" class="rounded-full" style="display: block;" data-avatar-image="" data-status="loaded" onerror="this.__e=event" onload="this.__e=event" alt="sukeshpabolu" />

sukeshpabolu<a href="https://github.com/idan" class="group flex items-center gap-2 rounded-lg px-2 py-1.5" target="_blank"></a>

<img src="out_runed/utilities/finite-state-machine/index_media/db1212ff3c46e6aed5a9d308317a79ffc2a776a6.jpg" id="bits-562" class="rounded-full" style="display: block;" data-avatar-image="" data-status="loaded" onerror="this.__e=event" onload="this.__e=event" alt="Idan Gazit" />

Idan Gazit
