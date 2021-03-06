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
pub struct ResourceQuota {
    #[serde(rename = "msgRateIn", skip_serializing_if = "Option::is_none")]
    pub msg_rate_in: Option<f64>,
    #[serde(rename = "msgRateOut", skip_serializing_if = "Option::is_none")]
    pub msg_rate_out: Option<f64>,
    #[serde(rename = "bandwidthIn", skip_serializing_if = "Option::is_none")]
    pub bandwidth_in: Option<f64>,
    #[serde(rename = "bandwidthOut", skip_serializing_if = "Option::is_none")]
    pub bandwidth_out: Option<f64>,
    #[serde(rename = "memory", skip_serializing_if = "Option::is_none")]
    pub memory: Option<f64>,
    #[serde(rename = "dynamic", skip_serializing_if = "Option::is_none")]
    pub dynamic: Option<bool>,
}

impl ResourceQuota {
    pub fn new() -> ResourceQuota {
        ResourceQuota {
            msg_rate_in: None,
            msg_rate_out: None,
            bandwidth_in: None,
            bandwidth_out: None,
            memory: None,
            dynamic: None,
        }
    }
}


