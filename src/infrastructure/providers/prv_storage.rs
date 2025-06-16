use std::fmt;

use gcloud_storage::{
    client::{Client, ClientConfig},
    http::objects::{
        download::Range,
        get::GetObjectRequest,
        upload::{Media, UploadObjectRequest, UploadType},
    },
    sign::SignedURLOptions,
};

#[derive(Clone)]
pub struct CloudStorageProvider {
    bucket_name: String,
    client: Client,
}

impl fmt::Debug for CloudStorageProvider {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("CloudStorage")
            .field("bucket_name", &self.bucket_name)
            .field(
                "client",
                &format_args!("<gcloud_storage::Client instance (hidden)>"),
            )
            .finish()
    }
}

impl CloudStorageProvider {
    pub async fn new(bucket_name: String) -> Self {
        let config = ClientConfig::default().with_auth().await.unwrap();
        let client = Client::new(config);
        Self {
            bucket_name,
            client,
        }
    }

    pub async fn upload_file(
        &self,
        object_name: String,
        file_content: Vec<u8>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let full_object_name = object_name.clone();

        let upload_type = UploadType::Simple(Media::new(full_object_name.clone()));

        let _ = self
            .client
            .upload_object(
                &UploadObjectRequest {
                    bucket: self.bucket_name.clone(),
                    ..Default::default()
                },
                file_content,
                &upload_type,
            )
            .await?;

        Ok(())
    }

    #[allow(dead_code)]
    pub async fn download_file(
        &self,
        object_name: String,
    ) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        let full_object_name = object_name.clone();

        let file_content = self
            .client
            .download_object(
                &GetObjectRequest {
                    bucket: self.bucket_name.clone(),
                    object: full_object_name,
                    ..Default::default()
                },
                &Range::default(),
            )
            .await?;

        Ok(file_content)
    }

    pub async fn get_signed_url(
        &self,
        object_name: String,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let full_object_name = object_name.clone();

        let signed_url = self
            .client
            .signed_url(
                &self.bucket_name,
                &full_object_name,
                None,
                None,
                SignedURLOptions::default(),
            )
            .await?;

        Ok(signed_url)
    }
}
