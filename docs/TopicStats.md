# TopicStats

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**msg_rate_in** | **f64** |  | [optional] 
**msg_throughput_in** | **f64** |  | [optional] 
**msg_rate_out** | **f64** |  | [optional] 
**msg_throughput_out** | **f64** |  | [optional] 
**average_msg_size** | **f64** |  | [optional] 
**storage_size** | **i64** |  | [optional] 
**publishers** | [**Vec<crate::models::PublisherStats>**](PublisherStats.md) |  | [optional] 
**subscriptions** | [**::std::collections::HashMap<String, crate::models::SubscriptionStats>**](SubscriptionStats.md) |  | [optional] 
**replication** | [**::std::collections::HashMap<String, crate::models::ReplicatorStats>**](ReplicatorStats.md) |  | [optional] 
**deduplication_status** | **String** |  | [optional] 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


