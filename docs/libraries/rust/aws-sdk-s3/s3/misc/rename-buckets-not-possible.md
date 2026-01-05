Title: How to Rename an AWS S3 Bucket in Terraform - Step by Step

Description: Learn how to rename an AWS S3 bucket in Terraform. See renaming process of Amazon s3 bucket in a few simple steps.

Terraform

# How to Rename an AWS S3 Bucket in Terraform – Step by Step

Jacob Martin

Updated 02 Oct 2025·3 min read

## 🚀 Level Up Your Infrastructure Skills

You focus on building. We’ll keep you updated. Get curated infrastructure insights that help you make smarter decisions.

Occasionally you might want to rename an AWS S3 bucket you are managing with Terraform. However, names of S3 buckets are immutable, which means you can’t change them directly. If you tried, Terraform would destroy the old one and then create a new one, resulting in data loss.

To avoid this, you need to **create a new bucket with the desired name**, move the data over to it, make the relevant Terraform state replacements, and finally delete the old bucket.

In this post you will learn how to rename an AWS S3 bucket in Terraform. First things first – let’s say you have a bucket definition in your Terraform code:

```hcl
resource “aws_s3_bucket” “my_bucket” {
bucket = “old-name”
}
```

and you want to change the name of the bucket to `new-name`.

## Step 1 - Create the new bucket

Firstly, we’ll need to create a new bucket. You can do this using the AWS CLI or the AWS console. Just make sure to properly replicate the old settings, especially the ACL (so that your data doesn’t accidentally become public).

Now, we can copy all the files from the old to the new bucket:

```bash
aws s3 sync s3://old-name s3://new-name
```

## Step 2 - Modify the State

Now that we have our new bucket, we need to remove the old one from our Terraform state and import the new one in its place:

```bash
terraform state rm aws_s3_bucket.my_bucket
terraform import aws_s3_bucket.my_bucket new-name
```

If you tried to run Terraform now, it would show you that there’s drift—and yes, there would be! We’ve just imported a bucket into a resource which still has `old-name` in the config.

## Step 3 - Change the Code

That’s why we now have to finally change the name of the bucket in our Terraform config:

```hcl
resource “aws_s3_bucket” “my_bucket” {
bucket = “new-name”
}
```

If you run Terraform now, you’ll see that there are no changes to be made.

💡 You might also like:

- 20 Best Practices for Using Terraform
- How to Automate Terraform Deployments and Infrastructure Provisioning
- How to Improve Your Infrastructure as Code using Terraform

## Step 4 - Do a Cleanup

If you want to, you can now delete your old bucket using:

```bash
aws s3 rm s3://old-name --recursive
aws s3 rb s3://old-name
```

Make sure all the data has successfully been copied over to the new bucket.

## Step 5 - Run the Above Arbitrary AWS CLI Commands if You’re Using Spacelift

If you’re using Spacelift then you can use tasks to run the above arbitrary AWS CLI commands. The default runner image already contains the AWS CLI, so no changes are necessary there.

One additional thing you can do is lock the Stack while you’re running the migration, this way nobody will accidentally run Terraform or make any other changes while you’re moving the data.

And that’s it! If you have any questions about how to rename an AWS S3 bucket, drop me a line in the comments and I’ll get back to you straight away.

**Note:** New versions of Terraform are placed under the BUSL license, but everything created before version 1.5.x stays open-source. OpenTofu is an open-source version of Terraform that expands on Terraform’s existing concepts and offerings. It is a viable alternative to HashiCorp’s Terraform, being forked from Terraform version 1.5.6.

### Manage Terraform Better and Faster

If you are struggling with Terraform automation and management, check out Spacelift. It helps you manage Terraform state, build more complex workflows, and adds several must-have capabilities for end-to-end infrastructure management.

Start free trial

## Frequently asked questions

- ### How do I rename the S3 bucket name?

S3 bucket names cannot be renamed directly. To rename one, create a new bucket with the desired name, copy all data using aws s3 sync, update references in your code or services, then delete the original bucket if no longer needed. Ensure the new name is globally unique.

- ### How do I name an S3 bucket?

S3 bucket names must be globally unique, 3–63 characters long, and use only lowercase letters, numbers, hyphens, and periods. Names must start and end with a letter or number and avoid patterns like IP addresses. A common naming convention is project-name-region-environment.

- ### Can I rename files in S3?

S3 doesn’t support renaming files directly. To rename a file, you need to copy the object to the new key (name), then delete the original:

`aws s3 cp s3://bucket/old-name.txt s3://bucket/new-name.txt`

`aws s3 rm s3://bucket/old-name.txt`

This applies because S3 uses a flat key-based storage system, not a traditional filesystem.

### Written by

### Jacob Martin

Jacob is a Software Engineering Team Lead at Spacelift with experience working in high scale distributed systems environments: building storage systems, service meshes, observability infrastructure, and developer experience workflows. He is Certified in Deep Learning Specialization. He has got a practical mindset of shipping, and building projects in as simple a way - but not simpler - as possible.

### Read also

Terraform27 min read

##### Terraform Backends – Local and Remote Explained

AWS14 min read

##### Bootstrap Complete Amazon EKS Clusters with EKS Blueprints for Terraform

Terraform9 min read

##### Terraform Taint, Untaint, Replace – How to Use It (Examples)

© 2025 Spacelift, Inc. All rights reserved

Terraform Commands Cheat Sheet

Grab our ultimate cheat sheet PDF  
for all the Terraform commands  
and concepts you need.

Download now

###### Share your data and download the cheat sheet
