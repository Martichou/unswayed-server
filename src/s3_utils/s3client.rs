use rusoto_core::Region;
use rusoto_s3::S3;
use rusoto_s3::{DeleteObjectRequest, PutObjectRequest, S3Client};
use std::io::Read;

pub struct Client {
    #[allow(dead_code)]
    region: Region,
    s3: S3Client,
    bucket_name: String,
}

impl Client {
    pub fn new() -> Client {
        let region = Region::Custom {
            name: std::env::var("AWS_REGION").unwrap(),
            endpoint: format!(
                "https://s3.{}.scw.cloud",
                std::env::var("AWS_REGION").unwrap()
            ),
        };

        Client {
            region: region.to_owned(),
            s3: S3Client::new(region),
            bucket_name: std::env::var("AWS_S3_BUCKET_NAME").unwrap(),
        }
    }

    pub fn url(&self, path: &str) -> String {
        format!(
            "https://{}.s3.{}.scw.cloud/{}",
            std::env::var("AWS_S3_BUCKET_NAME").unwrap(),
            std::env::var("AWS_REGION").unwrap(),
            path
        )
    }

    pub async fn put_object(&self, localfilepath: &str, path: &str) -> String {
        let mut file = std::fs::File::open(localfilepath).unwrap();
        let mut contents: Vec<u8> = Vec::new();
        let _ = file.read_to_end(&mut contents);
        let put_request = PutObjectRequest {
            bucket: self.bucket_name.to_owned(),
            key: path.to_owned(),
            body: Some(contents.into()),
            ..Default::default()
        };
        let _res = self
            .s3
            .put_object(put_request)
            .await
            .expect("Failed to put test object");
        self.url(path)
    }

    pub async fn delete_object(&self, path: String) {
        let delete_object_req = DeleteObjectRequest {
            bucket: self.bucket_name.to_owned(),
            key: path.to_owned(),
            ..Default::default()
        };
        let _res = self
            .s3
            .delete_object(delete_object_req)
            .await
            .expect("Couldn't delete object");
    }
}
