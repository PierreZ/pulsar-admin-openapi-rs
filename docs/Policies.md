# Policies

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**auth_policies** | [***crate::models::AuthPolicies**](AuthPolicies.md) |  | [optional] 
**replication_clusters** | **Vec<String>** |  | [optional] 
**bundles** | [***crate::models::BundlesData**](BundlesData.md) |  | [optional] 
**backlog_quota_map** | [**::std::collections::HashMap<String, crate::models::BacklogQuota>**](BacklogQuota.md) |  | [optional] 
**topic_dispatch_rate** | [**::std::collections::HashMap<String, crate::models::DispatchRate>**](DispatchRate.md) |  | [optional] 
**subscription_dispatch_rate** | [**::std::collections::HashMap<String, crate::models::DispatchRate>**](DispatchRate.md) |  | [optional] 
**replicator_dispatch_rate** | [**::std::collections::HashMap<String, crate::models::DispatchRate>**](DispatchRate.md) |  | [optional] 
**cluster_subscribe_rate** | [**::std::collections::HashMap<String, crate::models::SubscribeRate>**](SubscribeRate.md) |  | [optional] 
**persistence** | [***crate::models::PersistencePolicies**](PersistencePolicies.md) |  | [optional] 
**deduplication_enabled** | **bool** |  | [optional] 
**latency_stats_sample_rate** | **::std::collections::HashMap<String, i32>** |  | [optional] 
**message_ttl_in_seconds** | **i32** |  | [optional] 
**retention_policies** | [***crate::models::RetentionPolicies**](RetentionPolicies.md) |  | [optional] 
**deleted** | **bool** |  | [optional] 
**anti_affinity_group** | **String** |  | [optional] 
**encryption_required** | **bool** |  | [optional] 
**subscription_auth_mode** | **String** |  | [optional] 
**max_producers_per_topic** | **i32** |  | [optional] 
**max_consumers_per_topic** | **i32** |  | [optional] 
**max_consumers_per_subscription** | **i32** |  | [optional] 
**compaction_threshold** | **i64** |  | [optional] 
**offload_threshold** | **i64** |  | [optional] 
**offload_deletion_lag_ms** | **i64** |  | [optional] 
**schema_auto_update_compatibility_strategy** | **String** |  | [optional] 
**schema_validation_enforced** | **bool** |  | [optional] 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


