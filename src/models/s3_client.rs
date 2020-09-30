use std::{ env, str::FromStr };
use rusoto_core::{Region, RusotoError};
use rusoto_s3::{ S3Client, DeleteObjectRequest, PutObjectRequest, DeleteObjectError, PutObjectError, S3 };


#[derive(Clone)]
pub struct ConfiguredS3Client {
    region: Region,
    s3: S3Client,
    bucket_name: String,
}

impl ConfiguredS3Client {
    // construct S3 testing client
    pub fn new() -> ConfiguredS3Client {
        if let Ok(bucket_name) = env::var("S3_BUCKET"){
            let region_str = env::var("S3_REGION").expect("Missing S3_REGION in .env");
            let region = Region::from_str(&region_str).expect("failed to parse env var S3_REGION");
            ConfiguredS3Client {
                region: region.to_owned(),
                s3: S3Client::new(region),
                bucket_name: bucket_name.to_owned(),
            }
        }else{
            panic!("ENV var AWS_BUCKET not set");
        }
    }
    pub async fn delete_object(&self, key: String) -> Result<(), RusotoError<DeleteObjectError>> {
        let delete_object_req = DeleteObjectRequest {
            bucket: self.bucket_name.to_owned(),
            key: key.to_owned(),
            ..Default::default()
        };

        self.s3
            .delete_object(delete_object_req)
            .await?;
        Ok(())
    }

    pub async fn put_object(&self, filename: String, contents: Vec<u8>) -> Result<(), RusotoError<PutObjectError>> {
        let put_request = PutObjectRequest {
            bucket: self.bucket_name.to_owned(),
            key: filename.to_owned(),
            body: Some(contents.into()),
            acl: Some("public-read".to_string()),
            ..Default::default()
        };

        self.s3
            .put_object(put_request)
            .await?;
        Ok(())
    }
}
