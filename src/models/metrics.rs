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
pub struct Metrics {
    #[serde(rename = "metrics", skip_serializing_if = "Option::is_none")]
    pub metrics: Option<::std::collections::HashMap<String, serde_json::Value>>,
    #[serde(rename = "dimensions", skip_serializing_if = "Option::is_none")]
    pub dimensions: Option<::std::collections::HashMap<String, String>>,
}

impl Metrics {
    pub fn new() -> Metrics {
        Metrics {
            metrics: None,
            dimensions: None,
        }
    }
}


