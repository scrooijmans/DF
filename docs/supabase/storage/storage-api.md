Supabase Storage API
0.0.0
OAS3
api.json
API documentation for Supabase Storage

object
Object end-points

DELETE
/object/{bucketName}/{wildcard}
Delete an object

GET
/object/{bucketName}/{wildcard}
Get object

PUT
/object/{bucketName}/{wildcard}
Update the object at an existing key

POST
/object/{bucketName}/{wildcard}
Upload a new object

DELETE
/object/{bucketName}
Delete multiple objects

GET
/object/authenticated/{bucketName}/{wildcard}
Retrieve an object

POST
/object/upload/sign/{bucketName}/{wildcard}
Generate a presigned url to upload an object

PUT
/object/upload/sign/{bucketName}/{wildcard}
Uploads an object via a presigned URL

POST
/object/sign/{bucketName}/{wildcard}
Generate a presigned url to retrieve an object

GET
/object/sign/{bucketName}/{wildcard}
Retrieve an object via a presigned URL

POST
/object/sign/{bucketName}
Generate presigned urls to retrieve objects

POST
/object/move
Moves an object

POST
/object/list-v2/{bucketName}
Search for objects under a prefix

POST
/object/list/{bucketName}
Search for objects under a prefix

GET
/object/info/authenticated/{bucketName}/{wildcard}
Retrieve object info

GET
/object/info/{bucketName}/{wildcard}
Retrieve object info

POST
/object/copy
Copies an object

GET
/object/public/{bucketName}/{wildcard}
Retrieve an object from a public bucket

GET
/object/info/public/{bucketName}/{wildcard}
Get object info

DELETE
/cdn/{bucketName}/{wildcard}
Purge cache for an object
bucket
Bucket end-points

POST
/bucket/
Create a bucket

GET
/bucket/
Gets all buckets

POST
/bucket/{bucketId}/empty
Empty a bucket

GET
/bucket/{bucketId}
Get details of a bucket

PUT
/bucket/{bucketId}
Update properties of a bucket

DELETE
/bucket/{bucketId}
Delete a bucket
s3
S3 end-points

PUT
/s3/{Bucket}/{wildcard}

POST
/s3/{Bucket}/{wildcard}

DELETE
/s3/{Bucket}/{wildcard}

GET
/s3/{Bucket}/{wildcard}

DELETE
/s3/{Bucket}

PUT
/s3/{Bucket}

POST
/s3/{Bucket}

GET
/s3/{Bucket}

DELETE
/s3/{Bucket}/

PUT
/s3/{Bucket}/

POST
/s3/{Bucket}/

GET
/s3/{Bucket}/

GET
/s3/
transformation
Image transformation

GET
/render/image/authenticated/{bucketName}/{wildcard}
Render an authenticated image with the given transformations

GET
/render/image/sign/{bucketName}/{wildcard}
Render an authenticated image with the given transformations

GET
/render/image/public/{bucketName}/{wildcard}
Render a public image with the given transformations
resumable
Resumable Upload end-points

POST
/upload/resumable/
Handle POST request for TUS Resumable uploads

OPTIONS
/upload/resumable/
Handle OPTIONS request for TUS Resumable uploads

POST
/upload/resumable/{wildcard}
Handle POST request for TUS Resumable uploads

PUT
/upload/resumable/{wildcard}
Handle PUT request for TUS Resumable uploads

PATCH
/upload/resumable/{wildcard}
Handle PATCH request for TUS Resumable uploads

DELETE
/upload/resumable/{wildcard}
Handle DELETE request for TUS Resumable uploads

OPTIONS
/upload/resumable/{wildcard}
Handle OPTIONS request for TUS Resumable uploads

POST
/upload/resumable/sign/
Handle POST request for TUS Resumable uploads

OPTIONS
/upload/resumable/sign/
Handle OPTIONS request for TUS Resumable uploads

POST
/upload/resumable/sign/{wildcard}
Handle POST request for TUS Resumable uploads

PUT
/upload/resumable/sign/{wildcard}
Handle PUT request for TUS Resumable uploads

PATCH
/upload/resumable/sign/{wildcard}
Handle PATCH request for TUS Resumable uploads

DELETE
/upload/resumable/sign/{wildcard}
Handle DELETE request for TUS Resumable uploads

OPTIONS
/upload/resumable/sign/{wildcard}
Handle OPTIONS request for TUS Resumable uploads
default

GET
/metrics

GET
/health/
healthcheck
iceberg

GET
/iceberg/v1/config
Get Iceberg catalog configuration

POST
/iceberg/v1/{prefix}/namespaces
Create a namespace

GET
/iceberg/v1/{prefix}/namespaces
List namespaces

GET
/iceberg/v1/{prefix}/namespaces/{namespace}
Load a namespace

DELETE
/iceberg/v1/{prefix}/namespaces/{namespace}
Create a namespace

POST
/iceberg/v1/{prefix}/namespaces/{namespace}/tables
Create a table in the given namespace

GET
/iceberg/v1/{prefix}/namespaces/{namespace}/tables
Create a namespace

GET
/iceberg/v1/{prefix}/namespaces/{namespace}/tables/{table}
Load an Iceberg Table

POST
/iceberg/v1/{prefix}/namespaces/{namespace}/tables/{table}
Commit updates to multiple tables in an atomic operation

DELETE
/iceberg/v1/{prefix}/namespaces/{namespace}/tables/{table}
Drop a Table
