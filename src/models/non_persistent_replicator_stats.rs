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
pub struct NonPersistentReplicatorStats {
    #[serde(rename = "msgRateIn", skip_serializing_if = "Option::is_none")]
    pub msg_rate_in: Option<f64>,
    #[serde(rename = "msgThroughputIn", skip_serializing_if = "Option::is_none")]
    pub msg_throughput_in: Option<f64>,
    #[serde(rename = "msgRateOut", skip_serializing_if = "Option::is_none")]
    pub msg_rate_out: Option<f64>,
    #[serde(rename = "msgThroughputOut", skip_serializing_if = "Option::is_none")]
    pub msg_throughput_out: Option<f64>,
    #[serde(rename = "msgRateExpired", skip_serializing_if = "Option::is_none")]
    pub msg_rate_expired: Option<f64>,
    #[serde(rename = "replicationBacklog", skip_serializing_if = "Option::is_none")]
    pub replication_backlog: Option<i64>,
    #[serde(rename = "connected", skip_serializing_if = "Option::is_none")]
    pub connected: Option<bool>,
    #[serde(rename = "replicationDelayInSeconds", skip_serializing_if = "Option::is_none")]
    pub replication_delay_in_seconds: Option<i64>,
    #[serde(rename = "inboundConnection", skip_serializing_if = "Option::is_none")]
    pub inbound_connection: Option<String>,
    #[serde(rename = "inboundConnectedSince", skip_serializing_if = "Option::is_none")]
    pub inbound_connected_since: Option<String>,
    #[serde(rename = "outboundConnection", skip_serializing_if = "Option::is_none")]
    pub outbound_connection: Option<String>,
    #[serde(rename = "outboundConnectedSince", skip_serializing_if = "Option::is_none")]
    pub outbound_connected_since: Option<String>,
    #[serde(rename = "msgDropRate", skip_serializing_if = "Option::is_none")]
    pub msg_drop_rate: Option<f64>,
}

impl NonPersistentReplicatorStats {
    pub fn new() -> NonPersistentReplicatorStats {
        NonPersistentReplicatorStats {
            msg_rate_in: None,
            msg_throughput_in: None,
            msg_rate_out: None,
            msg_throughput_out: None,
            msg_rate_expired: None,
            replication_backlog: None,
            connected: None,
            replication_delay_in_seconds: None,
            inbound_connection: None,
            inbound_connected_since: None,
            outbound_connection: None,
            outbound_connected_since: None,
            msg_drop_rate: None,
        }
    }
}


