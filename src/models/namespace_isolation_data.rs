/*
 * Pulsar Admin REST API
 *
 * This provides the REST API for admin operations
 *
 * The version of the OpenAPI document: v2
 * 
 * Generated by: https://openapi-generator.tech
 */

/// NamespaceIsolationData : The data of namespace isolation configuration


#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct NamespaceIsolationData {
    /// The list of namespaces to apply this namespace isolation data
    #[serde(rename = "namespaces", skip_serializing_if = "Option::is_none")]
    pub namespaces: Option<Vec<String>>,
    /// The list of secondary brokers for serving the list of namespaces in this isolation policy
    #[serde(rename = "primary", skip_serializing_if = "Option::is_none")]
    pub primary: Option<Vec<String>>,
    #[serde(rename = "auto_failover_policy", skip_serializing_if = "Option::is_none")]
    pub auto_failover_policy: Option<crate::models::AutoFailoverPolicyData>,
}

impl NamespaceIsolationData {
    /// The data of namespace isolation configuration
    pub fn new() -> NamespaceIsolationData {
        NamespaceIsolationData {
            namespaces: None,
            primary: None,
            auto_failover_policy: None,
        }
    }
}

