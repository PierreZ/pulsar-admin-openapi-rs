/*
 * Pulsar Admin REST API
 *
 * This provides the REST API for admin operations
 *
 * The version of the OpenAPI document: v2
 * 
 * Generated by: https://openapi-generator.tech
 */



#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct OffloadProcessStatus {
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<Status>,
    #[serde(rename = "lastError", skip_serializing_if = "Option::is_none")]
    pub last_error: Option<String>,
    #[serde(rename = "firstUnoffloadedMessage", skip_serializing_if = "Option::is_none")]
    pub first_unoffloaded_message: Option<crate::models::MessageIdImpl>,
}

impl OffloadProcessStatus {
    pub fn new() -> OffloadProcessStatus {
        OffloadProcessStatus {
            status: None,
            last_error: None,
            first_unoffloaded_message: None,
        }
    }
}

/// 
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum Status {
    #[serde(rename = "NOT_RUN")]
    NOTRUN,
    #[serde(rename = "RUNNING")]
    RUNNING,
    #[serde(rename = "SUCCESS")]
    SUCCESS,
    #[serde(rename = "ERROR")]
    ERROR,
}

