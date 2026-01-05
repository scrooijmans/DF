Hi,

I'm looking to execute a lot (up to 1000) of simultaneous similar tasks that will last a few seconds.

I've thought about:

Calling multiple Tauri async commands from the React side with Promise.all
Calling one Tauri async command that will spawn the needed threads
What would be the benefits and disadvantages of each one?
Answer:
Generally i'd say the "call one command and do it all in rust" is better for multi-threaded workloads, especially since js being single threaded and the communication channel most likely being a bottleneck here.
Also, multi-threading is one of rust main selling points, might as well use it :P You have more control and (in most cases) better performance this way.

The main exception here is if you would be bottlenecked by the communication channel anyway (as in: you return huge amounts of data from the commands), then it's probably more like personal preference (good idea to benchmark that i guess).

tldr: huge amount of tasks and you don't need to return much data? -> Rust