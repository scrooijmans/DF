89

It's possible. You can add a Send constraint to a trait object like this:

let foo = Box::new(Foo { foo: 1 }) as Box<dyn Bar + Send>;

let (tx, rx): (Sender<Box<dyn Bar + Send>>, Receiver<Box<dyn Bar + Send>>) = channel();