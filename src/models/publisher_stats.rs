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
pub struct PublisherStats {
    #[serde(rename = "msgRateIn", skip_serializing_if = "Option::is_none")]
    pub msg_rate_in: Option<f64>,
    #[serde(rename = "msgThroughputIn", skip_serializing_if = "Option::is_none")]
    pub msg_throughput_in: Option<f64>,
    #[serde(rename = "averageMsgSize", skip_serializing_if = "Option::is_none")]
    pub average_msg_size: Option<f64>,
    #[serde(rename = "producerId", skip_serializing_if = "Option::is_none")]
    pub producer_id: Option<i64>,
    #[serde(rename = "metadata", skip_serializing_if = "Option::is_none")]
    pub metadata: Option<::std::collections::HashMap<String, String>>,
    #[serde(rename = "connectedSince", skip_serializing_if = "Option::is_none")]
    pub connected_since: Option<String>,
    #[serde(rename = "clientVersion", skip_serializing_if = "Option::is_none")]
    pub client_version: Option<String>,
    #[serde(rename = "producerName", skip_serializing_if = "Option::is_none")]
    pub producer_name: Option<String>,
    #[serde(rename = "address", skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
}

impl PublisherStats {
    pub fn new() -> PublisherStats {
        PublisherStats {
            msg_rate_in: None,
            msg_throughput_in: None,
            average_msg_size: None,
            producer_id: None,
            metadata: None,
            connected_since: None,
            client_version: None,
            producer_name: None,
            address: None,
        }
    }
}


