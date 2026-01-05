Description: $inspect • Svelte documentation

Title: $inspect • Docs • Svelte

Skip to main content

### On this page

> `$inspect` only works during development. In a production build it becomes a noop.

The `$inspect` rune is roughly equivalent to `console.log`, with the exception that it will re-run whenever its argument changes. `$inspect` tracks reactive state deeply, meaning that updating something inside an object or array using fine-grained reactivity will cause it to re-fire (demo):

```
<script>
let count = $state(0);
let message = $state('hello');

$inspect(count, message); // will console.log when `count` or `message` change
</script>

<button onclick={() => count++}>Increment</button>
<input bind:value={message} />
```

## $inspect(...).with

`$inspect` returns a property `with`, which you can invoke with a callback, which will then be invoked instead of `console.log`. The first argument to the callback is either `"init"` or `"update"`; subsequent arguments are the values passed to `$inspect` (demo):

```
<script>
let count = $state(0);

$inspect(count).with((type, count) => {
if (type === 'update') {
debugger; // or `console.trace`, or whatever you want
}
});
</script>

<button onclick={() => count++}>Increment</button>
```

A convenient way to find the origin of some change is to pass `console.trace` to `with`:

```
function $inspect<[any]>(values_0: any): {
with: (fn: (type: "init" | "update", values_0: any) => void) => void;
}
namespace $inspectInspects one or more values whenever they, or the properties they contain, change. Example:
$inspect(someValue, someOtherValue)$inspect returns a with function, which you can invoke with a callback function that
will be called with the value and the event type ('init' or 'update') on every change.
By default, the values will be logged to the console.
$inspect(x).with(console.trace);
$inspect(x, y).with(() => { debugger; });https://svelte.dev/docs/svelte/$inspect
$inspect(stuff).with: (fn: (type: "init" | "update", values_0: any) => void) => voidwith(var console: ConsoleThe console module provides a simple debugging console that is similar to the
JavaScript console mechanism provided by web browsers.
The module exports two specific components:

A Console class with methods such as console.log(), console.error() and console.warn() that can be used to write to any Node.js stream.
A global console instance configured to write to process.stdout and
process.stderr. The global console can be used without importing the node:console module.

Warning: The global console object’s methods are neither consistently
synchronous like the browser APIs they resemble, nor are they consistently
asynchronous like all other Node.js streams. See the note on process I/O for
more information.
Example using the global console:
console.log('hello world');
// Prints: hello world, to stdout
console.log('hello %s', 'world');
// Prints: hello world, to stdout
console.error(new Error('Whoops, something bad happened'));
// Prints error message and stack trace to stderr:
//   Error: Whoops, something bad happened
//     at [eval]:5:15
//     at Script.runInThisContext (node:vm:132:18)
//     at Object.runInThisContext (node:vm:309:38)
//     at node:internal/process/execution:77:19
//     at [eval]-wrapper:6:22
//     at evalScript (node:internal/process/execution:76:60)
//     at node:internal/main/eval_string:23:3

const name = 'Will Robinson';
console.warn(`Danger ${name}! Danger!`);
// Prints: Danger Will Robinson! Danger!, to stderrExample using the Console class:
const out = getStreamSomehow();
const err = getStreamSomehow();
const myConsole = new console.Console(out, err);

myConsole.log('hello world');
// Prints: hello world, to out
myConsole.log('hello %s', 'world');
// Prints: hello world, to out
myConsole.error(new Error('Whoops, something bad happened'));
// Prints: [Error: Whoops, something bad happened], to err

const name = 'Will Robinson';
myConsole.warn(`Danger ${name}! Danger!`);
// Prints: Danger Will Robinson! Danger!, to err@seesourceconsole.Console.trace(...data: any[]): void (+1 overload)MDN Reference
trace);
```

## $inspect.trace(...)

This rune, added in 5.14, causes the surrounding function to be _traced_ in development. Any time the function re-runs as part of an effect or a derived, information will be printed to the console about which pieces of reactive state caused the effect to fire.

```
<script>
import { doSomeWork } from './elsewhere';

$effect(() => {
// $inspect.trace must be the first statement of a function body
$inspect.trace();
doSomeWork();
});
</script>
```

`$inspect.trace` takes an optional first argument which will be used as the label.

Edit this page on GitHub llms.txt

previous next

$bindable $host
