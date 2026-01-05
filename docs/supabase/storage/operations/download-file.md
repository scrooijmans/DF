Title: JavaScript: Download a file | Supabase Docs

Description: Supabase API reference for JavaScript: Download a file

# JavaScript: Download a file

Downloads a file from a private bucket. For public buckets, make a request to the URL returned from `getPublicUrl` instead.

- RLS policy permissions required:
- `buckets` table permissions: none
- `objects` table permissions: `select`
- Refer to the Storage guide on how access control works

## Parameters

- ### path

(Required)The full path and file name of the file to be downloaded. For example \`folder/image.png\`.

- ### options

(Optional)

## Examples

### Download file

```js
const { data, error } = await supabase.storage
  .from("avatars")
  .download("folder/avatar1.png");
```

### Download file with transformations

```js
const { data, error } = await supabase.storage
  .from("avatars")
  .download("folder/avatar1.png", {
    transform: {
      width: 100,
      height: 100,
      quality: 80,
    },
  });
```
