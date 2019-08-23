# LoadReport

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | **String** |  | [optional] 
**broker_version_string** | **String** |  | [optional] 
**web_service_url** | **String** |  | [optional] 
**web_service_url_tls** | **String** |  | [optional] 
**pulsar_service_url** | **String** |  | [optional] 
**pulsar_service_url_tls** | **String** |  | [optional] 
**persistent_topics_enabled** | **bool** |  | [optional] 
**non_persistent_topics_enabled** | **bool** |  | [optional] 
**timestamp** | **i64** |  | [optional] 
**msg_rate_in** | **f64** |  | [optional] 
**msg_rate_out** | **f64** |  | [optional] 
**num_topics** | **i32** |  | [optional] 
**num_consumers** | **i32** |  | [optional] 
**num_producers** | **i32** |  | [optional] 
**num_bundles** | **i32** |  | [optional] 
**system_resource_usage** | [***crate::models::SystemResourceUsage**](SystemResourceUsage.md) |  | [optional] 
**bundle_stats** | [**::std::collections::HashMap<String, crate::models::NamespaceBundleStats>**](NamespaceBundleStats.md) |  | [optional] 
**bundle_gains** | **Vec<String>** |  | [optional] 
**bundle_losses** | **Vec<String>** |  | [optional] 
**allocated_cpu** | **f64** |  | [optional] 
**allocated_memory** | **f64** |  | [optional] 
**allocated_bandwidth_in** | **f64** |  | [optional] 
**allocated_bandwidth_out** | **f64** |  | [optional] 
**allocated_msg_rate_in** | **f64** |  | [optional] 
**allocated_msg_rate_out** | **f64** |  | [optional] 
**pre_allocated_cpu** | **f64** |  | [optional] 
**pre_allocated_memory** | **f64** |  | [optional] 
**pre_allocated_bandwidth_in** | **f64** |  | [optional] 
**pre_allocated_bandwidth_out** | **f64** |  | [optional] 
**pre_allocated_msg_rate_in** | **f64** |  | [optional] 
**pre_allocated_msg_rate_out** | **f64** |  | [optional] 
**over_loaded** | **bool** |  | [optional] 
**load_report_type** | **String** |  | [optional] 
**under_loaded** | **bool** |  | [optional] 
**bandwidth_in** | [***crate::models::ResourceUsage**](ResourceUsage.md) |  | [optional] 
**bandwidth_out** | [***crate::models::ResourceUsage**](ResourceUsage.md) |  | [optional] 
**memory** | [***crate::models::ResourceUsage**](ResourceUsage.md) |  | [optional] 
**last_update** | **i64** |  | [optional] 
**msg_throughput_in** | **f64** |  | [optional] 
**msg_throughput_out** | **f64** |  | [optional] 
**cpu** | [***crate::models::ResourceUsage**](ResourceUsage.md) |  | [optional] 
**direct_memory** | [***crate::models::ResourceUsage**](ResourceUsage.md) |  | [optional] 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


