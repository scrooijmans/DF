# S3 Authentication | Supabase Docs

Learn about authenticating with Supabase Storage S3.

---

---

You have two options to authenticate with Supabase Storage S3:

- Using the generated S3 access keys from your [project settings](https://supabase.com/dashboard/project/_/storage/settings) (Intended exclusively for server-side use)
- Using a Session Token, which will allow you to authenticate with a user JWT token and provide limited access via Row Level Security (RLS).

To authenticate with S3, generate a pair of credentials (Access Key ID and Secret Access Key), copy the endpoint and region from the [project settings page](https://supabase.com/dashboard/project/_/storage/settings).

This is all the information you need to connect to Supabase Storage using any S3-compatible service.

![Storage S3 Access keys](https://supabase.com/docs/img/storage/s3-credentials.png)

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
import { S3Client } from '@aws-sdk/client-s3';const client = new S3Client({  forcePathStyle: true,  region: 'project_region',  endpoint: 'https://project_ref.storage.supabase.co/storage/v1/s3',  credentials: {    accessKeyId: 'your_access_key_id',    secretAccessKey: 'your_secret_access_key',  }})
```

You can authenticate to Supabase S3 with a user JWT token to provide limited access via RLS to all S3 operations. This is useful when you want initialize the S3 client on the server scoped to a specific user, or use the S3 client directly from the client side.

All S3 operations performed with the Session Token are scoped to the authenticated user. RLS policies on the Storage Schema are respected.

To authenticate with S3 using a Session Token, use the following credentials:

- access_key_id: `project_ref`
- secret_access_key: `anonKey`
- session_token: `valid jwt token`

For example, using the `aws-sdk` library:

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
15
16
import { S3Client } from '@aws-sdk/client-s3'const {  data: { session },} = await supabase.auth.getSession()const client = new S3Client({  forcePathStyle: true,  region: 'project_region',  endpoint: 'https://project_ref.storage.supabase.co/storage/v1/s3',  credentials: {    accessKeyId: 'project_ref',    secretAccessKey: 'anonKey',    sessionToken: session.access_token,  },})
```
