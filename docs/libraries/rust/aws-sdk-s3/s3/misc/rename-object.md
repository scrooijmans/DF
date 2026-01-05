Renaming objects in directory buckets
PDF
RSS
Focus mode
Using the RenameObject operation, you can atomically rename an existing object in a directory bucket that uses the S3 Express One Zone storage class, without any data movement. You can rename an object by specifying the existing object’s name as the source and the new name of the object as the destination within the same directory bucket. The RenameObject API operation will not succeed on objects that end with the slash (/) delimiter character. For more information, see Naming Amazon S3 objects.

The RenameObject operation is typically completed in milliseconds regardless of the size of the object. This capability accelerates applications like log file management, media processing, and data analytics. Additionally, RenameObject preserves all object metadata properties, including the storage class, encryption type, creation date, last modified date, and checksum properties.

Note
RenameObject is only supported for objects stored in the S3 Express One Zone storage class.

To grant access to the RenameObject operation, we recommend that you use the CreateSession operation for session-based authorization. Specifically, you grant the s3express:CreateSession permission to the directory bucket in a bucket policy or an identity-based policy. Then, you make the CreateSession API call on the directory bucket to obtain a session token. With the session token in your request header, you can make API requests to this operation. After the session token expires, you make another CreateSession API call to generate a new session token for use. The AWS CLI and AWS SDKs will create and manage your session including refreshing the session token automatically to avoid service interruptions when a session expires. For more information about authorization, see CreateSession in the Amazon S3 API Reference. To learn more about Zonal endpoint API operations, see Authorizing Zonal endpoint API operations with CreateSession.

If you don't want to overwrite an existing object, you can add the If-None-Match conditional header with the value ‘\*’ in the RenameObject request. Amazon S3 returns a 412 Precondition Failed error if the object name already exists. For more information, see RenameObject in the Amazon S3 API Reference.

RenameObject is a Zonal endpoint API operation (object-level or data plane operation) that is logged to AWS CloudTrail. You can use CloudTrail to gather information on the RenameObject operation performed on your objects in directory buckets. For more information, see Logging with AWS CloudTrail for directory buckets and CloudTrail log file examples for directory buckets.

S3 Express One Zone is the only storage class that supports RenameObject, which is priced the same as PUT, COPY, POST, and LIST requests (per 1,000 requests) in S3 Express One Zone. For more information, see Amazon S3 pricing.

Renaming an object

To rename an object in your directory bucket, you can use the Amazon S3 console, AWS CLI, AWS SDKs, the REST API or Mountpoint for Amazon S3 (version 1.19.0 or higher).

Using the S3 console
Using the AWS CLI
Using the AWS SDKs

SDK for Java

SDK for Python

SDK for Rust
You can use the SDK for Rust to rename your objects. To use these examples, replace the user input placeholders with your own information.

The following example demonstrates how to rename an object in the amzn-s3-demo-bucket--usw2-az1--x-s3 directory bucket using the SDK for Rust.

async fn basic_rename_example(client: &Client) -> Result<(), Box<dyn Error>> {
let response = client
.rename_object()
.bucket(" amzn-s3-demo-bucket--usw2-az1--x-s3")
.key("new-name.txt") // New name/path for the object
.rename_source("old-name.txt") // Original object name/path
.send()
.await?;
Ok(())
}
This code does the following:

Creates a request to rename an object from "old-name.tx" to "new-name.txt" in the amzn-s3-demo-bucket--usw2-az1--x-s3 directory bucket.

Returns a Result type to handle potential errors.

Using the REST API
Using Mountpoint for Amazon S3
