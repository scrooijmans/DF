Title: Self-Hosting | Supabase Docs

Description: Supabase is the Postgres development platform providing all the backend features you need to build a product.

Storage Server Reference

# Self-Hosting Storage

An S3 compatible object storage service that integrates with Postgres.

- Uses Postgres as it's datastore for storing metadata
- Authorization rules are written as Postgres Row Level Security policies
- Integrates with S3 as the storage backend (with more in the pipeline!)
- Extremely lightweight and performant

### Client libraries#

- JavaScript
- Dart

### Additional links#

- Source code
- Known bugs and issues
- Storage guides
- OpenAPI docs
- Why we built a new object storage service

## Create a bucket

post`/bucket/`

### Body

- nameRequiredstring

- idOptionalstring

- publicOptionalboolean

- typeOptionalenum

Accepted values

- file_size_limitOptionalany of the following options

Options

- allowed_mime_typesOptionalArray<string>

### Response codes

- 200
- 4XX

### Response (200)

exampleschema

```
123{  "name": "avatars"}
```

## Gets all buckets

get`/bucket/`

### Query parameters

- limitOptionalinteger

- offsetOptionalinteger

- sortColumnOptionalenum

Accepted values

- sortOrderOptionalenum

Accepted values

- searchOptionalstring

### Response codes

- 200
- 4XX

### Response (200)

exampleschema

```
123456789101112131415[  {    "id": "bucket2",    "name": "bucket2",    "public": false,    "file_size_limit": 1000000,    "allowed_mime_types": [      "image/png",      "image/jpeg"    ],    "owner": "4d56e902-f0a0-4662-8448-a4d9e643c142",    "created_at": "2021-02-17T04:43:32.770206+00:00",    "updated_at": "2021-02-17T04:43:32.770206+00:00"  }]
```

## Empty a bucket

post`/bucket/{bucketId}/empty`

### Path parameters

- bucketIdRequiredstring

### Response codes

- 200
- 4XX

### Response (200)

exampleschema

```
123{  "message": "Empty bucket has been queued. Completion may take up to an hour."}
```

## Get details of a bucket

get`/bucket/{bucketId}`

### Path parameters

- bucketIdRequiredstring

### Response codes

- 200
- 4XX

### Response (200)

exampleschema

```
12345678910{  "id": "lorem",  "name": "lorem",  "owner": "lorem",  "owner_id": "lorem",  "public": true,  "type": "STANDARD",  "created_at": "lorem",  "updated_at": "lorem"}
```

## Update properties of a bucket

put`/bucket/{bucketId}`

### Body

- publicOptionalboolean

- file_size_limitOptionalany of the following options

Options

- allowed_mime_typesOptionalArray<string>

### Response codes

- 200
- 4XX

### Response (200)

exampleschema

```
123{  "message": "Successfully updated"}
```

## Delete a bucket

delete`/bucket/{bucketId}`

### Path parameters

- bucketIdRequiredstring

### Response codes

- 200
- 4XX

### Response (200)

exampleschema

```
123{  "message": "Successfully deleted"}
```

## Delete an object

delete`/object/{bucketName}/{wildcard}`

### Path parameters

- bucketNameRequiredstring

- \*Requiredstring

### Response codes

- 200
- 4XX

### Response (200)

exampleschema

```
123{  "message": "Successfully deleted"}
```

## Get object

get`/object/{bucketName}/{wildcard}`

Serve objects

### Path parameters

- bucketNameRequiredstring

- \*Requiredstring

### Response codes

- 4XX

## Update the object at an existing key

put`/object/{bucketName}/{wildcard}`

### Path parameters

- bucketNameRequiredstring

- \*Requiredstring

### Response codes

- 200
- 4XX

### Response (200)

exampleschema

```
1234{  "Id": "lorem",  "Key": "avatars/folder/cat.png"}
```

## Upload a new object

post`/object/{bucketName}/{wildcard}`

### Path parameters

- bucketNameRequiredstring

- \*Requiredstring

### Response codes

- 200
- 4XX

### Response (200)

exampleschema

```
1234{  "Id": "lorem",  "Key": "avatars/folder/cat.png"}
```

## Delete multiple objects

delete`/object/{bucketName}`

### Path parameters

- bucketNameRequiredstring

### Body

- prefixesRequiredArray<string>

### Response codes

- 200
- 4XX

### Response (200)

exampleschema

```
1234567891011121314[  {    "name": "folder/cat.png",    "bucket_id": "avatars",    "owner": "317eadce-631a-4429-a0bb-f19a7a517b4a",    "id": "eaa8bdb5-2e00-4767-b5a9-d2502efe2196",    "updated_at": "2021-04-06T16:30:35.394674+00:00",    "created_at": "2021-04-06T16:30:35.394674+00:00",    "last_accessed_at": "2021-04-06T16:30:35.394674+00:00",    "metadata": {      "size": 1234    }  }]
```

## Retrieve an object

