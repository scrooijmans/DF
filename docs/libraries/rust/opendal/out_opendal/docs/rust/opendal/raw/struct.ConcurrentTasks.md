# Struct ConcurrentTasks Copy item path

<a href="https://opendal.apache.org/docs/rust/src/opendal/raw/futures_util.rs.html#111-152" class="src">Source</a>

``` rust
pub struct ConcurrentTasks<I, O> { /* private fields */ }
```

Expand description

ConcurrentTasks is used to execute tasks concurrently.

ConcurrentTasks has two generic types:

- `I` represents the input type of the task.
- `O` represents the output type of the task.

## <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.ConcurrentTasks.html#implementation-notes" class="doc-anchor">Â§</a>Implementation Notes

The code patterns below are intentional; please do not modify them unless you fully understand these notes.

``` skip
 let (i, o) = self
    .tasks
    .front_mut()                                        // Use `front_mut` instead of `pop_front`
    .expect("tasks must be available")
    .await;
...
match o {
    Ok(o) => {
        let _ = self.tasks.pop_front();                 // `pop_front` after got `Ok(o)`
        self.results.push_back(o)
    }
    Err(err) => {
        if err.is_temporary() {
            let task = self.create_task(i);
            self.tasks
                .front_mut()
                .expect("tasks must be available")
                .replace(task)                          // Use replace here to instead of `push_front`
        } else {
            self.clear();
            self.errored = true;
        }
        return Err(err);
    }
}
```

Please keep in mind that there is no guarantee the task will be `await`ed until completion. Itâ€™s possible the task may be dropped before it resolves. Therefore, we should keep the `Task` in the `tasks` queue until it is resolved.

For example, users may have a timeout for the task, and the task will be dropped if it exceeds the timeout. If we `pop_front` the task before it resolves, the task will be canceled and the result will be lost.

## Implementations<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.ConcurrentTasks.html#implementations" class="anchor">Â§</a>

<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.ConcurrentTasks.html#impl-ConcurrentTasks%3CI,+O%3E" class="anchor">Â§</a>

### impl\<I: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a> + 'static, O: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a> + 'static\> <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.ConcurrentTasks.html" class="struct" title="struct opendal::raw::ConcurrentTasks">ConcurrentTasks</a>\<I, O\>

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.ConcurrentTasks.html#method.new" class="fn">new</a>( executor: <a href="https://opendal.apache.org/docs/rust/opendal/struct.Executor.html" class="struct" title="struct opendal::Executor">Executor</a>, concurrent: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, prefetch: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, factory: <a href="https://doc.rust-lang.org/nightly/std/primitive.fn.html" class="primitive">fn</a>(I) -\> <a href="https://opendal.apache.org/docs/rust/opendal/raw/type.BoxedStaticFuture.html" class="type" title="type opendal::raw::BoxedStaticFuture">BoxedStaticFuture</a>\<(I, <a href="https://opendal.apache.org/docs/rust/opendal/type.Result.html" class="type" title="type opendal::Result">Result</a>\<O\>)\>, ) -\> Self

Create a new concurrent tasks with given executor, concurrent, prefetch and factory.

The factory is a function pointer that shouldnâ€™t capture any context.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.ConcurrentTasks.html#method.clear" class="fn">clear</a>(&mut self)

Clear all tasks and results.

All ongoing tasks will be canceled.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.ConcurrentTasks.html#method.has_remaining" class="fn">has_remaining</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Check if there are remaining space to push new tasks.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.ConcurrentTasks.html#method.has_result" class="fn">has_result</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Chunk if there are remaining results to fetch.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.ConcurrentTasks.html#method.create_task" class="fn">create_task</a>(&self, input: I) -\> Task\<(I, <a href="https://opendal.apache.org/docs/rust/opendal/type.Result.html" class="type" title="type opendal::Result">Result</a>\<O\>)\>

Create a task with given input.

#### pub async fn <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.ConcurrentTasks.html#method.execute" class="fn">execute</a>(&mut self, input: I) -\> <a href="https://opendal.apache.org/docs/rust/opendal/type.Result.html" class="type" title="type opendal::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>\>

Execute the task with given input.

- Execute the task in the current thread if is not concurrent.
- Execute the task in the background if there are available slots.
- Await the first task in the queue if there is no available slots.

#### pub async fn <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.ConcurrentTasks.html#method.next" class="fn">next</a>(&mut self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://opendal.apache.org/docs/rust/opendal/type.Result.html" class="type" title="type opendal::Result">Result</a>\<O\>\>

Fetch the successful result from the result queue.

## Auto Trait Implementations<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.ConcurrentTasks.html#synthetic-implementations" class="anchor">Â§</a>

## Blanket Implementations<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.ConcurrentTasks.html#blanket-implementations" class="anchor">Â§</a>
