use std::fmt;

use google_cloud_tasks_v2::{
    client::CloudTasks,
    model::{HttpMethod, HttpRequest, Task},
};

#[derive(Clone)]
pub struct CloudTaskProvider {
    client: CloudTasks,
    parent: String,
}

impl fmt::Debug for CloudTaskProvider {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("CloudStorage")
            .field("parent", &self.parent)
            .field(
                "client",
                &format_args!("<gcloud_tasks::Client instance (hidden)>"),
            )
            .finish()
    }
}

impl CloudTaskProvider {
    pub async fn new(project_id: String, location: String, queue_id: String) -> Self {
        let client = CloudTasks::builder()
            .build()
            .await
            .expect("failed to create CloudTasks client");
        let parent = format!(
            "projects/{project}/locations/{location}/queues/{queue}",
            project = project_id,
            location = location,
            queue = queue_id
        );
        Self { client, parent }
    }

    pub async fn send_http_task(
        &self,
        destination_url: String,
        body: serde_json::Value,
    ) -> Result<(), String> {
        let body = serde_json::to_string(&body).unwrap();

        let request = HttpRequest::new()
            .set_url(destination_url)
            .set_http_method(HttpMethod::Post)
            .set_body(body);

        let task = Task::new().set_http_request(request);

        let _ = self
            .client
            .create_task()
            .set_parent(self.parent.clone())
            .set_task(task)
            .send()
            .await
            .map_err(|e| {
                tracing::warn!("Failed to create task: {}", e);
                e.to_string()
            })?;

        Ok(())
    }
}
