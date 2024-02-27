use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct PackageWebhookEventPayload {
    pub action: PackageWebhookEventAction,
    pub enterprise: Option<serde_json::Value>,
    pub package: serde_json::Value,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum PackageWebhookEventAction {
    Published,
    Updated,
}
