# Error Codes | Supabase Docs

Learn about the Storage error codes and how to resolve them

---

---

Error codes in Storage are returned as part of the response body. They are useful for debugging and understanding what went wrong with your request. The error codes are returned in the following format:

```

1
2
3
4
{  "code": "error_code",  "message": "error_message"}
```

Here is the full list of error codes and their descriptions:

- ErrorCode: NoSuchBucket
  - Description: The specified bucket does not exist.
  - StatusCode: 404
  - Resolution: Verify the bucket name and ensure it exists in the system, if it exists you don't have permissions to access it.
- ErrorCode: NoSuchKey
  - Description: The specified key does not exist.
  - StatusCode: 404
  - Resolution: Check the key name and ensure it exists in the specified bucket, if it exists you don't have permissions to access it.
- ErrorCode: NoSuchUpload
  - Description: The specified upload does not exist.
  - StatusCode: 404
  - Resolution: The upload ID provided might not exists or the Upload was previously aborted
- ErrorCode: InvalidJWT
  - Description: The provided JWT (JSON Web Token) is invalid.
  - StatusCode: 401
  - Resolution: The JWT provided might be expired or malformed, provide a valid JWT
- ErrorCode: InvalidRequest
  - Description: The request is not properly formed.
  - StatusCode: 400
  - Resolution: Review the request parameters and structure, ensure they meet the API's requirements, the error message will provide more details
- ErrorCode: TenantNotFound
  - Description: The specified tenant does not exist.
  - StatusCode: 404
  - Resolution: The Storage service had issues while provisioning, Contact Support
- ErrorCode: EntityTooLarge
  - Description: The entity being uploaded is too large.
  - StatusCode: 413
  - Resolution: Verify the max-file-limit is equal or higher to the resource you are trying to upload, you can change this value on the Project Settings
- ErrorCode: InternalError
  - Description: An internal server error occurred.
  - StatusCode: 500
  - Resolution: Investigate server logs to identify the cause of the internal error. If you think it's a Storage error Contact Support
- ErrorCode: ResourceAlreadyExists
  - Description: The specified resource already exists.
  - StatusCode: 409
  - Resolution: Use a different name or identifier for the resource to avoid conflicts. Use x-upsert:true header to overwrite the resource.
- ErrorCode: InvalidBucketName
  - Description: The specified bucket name is invalid.
  - StatusCode: 400
  - Resolution: Ensure the bucket name follows the naming conventions and does not contain invalid characters.
- ErrorCode: InvalidKey
  - Description: The specified key is invalid.
  - StatusCode: 400
  - Resolution: Verify the key name and ensure it follows the naming conventions.
- ErrorCode: InvalidRange
  - Description: The specified range is not valid.
  - StatusCode: 416
  - Resolution: Make sure that range provided is within the file size boundary and follow the HTTP Range spec
- ErrorCode: InvalidMimeType
  - Description: The specified MIME type is not valid.
  - StatusCode: 400
  - Resolution: Provide a valid MIME type, ensure using the standard MIME type format
- ErrorCode: InvalidUploadId
  - Description: The specified upload ID is invalid.
  - StatusCode: 400
  - Resolution: The upload ID provided is invalid or missing. Make sure to provide a active upload ID
- ErrorCode: KeyAlreadyExists
  - Description: The specified key already exists.
  - StatusCode: 409
  - Resolution: Use a different key name to avoid conflicts with existing keys. Use x-upsert:true header to overwrite the resource.
- ErrorCode: BucketAlreadyExists
  - Description: The specified bucket already exists.
  - StatusCode: 409
  - Resolution: Choose a unique name for the bucket that does not conflict with existing buckets.
- ErrorCode: DatabaseTimeout
  - Description: Timeout occurred while accessing the database.
  - StatusCode: 504
  - Resolution: Investigate database performance and increase the default pool size. If this error still occurs, upgrade your instance
- ErrorCode: InvalidSignature
  - Description: The signature provided does not match the calculated signature.
  - StatusCode: 403
  - Resolution: Check that you are providing the correct signature format, for more information refer to SignatureV4
- ErrorCode: SignatureDoesNotMatch
  - Description: The request signature does not match the calculated signature.
  - StatusCode: 403
  - Resolution: Check your credentials, access key id / access secret key / region that are all correct, refer to S3 Authentication.
