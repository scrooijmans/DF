Title: JavaScript: List all files in a bucket | Supabase Docs

Description: Supabase API reference for JavaScript: List all files in a bucket

# JavaScript: List all files in a bucket

Lists all the files and folders within a path of the bucket.

- RLS policy permissions required:
- `buckets` table permissions: none
- `objects` table permissions: `select`
- Refer to the Storage guide on how access control works

## Parameters

- ### path

(Optional)The folder path.

- ### options

(Optional)Search options including limit (defaults to 100), offset, sortBy, and search

- ### parameters

(Optional)

## Examples

### List files in a bucket

```js
const { data, error } = await supabase.storage.from("avatars").list("folder", {
  limit: 100,
  offset: 0,
  sortBy: { column: "name", order: "asc" },
});
```

### Search files in a bucket

```js
const { data, error } = await supabase.storage.from("avatars").list("folder", {
  limit: 100,
  offset: 0,
  sortBy: { column: "name", order: "asc" },
  search: "jon",
});
```
