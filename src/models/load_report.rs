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
pub struct LoadReport {
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "brokerVersionString", skip_serializing_if = "Option::is_none")]
    pub broker_version_string: Option<String>,
    #[serde(rename = "webServiceUrl", skip_serializing_if = "Option::is_none")]
    pub web_service_url: Option<String>,
    #[serde(rename = "webServiceUrlTls", skip_serializing_if = "Option::is_none")]
    pub web_service_url_tls: Option<String>,
    #[serde(rename = "pulsarServiceUrl", skip_serializing_if = "Option::is_none")]
    pub pulsar_service_url: Option<String>,
    #[serde(rename = "pulsarServiceUrlTls", skip_serializing_if = "Option::is_none")]
    pub pulsar_service_url_tls: Option<String>,
    #[serde(rename = "persistentTopicsEnabled", skip_serializing_if = "Option::is_none")]
    pub persistent_topics_enabled: Option<bool>,
    #[serde(rename = "nonPersistentTopicsEnabled", skip_serializing_if = "Option::is_none")]
    pub non_persistent_topics_enabled: Option<bool>,
    #[serde(rename = "timestamp", skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<i64>,
    #[serde(rename = "msgRateIn", skip_serializing_if = "Option::is_none")]
    pub msg_rate_in: Option<f64>,
    #[serde(rename = "msgRateOut", skip_serializing_if = "Option::is_none")]
    pub msg_rate_out: Option<f64>,
    #[serde(rename = "numTopics", skip_serializing_if = "Option::is_none")]
    pub num_topics: Option<i32>,
    #[serde(rename = "numConsumers", skip_serializing_if = "Option::is_none")]
    pub num_consumers: Option<i32>,
    #[serde(rename = "numProducers", skip_serializing_if = "Option::is_none")]
    pub num_producers: Option<i32>,
    #[serde(rename = "numBundles", skip_serializing_if = "Option::is_none")]
    pub num_bundles: Option<i32>,
    #[serde(rename = "systemResourceUsage", skip_serializing_if = "Option::is_none")]
    pub system_resource_usage: Option<crate::models::SystemResourceUsage>,
    #[serde(rename = "bundleStats", skip_serializing_if = "Option::is_none")]
    pub bundle_stats: Option<::std::collections::HashMap<String, crate::models::NamespaceBundleStats>>,
    #[serde(rename = "bundleGains", skip_serializing_if = "Option::is_none")]
    pub bundle_gains: Option<Vec<String>>,
    #[serde(rename = "bundleLosses", skip_serializing_if = "Option::is_none")]
    pub bundle_losses: Option<Vec<String>>,
    #[serde(rename = "allocatedCPU", skip_serializing_if = "Option::is_none")]
    pub allocated_cpu: Option<f64>,
    #[serde(rename = "allocatedMemory", skip_serializing_if = "Option::is_none")]
    pub allocated_memory: Option<f64>,
    #[serde(rename = "allocatedBandwidthIn", skip_serializing_if = "Option::is_none")]
    pub allocated_bandwidth_in: Option<f64>,
    #[serde(rename = "allocatedBandwidthOut", skip_serializing_if = "Option::is_none")]
    pub allocated_bandwidth_out: Option<f64>,
    #[serde(rename = "allocatedMsgRateIn", skip_serializing_if = "Option::is_none")]
    pub allocated_msg_rate_in: Option<f64>,
    #[serde(rename = "allocatedMsgRateOut", skip_serializing_if = "Option::is_none")]
    pub allocated_msg_rate_out: Option<f64>,
    #[serde(rename = "preAllocatedCPU", skip_serializing_if = "Option::is_none")]
    pub pre_allocated_cpu: Option<f64>,
    #[serde(rename = "preAllocatedMemory", skip_serializing_if = "Option::is_none")]
    pub pre_allocated_memory: Option<f64>,
    #[serde(rename = "preAllocatedBandwidthIn", skip_serializing_if = "Option::is_none")]
    pub pre_allocated_bandwidth_in: Option<f64>,
    #[serde(rename = "preAllocatedBandwidthOut", skip_serializing_if = "Option::is_none")]
    pub pre_allocated_bandwidth_out: Option<f64>,
    #[serde(rename = "preAllocatedMsgRateIn", skip_serializing_if = "Option::is_none")]
    pub pre_allocated_msg_rate_in: Option<f64>,
    #[serde(rename = "preAllocatedMsgRateOut", skip_serializing_if = "Option::is_none")]
    pub pre_allocated_msg_rate_out: Option<f64>,
    #[serde(rename = "overLoaded", skip_serializing_if = "Option::is_none")]
    pub over_loaded: Option<bool>,
    #[serde(rename = "loadReportType", skip_serializing_if = "Option::is_none")]
    pub load_report_type: Option<String>,
    #[serde(rename = "underLoaded", skip_serializing_if = "Option::is_none")]
    pub under_loaded: Option<bool>,
    #[serde(rename = "bandwidthIn", skip_serializing_if = "Option::is_none")]
    pub bandwidth_in: Option<crate::models::ResourceUsage>,
    #[serde(rename = "bandwidthOut", skip_serializing_if = "Option::is_none")]
    pub bandwidth_out: Option<crate::models::ResourceUsage>,
    #[serde(rename = "memory", skip_serializing_if = "Option::is_none")]
    pub memory: Option<crate::models::ResourceUsage>,
    #[serde(rename = "lastUpdate", skip_serializing_if = "Option::is_none")]
    pub last_update: Option<i64>,
    #[serde(rename = "msgThroughputIn", skip_serializing_if = "Option::is_none")]
    pub msg_throughput_in: Option<f64>,
    #[serde(rename = "msgThroughputOut", skip_serializing_if = "Option::is_none")]
    pub msg_throughput_out: Option<f64>,
    #[serde(rename = "cpu", skip_serializing_if = "Option::is_none")]
    pub cpu: Option<crate::models::ResourceUsage>,
    #[serde(rename = "directMemory", skip_serializing_if = "Option::is_none")]
    pub direct_memory: Option<crate::models::ResourceUsage>,
}

impl LoadReport {
    pub fn new() -> LoadReport {
        LoadReport {
            name: None,
            broker_version_string: None,
            web_service_url: None,
            web_service_url_tls: None,
            pulsar_service_url: None,
            pulsar_service_url_tls: None,
            persistent_topics_enabled: None,
            non_persistent_topics_enabled: None,
            timestamp: None,
            msg_rate_in: None,
            msg_rate_out: None,
            num_topics: None,
            num_consumers: None,
            num_producers: None,
            num_bundles: None,
            system_resource_usage: None,
            bundle_stats: None,
            bundle_gains: None,
            bundle_losses: None,
            allocated_cpu: None,
            allocated_memory: None,
            allocated_bandwidth_in: None,
            allocated_bandwidth_out: None,
            allocated_msg_rate_in: None,
            allocated_msg_rate_out: None,
            pre_allocated_cpu: None,
            pre_allocated_memory: None,
            pre_allocated_bandwidth_in: None,
            pre_allocated_bandwidth_out: None,
            pre_allocated_msg_rate_in: None,
            pre_allocated_msg_rate_out: None,
            over_loaded: None,
            load_report_type: None,
            under_loaded: None,
            bandwidth_in: None,
            bandwidth_out: None,
            memory: None,
            last_update: None,
            msg_throughput_in: None,
            msg_throughput_out: None,
            cpu: None,
            direct_memory: None,
        }
    }
}


