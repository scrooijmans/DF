Rust vs JavaScript: Achieving 66% Faster Performance with WebAssembly
Discover how Web Workers and WebAssembly can dramatically improve your JavaScript app’s performance, using the Fibonacci algorithm as a case study.
Titus Efferian
Titus Efferian

Follow
7 min read
·
Sep 12, 2024
590


8





Zoom image will be displayed

Illustration showing a comparison between JavaScript and AssemblyScript versus Rust with WebAssembly
JavaScript typically runs on a single thread, often referred to as the
“main thread.” This means that JavaScript executes one task at a time in a synchronous manner. The main thread also handles rendering tasks such as painting and layout, along with user interactions, meaning that long-running JavaScript tasks can cause the browser to become unresponsive. This is why web pages may “freeze” when a heavy JavaScript function runs, blocking user interactions

We are going to demonstrate how to block the main thread by simulating heavy computation using the Fibonacci algorithm, and we will solve the blocked main thread using several approaches such as:

multi-threaded (Web Worker),
WebAssembly using AssemblyScript,
WebAssembly using Rust.
Fibonacci Algorithm
We are going to use the simple and very common Fibonacci algorithm with a time complexity of O(2^n) for all of our case studies in this article.

const calculateFibonacci = (n: number): number => {
  if (n <= 1) return n;
  return calculateFibonacci(n - 1) + calculateFibonacci(n - 2);
};
Single Thread
Now, let’s implement the Fibonacci algorithm directly on the main thread. Simply call the Fibonacci function when the button is clicked.

"use client";
import { useState } from "react";

/**
 * simulate loading animation
 */
function Spinner() {
  return (
    <div className="flex justify-center items-center">
      <div className="animate-spin rounded-full h-16 w-16 border-t-4 border-b-4 border-blue-500"></div>
    </div>
  );
}

export default function Home() {
  const [result, setResult] = useState<number | null>(null);
  const [isLoading, setIsLoading] = useState<boolean>(false);

  const calculateFibonacci = (n: number): number => {
    if (n <= 1) return n;
    return calculateFibonacci(n - 1) + calculateFibonacci(n - 2);
  };

  const handleCalculate = () => {
    setIsLoading(true);
    /**
     * simulate a long-running calculation
     */
    const result = calculateFibonacci(42);
    setResult(result);
    setIsLoading(false);
  };

  return (
    <div className="flex flex-col items-center justify-center min-h-screen bg-gray-900 text-white">
      <button
        onClick={handleCalculate}
        className="mb-8 px-6 py-2 bg-blue-500 text-white rounded-md hover:bg-blue-600 transition"
      >
        Calculate Fibonacci
      </button>
      {isLoading ? <Spinner /> : <p className="text-xl">Result: {result}</p>}
    </div>
  );
}
Now, let’s try clicking the ‘Calculate Fibonacci’ button while measuring performance. To measure the performance of our code, we can use the performance tools in Chrome DevTools.

As you can see in the UI, our spinner button doesn’t even appear and instead suddenly shows the calculation result. We can also see from the performance tools that our spinning animation is blocked by the heavy computation of the Fibonacci algorithm on the main thread for about 2.06 seconds.

Zoom image will be displayed

Zoom image will be displayed

Performance tools showing the main thread being blocked for 2 seconds.
Multi-Threaded (Web Worker)
The common approach for offloading heavy computation from the main thread is using a Web Worker.

/**
 * move the fibonacci algorithm to web worker
 */
self.addEventListener("message", function (e) {
  const n = e.data;

  const fibonacci = (n) => {
    if (n <= 1) return n;
    return fibonacci(n - 1) + fibonacci(n - 2);
  };

  const result = fibonacci(n);
  self.postMessage(result);
});
"use client";
import { useState } from "react";

function Spinner() {
  return (
    <div className="flex justify-center items-center">
      <div className="animate-spin rounded-full h-16 w-16 border-t-4 border-b-4 border-blue-500"></div>
    </div>
  );
}

export default function Home() {
  const [result, setResult] = (useState < number) | (null > null);
  const [isLoading, setIsLoading] = useState < boolean > false;

  /**
   * instead of running the fibonacci function in the main thread,
   * we will run it in a web worker
   */
  const handleCalculate = () => {
    setIsLoading(true);

    const worker = new Worker(
      new URL("./fibonacci-worker.js", import.meta.url),
    );

    worker.postMessage(42);

    worker.onmessage = (e) => {
      setResult(e.data);
      setIsLoading(false);
      worker.terminate();
    };
  };

  return (
    <div className="flex flex-col items-center justify-center min-h-screen bg-gray-900 text-white">
      <button
        onClick={handleCalculate}
        className="mb-8 px-6 py-2 bg-blue-500 text-white rounded-md hover:bg-blue-600 transition"
      >
        Calculate Fibonacci
      </button>
      {isLoading ? <Spinner /> : <p className="text-xl">Result: {result}</p>}
    </div>
  );
}
Now, if we try to measure, the spinner animation runs smoothly. This is because we offloaded the heavy computation to a worker thread, avoiding blocking the main thread.

