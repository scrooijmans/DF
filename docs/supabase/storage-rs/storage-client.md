# StorageClient in supabase_storage_rs::models - Rust
Struct StorageClient
--------------------

[Source](about:blank/src/supabase_storage_rs/models.rs.html#7-14)

```
pub struct StorageClient {
    pub client: Client,
    pub project_url: String,
    pub api_key: String,
}
```


Expand description

Supabase Storage Client

REST endpoint for querying and managing your database Example: https://.supabase.co

WARN: The `service role` key has the ability to bypass Row Level Security. Never share it publicly.

[Source](about:blank/src/supabase_storage_rs/client.rs.html#17-746)
[§](#impl-StorageClient)

[Source](about:blank/src/supabase_storage_rs/client.rs.html#19-25)

Create a new StorageClient from a project\_url and api\_key

[Source](about:blank/src/supabase_storage_rs/client.rs.html#29-38)

Create a new StorageClient from the “SUPABASE\_URL” and “SUPABASE\_API\_KEY” environment variables.

[Source](about:blank/src/supabase_storage_rs/client.rs.html#46-94)

Create a new storage bucket, returning the name **_(not the id)_** of the bucket on success.

Requires your StorageClient to have the following RLS permissions: `buckets` table permissions: insert

WARNING: Do not use underscores in bucket names or ids

[Source](about:blank/src/supabase_storage_rs/client.rs.html#97-122)

Delete the bucket with the given id

[Source](about:blank/src/supabase_storage_rs/client.rs.html#132-159)

Get the bucket with the given id

##### [§](#example)Example

```
let bucket = client
    .get_bucket("a-cool-name-for-a-bucket-with-options")
    .await
    .unwrap();
```


[Source](about:blank/src/supabase_storage_rs/client.rs.html#166-190)

Retrieves the details of all Storage buckets within an existing project

##### [§](#example-1)Example

```
let buckets = client.list_buckets().await.unwrap();
```


[Source](about:blank/src/supabase_storage_rs/client.rs.html#196-242)

Updates a Storage bucket

Requires the following RLS permissions: `buckets` table: `select` and `update`

[Source](about:blank/src/supabase_storage_rs/client.rs.html#245-273)

Empty a bucket

[Source](about:blank/src/supabase_storage_rs/client.rs.html#350-359)

[Source](about:blank/src/supabase_storage_rs/client.rs.html#361-370)

[Source](about:blank/src/supabase_storage_rs/client.rs.html#372-381)

[Source](about:blank/src/supabase_storage_rs/client.rs.html#383-423)

[Source](about:blank/src/supabase_storage_rs/client.rs.html#425-452)

[Source](about:blank/src/supabase_storage_rs/client.rs.html#454-499)

[Source](about:blank/src/supabase_storage_rs/client.rs.html#501-544)

[Source](about:blank/src/supabase_storage_rs/client.rs.html#546-584)

[Source](about:blank/src/supabase_storage_rs/client.rs.html#586-629)

[Source](about:blank/src/supabase_storage_rs/client.rs.html#631-662)

[Source](about:blank/src/supabase_storage_rs/client.rs.html#664-723)

[Source](about:blank/src/supabase_storage_rs/client.rs.html#725-745)

[§](#impl-Freeze-for-StorageClient)

[§](#impl-RefUnwindSafe-for-StorageClient)

[§](#impl-Send-for-StorageClient)

[§](#impl-Sync-for-StorageClient)

[§](#impl-Unpin-for-StorageClient)

[§](#impl-UnwindSafe-for-StorageClient)