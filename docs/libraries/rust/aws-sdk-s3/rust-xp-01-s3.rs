#![allow(unused)] // silence unused warnings while exploring (to comment out)

use std::{error::Error, str};

use s3::bucket::Bucket;
use s3::creds::Credentials;
use s3::region::Region;
use s3::BucketConfiguration;

// Youtube Walkthrough - https://youtu.be/uQKBW8ZgYB8

// region:    Notes

// In cargo.toml add: 
// [dependencies]
// rust-s3 = "0.27.0-rc4"
// tokio = {version = "1.5.0", features = ["full"]}

// To start minio local server:
// docker run --name minio_1 --rm -p 9000:9000 \
//   -e "MINIO_ACCESS_KEY=AKIAIOSFODNN7EXAMPLE" \
//   -e "MINIO_SECRET_KEY=wJalrXUtnFEMIKK7MDENGKKPxRfiCYEXAMPLEKEY" \
//   minio/minio server /data

// endregion: Notes


#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
	// 1) Instantiate the bucket client
	let bucket = Bucket::new_with_path_style(
		"rust-s3",
		Region::Custom {
			region: "".to_owned(),
			endpoint: "http://127.0.0.1:9000".to_owned(),
		},
		Credentials {
			access_key: Some("AKIAIOSFODNN7EXAMPLE".to_owned()),
			secret_key: Some("wJalrXUtnFEMIKK7MDENGKKPxRfiCYEXAMPLEKEY".to_owned()),
			security_token: None,
			session_token: None,
		},
	)?;

	// 2) Create bucket if does not exist
	let (_, code) = bucket.head_object("/").await?;
	if code == 404 {
		let create_result = Bucket::create_with_path_style(
			bucket.name.as_str(),
			bucket.region.clone(),
			bucket.credentials.clone(),
			BucketConfiguration::default(),
		)
		.await?;

		println!(
			"=== Bucket created\n{} - {} - {}",
			bucket.name, create_result.response_code, create_result.response_text
		);
	}

	// 3) Create object (text/plain)
	let key = "test_file_2";
	println!("=== Put content");
	bucket
		.put_object_with_content_type(key, "NEW !!! Stuff!!!".as_bytes(), "text/plain")
		.await?;

	// 4) List bucket content
	println!("=== List bucket content");
	let results = bucket.list("/".to_owned(), Some("/".to_owned())).await?;
	for result in results {
		for item in result.contents {
			println!("key: {}", item.key);
		}
	}

	// 5) Get object content back
	println!("=== Get content");
	let (data, _) = bucket.get_object(key).await?;
	let data = str::from_utf8(&data).expect("Wrong data!!!");
	println!("data: {}", data);

	Ok(())
}