As you can see, both the single-threaded and worker-threaded computations take a similar duration of around 2 seconds. The question now is, how can we improve this? The answer is by using WebAssembly.

Zoom image will be displayed

Zoom image will be displayed

Performance tools showing that heavy computation now runs on a worker.
WebAssembly — AssemblyScript
As a frontend engineer with limited experience in other languages who wants to try WebAssembly, we typically choose AssemblyScript because it offers a developer experience closest to TypeScript.

Here is the equivalent Fibonacci code written in AssemblyScript.

export function fibonacci(n: i32): i32 {
  if (n <= 1) return n;
  return fibonacci(n - 1) + fibonacci(n - 2);
}
If we compile that code, it will generate a release.wasm file. We can then use this Wasm file in our JavaScript codebase.

"use client";
import { useState } from "react";

function Spinner() {
  return (
    <div className="flex justify-center items-center">
      <div className="animate-spin rounded-full h-16 w-16 border-t-4 border-b-4 border-blue-500"></div>
    </div>
  );
}

export default function Home() {
  const [result, setResult] = (useState < number) | (null > null);
  const [isLoading, setIsLoading] = useState < boolean > false;

  const handleCalculate = async () => {
    setIsLoading(true);

    // Load and instantiate the WebAssembly module
    const wasmModule = await fetch("/release.wasm");
    const buffer = await wasmModule.arrayBuffer();
    const module = await WebAssembly.instantiate(buffer);
    const wasm = module.instance.exports;

    // Call the Fibonacci function from the WebAssembly module
    const fibResult = wasm.fibonacci(42);

    setResult(fibResult);
    setIsLoading(false);
  };

  return (
    <div className="flex flex-col items-center justify-center min-h-screen bg-gray-900 text-white">
      <button
        onClick={handleCalculate}
        className="mb-8 px-6 py-2 bg-blue-500 text-white rounded-md hover:bg-blue-600 transition"
      >
        Calculate Fibonacci
      </button>
      {isLoading ? <Spinner /> : <p className="text-xl">Result: {result}</p>}
    </div>
  );
}
Now, if we measure this again, even though we are still on the main thread, the loading animation appears and is not blocked by the heavy computation. The Fibonacci algorithm now takes around 950ms, which is 53% faster than using only JavaScript.

Zoom image will be displayed

Zoom image will be displayed

Performance tools showing AssemblyScript is 53% faster than JavaScript.
WebAssembly — Rust
Rust is one of the popular choices for WebAssembly, as highlighted by Mozilla’s official documentation. Let’s try to implement the same Fibonacci algorithm, but written in Rust.

use wasm_bindgen::prelude::*;

// Expose the function to JavaScript through WebAssembly
#[wasm_bindgen]
pub fn fibonacci(n: u32) -> u32 {
    match n {
        0 => 0,
        1 => 1,
        _ => fibonacci(n - 1) + fibonacci(n - 2),
    }
}
"use client";
import { useState } from "react";

function Spinner() {
  return (
    <div className="flex justify-center items-center">
      <div className="animate-spin rounded-full h-16 w-16 border-t-4 border-b-4 border-blue-500"></div>
    </div>
  );
}

export default function Home() {
  const [result, setResult] = (useState < number) | (null > null);
  const [isLoading, setIsLoading] = useState < boolean > false;

  const handleCalculate = async () => {
    setIsLoading(true);

    // Load and instantiate the WebAssembly module
    const wasmModule = await fetch("/pkg/rust_wasm_fibonacci_bg.wasm"); // Use the actual wasm file
    const buffer = await wasmModule.arrayBuffer();

    const module = await WebAssembly.instantiate(buffer);
    const wasm = module.instance.exports;

    // Call the Fibonacci function from the WebAssembly module
    const fibResult = wasm.fibonacci(42); // Assuming the function is exported as 'fibonacci'

    setResult(fibResult);
    setIsLoading(false);
  };

  return (
    <div className="flex flex-col items-center justify-center min-h-screen bg-gray-900 text-white">
      <button
        onClick={handleCalculate}
        className="mb-8 px-6 py-2 bg-blue-500 text-white rounded-md hover:bg-blue-600 transition"
      >
        Calculate Fibonacci
      </button>
      {isLoading ? <Spinner /> : <p className="text-xl">Result: {result}</p>}
    </div>
  );
}
Now, let’s examine the result of using WebAssembly with Rust. We’re still using the main thread, but now with the Wasm. Similar to AssemblyScript, even though we’re running this Wasm on the main thread, the loading animation still appears and is not blocked. The amazing thing is that this heavy computation now takes only 684ms, which is 66% faster than using just JavaScript.

Zoom image will be displayed

Zoom image will be displayed

Performance tools showing Rust is 66% faster than JavaScript.
TL;DR and Conclusion
Heavy computation will block the main thread and stop all animations.
Heavy computation can be offloaded to a Web Worker.
Heavy computation can be improved by rewriting the logic in WebAssembly. Using the Fibonacci algorithm as a case study, we obtained the following results:
- JavaScript: 2s
- WebAssembly — AssemblyScript: 953ms (53% faster than JavaScript)
- WebAssembly — Rust: 684ms (66% faster than JavaScript)
References