get`/object/authenticated/{bucketName}/{wildcard}`

### Path parameters

- bucketNameRequiredstring

- \*Requiredstring

### Response codes

- 4XX

## Generate a presigned url to retrieve an object

post`/object/sign/{bucketName}/{wildcard}`

### Path parameters

- bucketNameRequiredstring

- \*Requiredstring

### Body

- expiresInRequiredinteger

- transformOptionalobject

Object schema

### Response codes

- 200
- 4XX

### Response (200)

exampleschema

```
123{  "signedURL": "/object/sign/avatars/folder/cat.png?token=eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJ1cmwiOiJhdmF0YXJzL2ZvbGRlci9jYXQucG5nIiwiaWF0IjoxNjE3NzI2MjczLCJleHAiOjE2MTc3MjcyNzN9.s7Gt8ME80iREVxPhH01ZNv8oUn4XtaWsmiQ5csiUHn4"}
```

## Retrieve an object via a presigned URL

get`/object/sign/{bucketName}/{wildcard}`

### Path parameters

- bucketNameRequiredstring

- \*Requiredstring

### Query parameters

- downloadOptionalstring

- tokenRequiredstring

### Response codes

- 4XX

## Generate presigned urls to retrieve objects

post`/object/sign/{bucketName}`

### Path parameters

- bucketNameRequiredstring

### Body

- expiresInRequiredinteger

- pathsRequiredArray<string>

### Response codes

- 200
- 4XX

### Response (200)

exampleschema

```
1234567[  {    "error": "Either the object does not exist or you do not have access to it",    "path": "folder/cat.png",    "signedURL": "/object/sign/avatars/folder/cat.png?token=eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJ1cmwiOiJhdmF0YXJzL2ZvbGRlci9jYXQucG5nIiwiaWF0IjoxNjE3NzI2MjczLCJleHAiOjE2MTc3MjcyNzN9.s7Gt8ME80iREVxPhH01ZNv8oUn4XtaWsmiQ5csiUHn4"  }]
```

## Moves an object

post`/object/move`

### Body

- bucketIdRequiredstring

- sourceKeyRequiredstring

- destinationBucketOptionalstring

- destinationKeyRequiredstring

### Response codes

- 200
- 4XX

### Response (200)

exampleschema

```
123{  "message": "Successfully moved"}
```

## Search for objects under a prefix

post`/object/list/{bucketName}`

### Path parameters

- bucketNameRequiredstring

### Body

- prefixRequiredstring

- limitOptionalinteger

- offsetOptionalinteger

- sortByOptionalobject

Object schema

- searchOptionalstring

### Response codes

- 200
- 4XX

### Response (200)

exampleschema

```
1234567891011121314[  {    "name": "folder/cat.png",    "bucket_id": "avatars",    "owner": "317eadce-631a-4429-a0bb-f19a7a517b4a",    "id": "eaa8bdb5-2e00-4767-b5a9-d2502efe2196",    "updated_at": "2021-04-06T16:30:35.394674+00:00",    "created_at": "2021-04-06T16:30:35.394674+00:00",    "last_accessed_at": "2021-04-06T16:30:35.394674+00:00",    "metadata": {      "size": 1234    }  }]
```

## Retrieve object info

get`/object/info/{bucketName}/{wildcard}`

Object Info

### Path parameters

- bucketNameRequiredstring

- \*Requiredstring

### Response codes

- 4XX

## Copies an object

post`/object/copy`

### Body

- bucketIdRequiredstring

- sourceKeyRequiredstring

- destinationBucketOptionalstring

- destinationKeyRequiredstring

- metadataOptionalobject

Object schema

- copyMetadataOptionalboolean

### Response codes

- 200
- 4XX

### Response (200)

exampleschema

```
12345678910111213141516171819202122232425262728{  "Id": "lorem",  "Key": "folder/destination.png",  "name": "lorem",  "bucket_id": "lorem",  "owner": "lorem",  "owner_id": "lorem",  "version": "lorem",  "id": "lorem",  "updated_at": "lorem",  "created_at": "lorem",  "last_accessed_at": "lorem",  "metadata": {},  "user_metadata": {},  "buckets": {    "id": "bucket2",    "name": "bucket2",    "public": false,    "file_size_limit": 1000000,    "allowed_mime_types": [      "image/png",      "image/jpeg"    ],    "owner": "4d56e902-f0a0-4662-8448-a4d9e643c142",    "created_at": "2021-02-17T04:43:32.770206+00:00",    "updated_at": "2021-02-17T04:43:32.770206+00:00"  }}
```

## Retrieve an object from a public bucket

get`/object/public/{bucketName}/{wildcard}`

### Path parameters

- bucketNameRequiredstring

- \*Requiredstring

### Response codes

- 4XX

## Get object info

get`/object/info/public/{bucketName}/{wildcard}`

returns object info

### Path parameters

- bucketNameRequiredstring

- \*Requiredstring

### Response codes

- 4XX