- ErrorCode: AccessDenied
  - Description: Access to the specified resource is denied.
  - StatusCode: 403
  - Resolution: Check that you have the correct RLS policy to allow access to this resource
- ErrorCode: ResourceLocked
  - Description: The specified resource is locked.
  - StatusCode: 423
  - Resolution: This resource cannot be altered while there is a lock. Wait and try the request again
- ErrorCode: DatabaseError
  - Description: An error occurred while accessing the database.
  - StatusCode: 500
  - Resolution: Investigate database logs and system configuration to identify and address the database error.
- ErrorCode: MissingContentLength
  - Description: The Content-Length header is missing.
  - StatusCode: 411
  - Resolution: Ensure the Content-Length header is included in the request with the correct value.
- ErrorCode: MissingParameter
  - Description: A required parameter is missing in the request.
  - StatusCode: 400
  - Resolution: Provide all required parameters in the request to fulfill the API's requirements. The message field will contain more details
- ErrorCode: InvalidUploadSignature
  - Description: The provided upload signature is invalid.
  - StatusCode: 403
  - Resolution: The MultiPartUpload record was altered while the upload was ongoing, the signature do not match. Do not alter the upload record
- ErrorCode: LockTimeout
  - Description: Timeout occurred while waiting for a lock.
  - StatusCode: 423
  - Resolution: The lock couldn't be acquired within the specified timeout. Wait and try the request again
- ErrorCode: S3Error
  - Description: An error occurred related to Amazon S3.
  - StatusCode: -
  - Resolution: Refer to Amazon S3 documentation or Contact Support for assistance with resolving the S3 error.
- ErrorCode: S3InvalidAccessKeyId
  - Description: The provided AWS access key ID is invalid.
  - StatusCode: 403
  - Resolution: Verify the AWS access key ID provided and ensure it is correct and active.
- ErrorCode: S3MaximumCredentialsLimit
  - Description: The maximum number of credentials has been reached.
  - StatusCode: 400
  - Resolution: The maximum limit of credentials is reached.
- ErrorCode: InvalidChecksum
  - Description: The checksum of the entity does not match.
  - StatusCode: 400
  - Resolution: Recalculate the checksum of the entity and ensure it matches the one provided in the request.
- ErrorCode: MissingPart
  - Description: A part of the entity is missing.
  - StatusCode: 400
  - Resolution: Ensure all parts of the entity are included in the request before completing the operation.
- ErrorCode: SlowDown
  - Description: The request rate is too high and has been throttled.
  - StatusCode: 503
  - Resolution: Reduce the request rate or implement exponential backoff and retry mechanisms to handle throttling.

As we are transitioning to a new error code system, you might still see the following error format:

```

1
2
3
4
5
{  "httpStatusCode": 400,  "code": "error_code",  "message": "error_message"}
```

Here's a list of the most common error codes and their potential resolutions:

### 404 `not_found`

Indicates that the resource is not found or you don't have the correct permission to access it **Resolution:**

- Add a RLS policy to grant permission to the resource. See our [Access Control docs](https://supabase.com/docs/guides/storage/uploads/access-control) for more information.
- Ensure you include the user `Authorization` header
- Verify the object exists

### 409 `already_exists`

Indicates that the resource already exists. **Resolution:**

- Use the `upsert` functionality in order to overwrite the file. Find out more [here](about:/docs/guides/storage/uploads/standard-uploads#overwriting-files).

You don't have permission to action this request **Resolution:**

- Add RLS policy to grant permission. See our [Access Control docs](https://supabase.com/docs/guides/storage/security/access-control) for more information.
- Ensure you include the user `Authorization` header

### 429 `too many requests`

This problem typically arises when a large number of clients are concurrently interacting with the Storage service, and the pooler has reached its `max_clients` limit.

**Resolution:**

- Increase the max_clients limits of the pooler.
- Upgrade to a bigger project compute instance [here](https://supabase.com/dashboard/project/_/settings/addons).

### 544 `database_timeout`

This problem arises when a high number of clients are concurrently using the Storage service, and Postgres doesn't have enough available connections to efficiently handle requests to Storage.

**Resolution:**

- Increase the pool_size limits of the pooler.
- Upgrade to a bigger project compute instance [here](https://supabase.com/dashboard/project/_/settings/addons).

### 500 `internal_server_error`

This issue occurs where there is a unhandled error. **Resolution:**

- File a support ticket to Storage team [here](https://supabase.com/dashboard/support/new)
