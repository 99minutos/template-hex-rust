#![allow(dead_code)]
use std::{collections::HashMap, str::from_utf8};

use base64::engine::general_purpose::STANDARD as base64_engine;
use base64::Engine;
use serde::{Deserialize, Serialize};
use std::error::Error;

#[derive(Serialize, Deserialize, Debug)]
pub struct PubSubRequest {
    subscription: String,
    message: PubSubMessage,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PubSubMessage {
    attributes: Option<HashMap<String, String>>,
    message_id: String,
    publish_time: String,
    data: String,
}
impl PubSubRequest {
    pub fn decode_data<T: for<'de> Deserialize<'de>>(&self) -> Result<T, Box<dyn Error>> {
        let decoded_bytes = base64_engine.decode(&self.message.data)?;
        let decoded_str = from_utf8(&decoded_bytes)?;
        let decoded_struct: T = serde_json::from_str(decoded_str)?;
        Ok(decoded_struct)
    }
}
