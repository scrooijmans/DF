13

Box<dyn Error> is an opaque type. It may contain any type.

Suppose it contains a type that cannot be safely send to a different thread than the one it was created in. For example, a MutexGuard, that must release the mutex when dropped from the same thread as it acquired it. Or an Rc, that decrements the reference count on drop non-atomically, and thus can cause a data race on drop if moved to another thread. Then we should not send it to another thread. We say the type does not implement Send. Because Box<dyn Error> may contain such types, it itself does not implement Send.

However, tokio::spawn() may move the future between threads between .await points. This is to improve efficiency: tokio uses a work-stealing scheduler, meaning it will move tasks to threads (and therefore CPU cores) that are less busy. But suppose the Box<dyn Error> would contain a type that does not implement Send, for example MutexGuard, it would be created before the tokio::join!() call, in thread A, and dropped after it, potentially in thread B! (because tokio::join() has an implicit .await). This mean this is not safe, therefore the future of foo() is not Send either, and you cannot spawn it into a task.

The fix is simple: ensure the Box<dyn Error> is Send. This can be done by adding a + Send bound to it:

async fn bar() -> Result<(), Box<dyn std::error::Error + Send>> {
    println!("Hello world");
    Ok(())
}

It's impossible to convert a Box<dyn Error> to Box<dyn Error + Send + Sync> because a Box<dyn Error> is neither Send nor Sync.

I guess you could turn the error into a string and convert that to the right error.
The good way would be to fix the thing returning a Box<dyn Error>
