# FileOptions in supabase_storage_rs::models - Rust

```
pub struct FileOptions<'a> {
    pub cache_control: Option<Duration>,
    pub content_type: Option<&'a str>,
    pub duplex: Option<&'a str>,
    pub upsert: bool,
}
```

The number of seconds the asset is cached in the browser and Supabase CDN Sets the Cache-Control: max-age= header Defaults to 3600 seconds

[§](#structfield.content_type)`content_type: [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'a [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)>`

The Content-Type header value Required if using a fileBody that is neither Blob, File, nor FormData Defaults to “text/plain;charset=UTF-8”

Enables or disables duplex streaming for reading and writing data in the same stream

When true, the file is overwritten if it exists When false, an error is thrown if the object already exists Defaults to false

[§](#impl-Freeze-for-FileOptions%3C'a%3E)

[§](#impl-RefUnwindSafe-for-FileOptions%3C'a%3E)

[§](#impl-Send-for-FileOptions%3C'a%3E)

[§](#impl-Sync-for-FileOptions%3C'a%3E)

[§](#impl-Unpin-for-FileOptions%3C'a%3E)

[§](#impl-UnwindSafe-for-FileOptions%3C'a%3E)
