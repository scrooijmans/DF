# Upload in supabase_storage_rs::models - Rust

```
pub struct Upload<'a> {
    pub path: &'a str,
    pub file_body: Vec<u8>,
    pub file_options: Option<FileOptions<'a>>,
}
```

Expand description

Configuration options for file uploads to Supabase Storage

## Fields[§](#fields)

[§](#structfield.path)`path: &'a [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)`

The file path, including the file name (format: folder/subfolder/filename.png) The bucket must already exist before attempting to upload.

[§](#structfield.file_body)`file_body: [Vec](https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html "struct alloc::vec::Vec")<[u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html)>`

The body of the file to be stored in the bucket

[§](#structfield.file_options)`file_options: [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[FileOptions](struct.FileOptions.html "struct supabase_storage_rs::models::FileOptions")<'a>>`

Optional file configuration settings

## Trait Implementations[§](#trait-implementations)

## Auto Trait Implementations[§](#synthetic-implementations)

[§](#impl-Freeze-for-Upload%3C'a%3E)

### impl<'a> [Freeze](https://doc.rust-lang.org/nightly/core/marker/trait.Freeze.html "trait core::marker::Freeze") for [Upload](struct.Upload.html "struct supabase_storage_rs::models::Upload")<'a>

[§](#impl-RefUnwindSafe-for-Upload%3C'a%3E)

### impl<'a> [RefUnwindSafe](https://doc.rust-lang.org/nightly/core/panic/unwind_safe/trait.RefUnwindSafe.html "trait core::panic::unwind_safe::RefUnwindSafe") for [Upload](struct.Upload.html "struct supabase_storage_rs::models::Upload")<'a>

[§](#impl-Send-for-Upload%3C'a%3E)

### impl<'a> [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") for [Upload](struct.Upload.html "struct supabase_storage_rs::models::Upload")<'a>

[§](#impl-Sync-for-Upload%3C'a%3E)

### impl<'a> [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") for [Upload](struct.Upload.html "struct supabase_storage_rs::models::Upload")<'a>

[§](#impl-Unpin-for-Upload%3C'a%3E)

### impl<'a> [Unpin](https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html "trait core::marker::Unpin") for [Upload](struct.Upload.html "struct supabase_storage_rs::models::Upload")<'a>

[§](#impl-UnwindSafe-for-Upload%3C'a%3E)

### impl<'a> [UnwindSafe](https://doc.rust-lang.org/nightly/core/panic/unwind_safe/trait.UnwindSafe.html "trait core::panic::unwind_safe::UnwindSafe") for [Upload](struct.Upload.html "struct supabase_storage_rs::models::Upload")<'a>

## Blanket Implementations[§](#blanket-implementations)
