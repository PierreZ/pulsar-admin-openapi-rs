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
pub struct NonPersistentSubscriptionStats {
    #[serde(rename = "msgRateOut", skip_serializing_if = "Option::is_none")]
    pub msg_rate_out: Option<f64>,
    #[serde(rename = "msgThroughputOut", skip_serializing_if = "Option::is_none")]
    pub msg_throughput_out: Option<f64>,
    #[serde(rename = "msgRateRedeliver", skip_serializing_if = "Option::is_none")]
    pub msg_rate_redeliver: Option<f64>,
    #[serde(rename = "msgBacklog", skip_serializing_if = "Option::is_none")]
    pub msg_backlog: Option<i64>,
    #[serde(rename = "blockedSubscriptionOnUnackedMsgs", skip_serializing_if = "Option::is_none")]
    pub blocked_subscription_on_unacked_msgs: Option<bool>,
    #[serde(rename = "msgDelayed", skip_serializing_if = "Option::is_none")]
    pub msg_delayed: Option<i64>,
    #[serde(rename = "unackedMessages", skip_serializing_if = "Option::is_none")]
    pub unacked_messages: Option<i64>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub _type: Option<Type>,
    #[serde(rename = "activeConsumerName", skip_serializing_if = "Option::is_none")]
    pub active_consumer_name: Option<String>,
    #[serde(rename = "msgRateExpired", skip_serializing_if = "Option::is_none")]
    pub msg_rate_expired: Option<f64>,
    #[serde(rename = "consumers", skip_serializing_if = "Option::is_none")]
    pub consumers: Option<Vec<crate::models::ConsumerStats>>,
    #[serde(rename = "isReplicated", skip_serializing_if = "Option::is_none")]
    pub is_replicated: Option<bool>,
    #[serde(rename = "msgDropRate", skip_serializing_if = "Option::is_none")]
    pub msg_drop_rate: Option<f64>,
}

impl NonPersistentSubscriptionStats {
    pub fn new() -> NonPersistentSubscriptionStats {
        NonPersistentSubscriptionStats {
            msg_rate_out: None,
            msg_throughput_out: None,
            msg_rate_redeliver: None,
            msg_backlog: None,
            blocked_subscription_on_unacked_msgs: None,
            msg_delayed: None,
            unacked_messages: None,
            _type: None,
            active_consumer_name: None,
            msg_rate_expired: None,
            consumers: None,
            is_replicated: None,
            msg_drop_rate: None,
        }
    }
}

/// 
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "Exclusive")]
    Exclusive,
    #[serde(rename = "Shared")]
    Shared,
    #[serde(rename = "Failover")]
    Failover,
    #[serde(rename = "Key_Shared")]
    KeyShared,
}

