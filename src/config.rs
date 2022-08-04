use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
pub struct WolgramConfig {
    pub(crate) api_key: String,
    pub(crate) chat_ids: Vec<i64>,
    pub(crate) devices: HashMap<String, String>,
}

impl Default for WolgramConfig {
    fn default() -> Self {
        Self {
            api_key: "<BOT-API-KEY-GOES-HERE>".to_string(),
            chat_ids: vec![],
            devices: Default::default(),
        }
    }
}
