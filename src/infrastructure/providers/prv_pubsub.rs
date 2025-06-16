use std::fmt;

use google_cloud_googleapis::pubsub::v1::PubsubMessage;
use google_cloud_pubsub::client::{Client, ClientConfig};

#[derive(Clone)]
pub struct PubsubProvider {
    client: Client,
}

impl fmt::Debug for PubsubProvider {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("PubsubProvider")
            .field(
                "client",
                &format_args!("<google_cloud_pubsub::client::Client instance (hidden)>"),
            )
            .finish()
    }
}

impl PubsubProvider {
    pub async fn new() -> Self {
        let config = ClientConfig::default().with_auth().await.unwrap();
        let client = Client::new(config).await.unwrap();
        Self { client }
    }

    pub async fn publish(&self, topic_id: String, message: PubsubMessage) -> Result<(), String> {
        tracing::debug!("Attempting to publish to topic_id: {}", topic_id);

        let topic_ref = self.client.topic(topic_id.as_str());

        match topic_ref.exists(None).await {
            Ok(exists) => {
                if !exists {
                    tracing::info!("Topic '{}' does not exist. Attempting to create.", topic_id);
                    match topic_ref.create(None, None).await {
                        Ok(_) => {
                            tracing::info!("Topic '{}' created successfully.", topic_id);
                        }
                        Err(e) => {
                            tracing::error!("Failed to create topic '{}': {}", topic_id, e);
                            return Err(format!("Failed to create topic '{}': {}", topic_id, e));
                        }
                    }
                } else {
                    tracing::debug!("Topic '{}' already exists.", topic_id);
                }
            }
            Err(e) => {
                tracing::error!("Failed to check existence of topic '{}': {}", topic_id, e);
                return Err(format!(
                    "Failed to check existence of topic '{}': {}",
                    topic_id, e
                ));
            }
        }

        let mut publisher = topic_ref.new_publisher(None);

        match publisher.publish(message).await.get().await {
            Ok(_) => {
                publisher.shutdown().await;
                Ok(())
            }
            Err(e) => {
                tracing::error!("Failed to publish message to topic '{}': {}", topic_id, e);
                publisher.shutdown().await;
                Err(format!(
                    "Failed to publish message to topic '{}': {}",
                    topic_id, e
                ))
            }
        }
    }
}
