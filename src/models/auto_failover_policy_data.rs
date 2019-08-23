/*
 * Pulsar Admin REST API
 *
 * This provides the REST API for admin operations
 *
 * The version of the OpenAPI document: v2
 * 
 * Generated by: https://openapi-generator.tech
 */

/// AutoFailoverPolicyData : The auto failover policy configuration data


#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct AutoFailoverPolicyData {
    /// The auto failover policy type
    #[serde(rename = "policy_type", skip_serializing_if = "Option::is_none")]
    pub policy_type: Option<PolicyType>,
    /// The parameters applied to the auto failover policy specified by `policy_type`. The parameters for 'min_available' are :   - 'min_limit': the limit of minimal number of available brokers in primary group before auto failover   - 'usage_threshold': the resource usage threshold. If the usage of a broker is beyond this value, it would be marked as unavailable 
    #[serde(rename = "parameters", skip_serializing_if = "Option::is_none")]
    pub parameters: Option<::std::collections::HashMap<String, String>>,
}

impl AutoFailoverPolicyData {
    /// The auto failover policy configuration data
    pub fn new() -> AutoFailoverPolicyData {
        AutoFailoverPolicyData {
            policy_type: None,
            parameters: None,
        }
    }
}

/// The auto failover policy type
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum PolicyType {
    #[serde(rename = "min_available")]
    MinAvailable,
}

