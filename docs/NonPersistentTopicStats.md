# NonPersistentTopicStats

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**msg_rate_in** | **f64** |  | [optional] 
**msg_throughput_in** | **f64** |  | [optional] 
**msg_rate_out** | **f64** |  | [optional] 
**msg_throughput_out** | **f64** |  | [optional] 
**average_msg_size** | **f64** |  | [optional] 
**storage_size** | **i64** |  | [optional] 
**publishers** | [**Vec<crate::models::NonPersistentPublisherStats>**](NonPersistentPublisherStats.md) |  | [optional] 
**subscriptions** | [**::std::collections::HashMap<String, crate::models::NonPersistentSubscriptionStats>**](NonPersistentSubscriptionStats.md) |  | [optional] 
**replication** | [**::std::collections::HashMap<String, crate::models::NonPersistentReplicatorStats>**](NonPersistentReplicatorStats.md) |  | [optional] 
**deduplication_status** | **String** |  | [optional] 
**msg_drop_rate** | **f64** |  | [optional] 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


