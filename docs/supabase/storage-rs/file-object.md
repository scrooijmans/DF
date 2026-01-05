# FileObject in supabase_storage_rs::models - Rust

```
pub struct FileObject {
    pub name: String,
    pub id: String,
    pub updated_at: String,
    pub created_at: String,
    pub last_accessed_at: String,
    pub metadata: Metadata,
    pub bucket_id: Option<String>,
    pub owner: Option<String>,
    pub buckets: Option<Value>,
}
```

## Fields[§](#fields)

[§](#structfield.name)`name: [String](https://doc.rust-lang.org/nightly/alloc/string/struct.String.html "struct alloc::string::String")`[§](#structfield.id)`id: [String](https://doc.rust-lang.org/nightly/alloc/string/struct.String.html "struct alloc::string::String")`[§](#structfield.updated_at)`updated_at: [String](https://doc.rust-lang.org/nightly/alloc/string/struct.String.html "struct alloc::string::String")`[§](#structfield.created_at)`created_at: [String](https://doc.rust-lang.org/nightly/alloc/string/struct.String.html "struct alloc::string::String")`[§](#structfield.last_accessed_at)`last_accessed_at: [String](https://doc.rust-lang.org/nightly/alloc/string/struct.String.html "struct alloc::string::String")`[§](#structfield.metadata)`metadata: [Metadata](struct.Metadata.html "struct supabase_storage_rs::models::Metadata")`[§](#structfield.bucket_id)`bucket_id: [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[String](https://doc.rust-lang.org/nightly/alloc/string/struct.String.html "struct alloc::string::String")>`[§](#structfield.owner)`owner: [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[String](https://doc.rust-lang.org/nightly/alloc/string/struct.String.html "struct alloc::string::String")>`[§](#structfield.buckets)`buckets: [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[Value](https://docs.rs/serde_json/1.0.133/x86_64-unknown-linux-gnu/serde_json/value/enum.Value.html "enum serde_json::value::Value")>`

## Trait Implementations[§](#trait-implementations)

## Auto Trait Implementations[§](#synthetic-implementations)

[§](#impl-Freeze-for-FileObject)

### impl [Freeze](https://doc.rust-lang.org/nightly/core/marker/trait.Freeze.html "trait core::marker::Freeze") for [FileObject](struct.FileObject.html "struct supabase_storage_rs::models::FileObject")

[§](#impl-RefUnwindSafe-for-FileObject)

### impl [RefUnwindSafe](https://doc.rust-lang.org/nightly/core/panic/unwind_safe/trait.RefUnwindSafe.html "trait core::panic::unwind_safe::RefUnwindSafe") for [FileObject](struct.FileObject.html "struct supabase_storage_rs::models::FileObject")

[§](#impl-Send-for-FileObject)

### impl [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") for [FileObject](struct.FileObject.html "struct supabase_storage_rs::models::FileObject")

[§](#impl-Sync-for-FileObject)

### impl [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") for [FileObject](struct.FileObject.html "struct supabase_storage_rs::models::FileObject")

[§](#impl-Unpin-for-FileObject)

### impl [Unpin](https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html "trait core::marker::Unpin") for [FileObject](struct.FileObject.html "struct supabase_storage_rs::models::FileObject")

[§](#impl-UnwindSafe-for-FileObject)

### impl [UnwindSafe](https://doc.rust-lang.org/nightly/core/panic/unwind_safe/trait.UnwindSafe.html "trait core::panic::unwind_safe::UnwindSafe") for [FileObject](struct.FileObject.html "struct supabase_storage_rs::models::FileObject")

## Blanket Implementations[§](#blanket-implementations)
