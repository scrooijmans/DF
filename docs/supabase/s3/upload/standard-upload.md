# Standard Uploads | Supabase Docs

Learn how to upload files to Supabase Storage.

---

---

The standard file upload method is ideal for small files that are not larger than 6MB.

It uses the traditional `multipart/form-data` format and is simple to implement using the supabase-js SDK. Here's an example of how to upload a file using the standard upload method:

```

1
2
3
4
5
6
7
8
9
10
11
12
13
14
import {  } from '@supabase/supabase-js'// Create Supabase clientconst  = ('your_project_url', 'your_supabase_api_key')// Upload file using standard uploadasync function () {  const { ,  } = await ..('bucket_name').('file_path', )  if () {    // Handle error  } else {    // Handle success  }}
```

When uploading a file to a path that already exists, the default behavior is to return a `400 Asset Already Exists` error. If you want to overwrite a file on a specific path you can set the `upsert` options to `true` or using the `x-upsert` header.

```

1
2
3
4
5
6
// Create Supabase clientconst  = ('your_project_url', 'your_supabase_api_key')await ..('bucket_name').('file_path', , {  : true,})
```

We do advise against overwriting files when possible, as our Content Delivery Network will take sometime to propagate the changes to all the edge nodes leading to stale content. Uploading a file to a new path is the recommended way to avoid propagation delays and stale content.

## Content type

By default, Storage will assume the content type of an asset from the file extension. If you want to specify the content type for your asset, pass the `contentType` option during upload.

```

1
2
3
4
5
6
// Create Supabase clientconst  = ('your_project_url', 'your_supabase_api_key')await ..('bucket_name').('file_path', , {  : 'image/jpeg',})
```

When two or more clients upload a file to the same path, the first client to complete the upload will succeed and the other clients will receive a `400 Asset Already Exists` error. If you provide the `x-upsert` header the last client to complete the upload will succeed instead.
