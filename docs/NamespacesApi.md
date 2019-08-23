# \NamespacesApi

All URIs are relative to *http://localhost/admin/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**clear_namespace_backlog**](NamespacesApi.md#clear_namespace_backlog) | **Post** /namespaces/{tenant}/{namespace}/clearBacklog | Clear backlog for all topics on a namespace.
[**clear_namespace_backlog_for_subscription**](NamespacesApi.md#clear_namespace_backlog_for_subscription) | **Post** /namespaces/{tenant}/{namespace}/clearBacklog/{subscription} | Clear backlog for a given subscription on all topics on a namespace.
[**clear_namespace_bundle_backlog**](NamespacesApi.md#clear_namespace_bundle_backlog) | **Post** /namespaces/{tenant}/{namespace}/{bundle}/clearBacklog | Clear backlog for all topics on a namespace bundle.
[**clear_namespace_bundle_backlog_for_subscription**](NamespacesApi.md#clear_namespace_bundle_backlog_for_subscription) | **Post** /namespaces/{tenant}/{namespace}/{bundle}/clearBacklog/{subscription} | Clear backlog for a given subscription on all topics on a namespace bundle.
[**clear_offload_deletion_lag**](NamespacesApi.md#clear_offload_deletion_lag) | **Delete** /namespaces/{tenant}/{namespace}/offloadDeletionLagMs | Clear the namespace configured offload deletion lag. The topics in the namespace will fallback to using the default configured deletion lag for the broker
[**create_namespace**](NamespacesApi.md#create_namespace) | **Put** /namespaces/{tenant}/{namespace} | Creates a new namespace with the specified policies
[**delete_bookie_affinity_group**](NamespacesApi.md#delete_bookie_affinity_group) | **Delete** /namespaces/{property}/{namespace}/persistence/bookieAffinity | Delete the bookie-affinity-group from namespace-local policy.
[**delete_namespace**](NamespacesApi.md#delete_namespace) | **Delete** /namespaces/{tenant}/{namespace} | Delete a namespace and all the topics under it.
[**delete_namespace_bundle**](NamespacesApi.md#delete_namespace_bundle) | **Delete** /namespaces/{tenant}/{namespace}/{bundle} | Delete a namespace bundle and all the topics under it.
[**get_anti_affinity_namespaces**](NamespacesApi.md#get_anti_affinity_namespaces) | **Get** /namespaces/{cluster}/antiAffinity/{group} | Get all namespaces that are grouped by given anti-affinity group in a given cluster. api can be only accessed by admin of any of the existing tenant
[**get_backlog_quota_map**](NamespacesApi.md#get_backlog_quota_map) | **Get** /namespaces/{tenant}/{namespace}/backlogQuotaMap | Get backlog quota map on a namespace.
[**get_bookie_affinity_group**](NamespacesApi.md#get_bookie_affinity_group) | **Get** /namespaces/{property}/{namespace}/persistence/bookieAffinity | Get the bookie-affinity-group from namespace-local policy.
[**get_bundles_data**](NamespacesApi.md#get_bundles_data) | **Get** /namespaces/{tenant}/{namespace}/bundles | Get the bundles split data.
[**get_compaction_threshold**](NamespacesApi.md#get_compaction_threshold) | **Get** /namespaces/{tenant}/{namespace}/compactionThreshold | Maximum number of uncompacted bytes in topics before compaction is triggered.
[**get_dispatch_rate**](NamespacesApi.md#get_dispatch_rate) | **Get** /namespaces/{tenant}/{namespace}/dispatchRate | Get dispatch-rate configured for the namespace, -1 represents not configured yet
[**get_max_consumers_per_subscription**](NamespacesApi.md#get_max_consumers_per_subscription) | **Get** /namespaces/{tenant}/{namespace}/maxConsumersPerSubscription | Get maxConsumersPerSubscription config on a namespace.
[**get_max_consumers_per_topic**](NamespacesApi.md#get_max_consumers_per_topic) | **Get** /namespaces/{tenant}/{namespace}/maxConsumersPerTopic | Get maxConsumersPerTopic config on a namespace.
[**get_max_producers_per_topic**](NamespacesApi.md#get_max_producers_per_topic) | **Get** /namespaces/{tenant}/{namespace}/maxProducersPerTopic | Get maxProducersPerTopic config on a namespace.
[**get_namespace_anti_affinity_group**](NamespacesApi.md#get_namespace_anti_affinity_group) | **Get** /namespaces/{tenant}/{namespace}/antiAffinity | Get anti-affinity group of a namespace.
[**get_namespace_message_ttl**](NamespacesApi.md#get_namespace_message_ttl) | **Get** /namespaces/{tenant}/{namespace}/messageTTL | Get the message TTL for the namespace
[**get_namespace_replication_clusters**](NamespacesApi.md#get_namespace_replication_clusters) | **Get** /namespaces/{tenant}/{namespace}/replication | Get the replication clusters for a namespace.
[**get_offload_deletion_lag**](NamespacesApi.md#get_offload_deletion_lag) | **Get** /namespaces/{tenant}/{namespace}/offloadDeletionLagMs | Number of milliseconds to wait before deleting a ledger segment which has been offloaded from the Pulsar cluster's local storage (i.e. BookKeeper)
[**get_offload_threshold**](NamespacesApi.md#get_offload_threshold) | **Get** /namespaces/{tenant}/{namespace}/offloadThreshold | Maximum number of bytes stored on the pulsar cluster for a topic, before the broker will start offloading to longterm storage
[**get_permissions**](NamespacesApi.md#get_permissions) | **Get** /namespaces/{tenant}/{namespace}/permissions | Retrieve the permissions for a namespace.
[**get_persistence**](NamespacesApi.md#get_persistence) | **Get** /namespaces/{tenant}/{namespace}/persistence | Get the persistence configuration for a namespace.
[**get_policies**](NamespacesApi.md#get_policies) | **Get** /namespaces/{tenant}/{namespace} | Get the dump all the policies specified for a namespace.
[**get_replicator_dispatch_rate**](NamespacesApi.md#get_replicator_dispatch_rate) | **Get** /namespaces/{tenant}/{namespace}/replicatorDispatchRate | Get replicator dispatch-rate configured for the namespace, -1 represents not configured yet
[**get_retention**](NamespacesApi.md#get_retention) | **Get** /namespaces/{tenant}/{namespace}/retention | Get retention config on a namespace.
[**get_schema_auto_update_compatibility_strategy**](NamespacesApi.md#get_schema_auto_update_compatibility_strategy) | **Get** /namespaces/{tenant}/{namespace}/schemaAutoUpdateCompatibilityStrategy | The strategy used to check the compatibility of new schemas, provided by producers, before automatically updating the schema
[**get_schema_validtion_enforced**](NamespacesApi.md#get_schema_validtion_enforced) | **Get** /namespaces/{tenant}/{namespace}/schemaValidationEnforced | Get schema validation enforced flag for namespace.
[**get_subscribe_rate**](NamespacesApi.md#get_subscribe_rate) | **Get** /namespaces/{tenant}/{namespace}/subscribeRate | Get subscribe-rate configured for the namespace
[**get_subscription_dispatch_rate**](NamespacesApi.md#get_subscription_dispatch_rate) | **Get** /namespaces/{tenant}/{namespace}/subscriptionDispatchRate | Get Subscription dispatch-rate configured for the namespace, -1 represents not configured yet
[**get_tenant_namespaces**](NamespacesApi.md#get_tenant_namespaces) | **Get** /namespaces/{tenant} | Get the list of all the namespaces for a certain tenant.
[**get_topics**](NamespacesApi.md#get_topics) | **Get** /namespaces/{tenant}/{namespace}/topics | Get the list of all the topics under a certain namespace.
[**grant_permission_on_namespace**](NamespacesApi.md#grant_permission_on_namespace) | **Post** /namespaces/{tenant}/{namespace}/permissions/{role} | Grant a new permission to a role on a namespace.
[**modify_deduplication**](NamespacesApi.md#modify_deduplication) | **Post** /namespaces/{tenant}/{namespace}/deduplication | Enable or disable broker side deduplication for all topics in a namespace
[**modify_encryption_required**](NamespacesApi.md#modify_encryption_required) | **Post** /namespaces/{tenant}/{namespace}/encryptionRequired | Message encryption is required or not for all topics in a namespace
[**remove_backlog_quota**](NamespacesApi.md#remove_backlog_quota) | **Delete** /namespaces/{tenant}/{namespace}/backlogQuota | Remove a backlog quota policy from a namespace.
[**remove_namespace_anti_affinity_group**](NamespacesApi.md#remove_namespace_anti_affinity_group) | **Delete** /namespaces/{tenant}/{namespace}/antiAffinity | Remove anti-affinity group of a namespace.
[**revoke_permissions_on_namespace**](NamespacesApi.md#revoke_permissions_on_namespace) | **Delete** /namespaces/{tenant}/{namespace}/permissions/{role} | Revoke all permissions to a role on a namespace.
[**set_backlog_quota**](NamespacesApi.md#set_backlog_quota) | **Post** /namespaces/{tenant}/{namespace}/backlogQuota |  Set a backlog quota for all the topics on a namespace.
[**set_bookie_affinity_group**](NamespacesApi.md#set_bookie_affinity_group) | **Post** /namespaces/{tenant}/{namespace}/persistence/bookieAffinity | Set the bookie-affinity-group to namespace-persistent policy.
[**set_compaction_threshold**](NamespacesApi.md#set_compaction_threshold) | **Put** /namespaces/{tenant}/{namespace}/compactionThreshold | Set maximum number of uncompacted bytes in a topic before compaction is triggered.
[**set_dispatch_rate**](NamespacesApi.md#set_dispatch_rate) | **Post** /namespaces/{tenant}/{namespace}/dispatchRate | Set dispatch-rate throttling for all topics of the namespace
[**set_max_consumers_per_subscription**](NamespacesApi.md#set_max_consumers_per_subscription) | **Post** /namespaces/{tenant}/{namespace}/maxConsumersPerSubscription |  Set maxConsumersPerSubscription configuration on a namespace.
[**set_max_consumers_per_topic**](NamespacesApi.md#set_max_consumers_per_topic) | **Post** /namespaces/{tenant}/{namespace}/maxConsumersPerTopic |  Set maxConsumersPerTopic configuration on a namespace.
[**set_max_producers_per_topic**](NamespacesApi.md#set_max_producers_per_topic) | **Post** /namespaces/{tenant}/{namespace}/maxProducersPerTopic |  Set maxProducersPerTopic configuration on a namespace.
[**set_namespace_anti_affinity_group**](NamespacesApi.md#set_namespace_anti_affinity_group) | **Post** /namespaces/{tenant}/{namespace}/antiAffinity | Set anti-affinity group for a namespace
[**set_namespace_message_ttl**](NamespacesApi.md#set_namespace_message_ttl) | **Post** /namespaces/{tenant}/{namespace}/messageTTL | Set message TTL in seconds for namespace
[**set_namespace_replication_clusters**](NamespacesApi.md#set_namespace_replication_clusters) | **Post** /namespaces/{tenant}/{namespace}/replication | Set the replication clusters for a namespace.
[**set_offload_deletion_lag**](NamespacesApi.md#set_offload_deletion_lag) | **Put** /namespaces/{tenant}/{namespace}/offloadDeletionLagMs | Set number of milliseconds to wait before deleting a ledger segment which has been offloaded from the Pulsar cluster's local storage (i.e. BookKeeper)
[**set_offload_threshold**](NamespacesApi.md#set_offload_threshold) | **Put** /namespaces/{tenant}/{namespace}/offloadThreshold | Set maximum number of bytes stored on the pulsar cluster for a topic, before the broker will start offloading to longterm storage
[**set_persistence**](NamespacesApi.md#set_persistence) | **Post** /namespaces/{tenant}/{namespace}/persistence | Set the persistence configuration for all the topics on a namespace.
[**set_replicator_dispatch_rate**](NamespacesApi.md#set_replicator_dispatch_rate) | **Post** /namespaces/{tenant}/{namespace}/replicatorDispatchRate | Set replicator dispatch-rate throttling for all topics of the namespace
[**set_retention**](NamespacesApi.md#set_retention) | **Post** /namespaces/{tenant}/{namespace}/retention |  Set retention configuration on a namespace.
[**set_schema_auto_update_compatibility_strategy**](NamespacesApi.md#set_schema_auto_update_compatibility_strategy) | **Put** /namespaces/{tenant}/{namespace}/schemaAutoUpdateCompatibilityStrategy | Update the strategy used to check the compatibility of new schemas, provided by producers, before automatically updating the schema
[**set_schema_validtion_enforced**](NamespacesApi.md#set_schema_validtion_enforced) | **Post** /namespaces/{tenant}/{namespace}/schemaValidationEnforced | Set schema validation enforced flag on namespace.
[**set_subscribe_rate**](NamespacesApi.md#set_subscribe_rate) | **Post** /namespaces/{tenant}/{namespace}/subscribeRate | Set subscribe-rate throttling for all topics of the namespace
[**set_subscription_auth_mode**](NamespacesApi.md#set_subscription_auth_mode) | **Post** /namespaces/{tenant}/{namespace}/subscriptionAuthMode |  Set a subscription auth mode for all the topics on a namespace.
[**set_subscription_dispatch_rate**](NamespacesApi.md#set_subscription_dispatch_rate) | **Post** /namespaces/{tenant}/{namespace}/subscriptionDispatchRate | Set Subscription dispatch-rate throttling for all topics of the namespace
[**split_namespace_bundle**](NamespacesApi.md#split_namespace_bundle) | **Put** /namespaces/{tenant}/{namespace}/{bundle}/split | Split a namespace bundle
[**unload_namespace**](NamespacesApi.md#unload_namespace) | **Put** /namespaces/{tenant}/{namespace}/unload | Unload namespace
[**unload_namespace_bundle**](NamespacesApi.md#unload_namespace_bundle) | **Put** /namespaces/{tenant}/{namespace}/{bundle}/unload | Unload a namespace bundle
[**unsubscribe_namespace**](NamespacesApi.md#unsubscribe_namespace) | **Post** /namespaces/{tenant}/{namespace}/unsubscribe/{subscription} | Unsubscribes the given subscription on all topics on a namespace.
[**unsubscribe_namespace_bundle**](NamespacesApi.md#unsubscribe_namespace_bundle) | **Post** /namespaces/{tenant}/{namespace}/{bundle}/unsubscribe/{subscription} | Unsubscribes the given subscription on all topics on a namespace bundle.



## clear_namespace_backlog

> clear_namespace_backlog(tenant, namespace, authoritative)
Clear backlog for all topics on a namespace.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** |  | Required | 
**namespace** | **String** |  | Required | 
**authoritative** | **bool** |  |  | [default to false]

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## clear_namespace_backlog_for_subscription

> clear_namespace_backlog_for_subscription(tenant, namespace, subscription, authoritative)
Clear backlog for a given subscription on all topics on a namespace.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** |  | Required | 
**namespace** | **String** |  | Required | 
**subscription** | **String** |  | Required | 
**authoritative** | **bool** |  |  | [default to false]

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## clear_namespace_bundle_backlog

> clear_namespace_bundle_backlog(tenant, namespace, bundle, authoritative)
Clear backlog for all topics on a namespace bundle.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** |  | Required | 
**namespace** | **String** |  | Required | 
**bundle** | **String** |  | Required | 
**authoritative** | **bool** |  |  | [default to false]

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## clear_namespace_bundle_backlog_for_subscription

> clear_namespace_bundle_backlog_for_subscription(tenant, namespace, subscription, bundle, authoritative)
Clear backlog for a given subscription on all topics on a namespace bundle.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** |  | Required | 
**namespace** | **String** |  | Required | 
**subscription** | **String** |  | Required | 
**bundle** | **String** |  | Required | 
**authoritative** | **bool** |  |  | [default to false]

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## clear_offload_deletion_lag

> clear_offload_deletion_lag(tenant, namespace)
Clear the namespace configured offload deletion lag. The topics in the namespace will fallback to using the default configured deletion lag for the broker

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** |  | Required | 
**namespace** | **String** |  | Required | 

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_namespace

> create_namespace(tenant, namespace)
Creates a new namespace with the specified policies

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** |  | Required | 
**namespace** | **String** |  | Required | 

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_bookie_affinity_group

> delete_bookie_affinity_group(property, namespace)
Delete the bookie-affinity-group from namespace-local policy.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**property** | **String** |  | Required | 
**namespace** | **String** |  | Required | 

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_namespace

> delete_namespace(tenant, namespace, authoritative)
Delete a namespace and all the topics under it.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** |  | Required | 
**namespace** | **String** |  | Required | 
**authoritative** | **bool** |  |  | [default to false]

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_namespace_bundle

> delete_namespace_bundle(tenant, namespace, bundle, authoritative)
Delete a namespace bundle and all the topics under it.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** |  | Required | 
**namespace** | **String** |  | Required | 
**bundle** | **String** |  | Required | 
**authoritative** | **bool** |  |  | [default to false]

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_anti_affinity_namespaces

> Vec<serde_json::Value> get_anti_affinity_namespaces(cluster, group, tenant)
Get all namespaces that are grouped by given anti-affinity group in a given cluster. api can be only accessed by admin of any of the existing tenant

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cluster** | **String** |  | Required | 
**group** | **String** |  | Required | 
**tenant** | **String** |  |  | 

### Return type

[**Vec<serde_json::Value>**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_backlog_quota_map

> ::std::collections::HashMap<String, serde_json::Value> get_backlog_quota_map(tenant, namespace)
Get backlog quota map on a namespace.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** |  | Required | 
**namespace** | **String** |  | Required | 

### Return type

[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_bookie_affinity_group

> crate::models::BookieAffinityGroupData get_bookie_affinity_group(property, namespace)
Get the bookie-affinity-group from namespace-local policy.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**property** | **String** |  | Required | 
**namespace** | **String** |  | Required | 

### Return type

[**crate::models::BookieAffinityGroupData**](BookieAffinityGroupData.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_bundles_data

> crate::models::BundlesData get_bundles_data(tenant, namespace)
Get the bundles split data.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** |  | Required | 
**namespace** | **String** |  | Required | 

### Return type

[**crate::models::BundlesData**](BundlesData.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_compaction_threshold

> i64 get_compaction_threshold(tenant, namespace)
Maximum number of uncompacted bytes in topics before compaction is triggered.

The backlog size is compared to the threshold periodically. A threshold of 0 disabled automatic compaction

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** |  | Required | 
**namespace** | **String** |  | Required | 

### Return type

**i64**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_dispatch_rate

> crate::models::DispatchRate get_dispatch_rate(tenant, namespace)
Get dispatch-rate configured for the namespace, -1 represents not configured yet

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** |  | Required | 
**namespace** | **String** |  | Required | 

### Return type

[**crate::models::DispatchRate**](DispatchRate.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_max_consumers_per_subscription

> i32 get_max_consumers_per_subscription(tenant, namespace)
Get maxConsumersPerSubscription config on a namespace.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** |  | Required | 
**namespace** | **String** |  | Required | 

### Return type

**i32**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_max_consumers_per_topic

> i32 get_max_consumers_per_topic(tenant, namespace)
Get maxConsumersPerTopic config on a namespace.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** |  | Required | 
**namespace** | **String** |  | Required | 

### Return type

**i32**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_max_producers_per_topic

> i32 get_max_producers_per_topic(tenant, namespace)
Get maxProducersPerTopic config on a namespace.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** |  | Required | 
**namespace** | **String** |  | Required | 

### Return type

**i32**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_namespace_anti_affinity_group

> String get_namespace_anti_affinity_group(tenant, namespace)
Get anti-affinity group of a namespace.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** |  | Required | 
**namespace** | **String** |  | Required | 

### Return type

**String**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_namespace_message_ttl

> i32 get_namespace_message_ttl(tenant, namespace)
Get the message TTL for the namespace

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** |  | Required | 
**namespace** | **String** |  | Required | 

### Return type

**i32**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_namespace_replication_clusters

> Vec<String> get_namespace_replication_clusters(tenant, namespace)
Get the replication clusters for a namespace.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** |  | Required | 
**namespace** | **String** |  | Required | 

### Return type

**Vec<String>**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_offload_deletion_lag

> i64 get_offload_deletion_lag(tenant, namespace)
Number of milliseconds to wait before deleting a ledger segment which has been offloaded from the Pulsar cluster's local storage (i.e. BookKeeper)

A negative value denotes that deletion has been completely disabled. 'null' denotes that the topics in the namespace will fall back to the broker default for deletion lag.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** |  | Required | 
**namespace** | **String** |  | Required | 

### Return type

**i64**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_offload_threshold

> i64 get_offload_threshold(tenant, namespace)
Maximum number of bytes stored on the pulsar cluster for a topic, before the broker will start offloading to longterm storage

A negative value disables automatic offloading

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** |  | Required | 
**namespace** | **String** |  | Required | 

### Return type

**i64**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_permissions

> ::std::collections::HashMap<String, serde_json::Value> get_permissions(tenant, cluster, namespace)
Retrieve the permissions for a namespace.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** |  | Required | 
**cluster** | **String** |  | Required | 
**namespace** | **String** |  | Required | 

### Return type

[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_persistence

> crate::models::PersistencePolicies get_persistence(tenant, namespace)
Get the persistence configuration for a namespace.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** |  | Required | 
**namespace** | **String** |  | Required | 

### Return type

[**crate::models::PersistencePolicies**](PersistencePolicies.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_policies

> crate::models::Policies get_policies(tenant, namespace)
Get the dump all the policies specified for a namespace.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** |  | Required | 
**namespace** | **String** |  | Required | 

### Return type

[**crate::models::Policies**](Policies.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_replicator_dispatch_rate

> crate::models::DispatchRate get_replicator_dispatch_rate(tenant, namespace)
Get replicator dispatch-rate configured for the namespace, -1 represents not configured yet

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** |  | Required | 
**namespace** | **String** |  | Required | 

### Return type

[**crate::models::DispatchRate**](DispatchRate.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_retention

> crate::models::RetentionPolicies get_retention(tenant, namespace)
Get retention config on a namespace.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** |  | Required | 
**namespace** | **String** |  | Required | 

### Return type

[**crate::models::RetentionPolicies**](RetentionPolicies.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_schema_auto_update_compatibility_strategy

> String get_schema_auto_update_compatibility_strategy(tenant, namespace)
The strategy used to check the compatibility of new schemas, provided by producers, before automatically updating the schema

The value AutoUpdateDisabled prevents producers from updating the schema.  If set to AutoUpdateDisabled, schemas must be updated through the REST api

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** |  | Required | 
**namespace** | **String** |  | Required | 

### Return type

**String**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_schema_validtion_enforced

> bool get_schema_validtion_enforced(tenant, namespace)
Get schema validation enforced flag for namespace.

If the flag is set to true, when a producer without a schema attempts to produce to a topic with schema in this namespace, the producer will be failed to connect. PLEASE be carefully on using this, since non-java clients don't support schema.if you enable this setting, it will cause non-java clients failed to produce.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** |  | Required | 
**namespace** | **String** |  | Required | 

### Return type

**bool**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_subscribe_rate

> crate::models::SubscribeRate get_subscribe_rate(tenant, namespace)
Get subscribe-rate configured for the namespace

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** |  | Required | 
**namespace** | **String** |  | Required | 

### Return type

[**crate::models::SubscribeRate**](SubscribeRate.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_subscription_dispatch_rate

> crate::models::DispatchRate get_subscription_dispatch_rate(tenant, namespace)
Get Subscription dispatch-rate configured for the namespace, -1 represents not configured yet

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** |  | Required | 
**namespace** | **String** |  | Required | 

### Return type

[**crate::models::DispatchRate**](DispatchRate.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_tenant_namespaces

> Vec<String> get_tenant_namespaces(tenant)
Get the list of all the namespaces for a certain tenant.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** |  | Required | 

### Return type

**Vec<String>**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_topics

> Vec<String> get_topics(tenant, namespace, mode)
Get the list of all the topics under a certain namespace.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** |  | Required | 
**namespace** | **String** |  | Required | 
**mode** | **String** |  |  | [default to PERSISTENT]

### Return type

**Vec<String>**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## grant_permission_on_namespace

> grant_permission_on_namespace(tenant, namespace, role)
Grant a new permission to a role on a namespace.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** |  | Required | 
**namespace** | **String** |  | Required | 
**role** | **String** |  | Required | 

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## modify_deduplication

> modify_deduplication(tenant, namespace)
Enable or disable broker side deduplication for all topics in a namespace

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** |  | Required | 
**namespace** | **String** |  | Required | 

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## modify_encryption_required

> modify_encryption_required(tenant, namespace)
Message encryption is required or not for all topics in a namespace

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** |  | Required | 
**namespace** | **String** |  | Required | 

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## remove_backlog_quota

> remove_backlog_quota(tenant, namespace, backlog_quota_type)
Remove a backlog quota policy from a namespace.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** |  | Required | 
**namespace** | **String** |  | Required | 
**backlog_quota_type** | **String** |  |  | 

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## remove_namespace_anti_affinity_group

> remove_namespace_anti_affinity_group(tenant, namespace)
Remove anti-affinity group of a namespace.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** |  | Required | 
**namespace** | **String** |  | Required | 

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## revoke_permissions_on_namespace

> revoke_permissions_on_namespace(tenant, namespace, role)
Revoke all permissions to a role on a namespace.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** |  | Required | 
**namespace** | **String** |  | Required | 
**role** | **String** |  | Required | 

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## set_backlog_quota

> set_backlog_quota(tenant, namespace, backlog_quota_type)
 Set a backlog quota for all the topics on a namespace.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** |  | Required | 
**namespace** | **String** |  | Required | 
**backlog_quota_type** | **String** |  |  | 

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## set_bookie_affinity_group

> set_bookie_affinity_group(tenant, namespace)
Set the bookie-affinity-group to namespace-persistent policy.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** |  | Required | 
**namespace** | **String** |  | Required | 

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## set_compaction_threshold

> set_compaction_threshold(tenant, namespace)
Set maximum number of uncompacted bytes in a topic before compaction is triggered.

The backlog size is compared to the threshold periodically. A threshold of 0 disabled automatic compaction

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** |  | Required | 
**namespace** | **String** |  | Required | 

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## set_dispatch_rate

> set_dispatch_rate(tenant, namespace)
Set dispatch-rate throttling for all topics of the namespace

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** |  | Required | 
**namespace** | **String** |  | Required | 

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## set_max_consumers_per_subscription

> set_max_consumers_per_subscription(tenant, namespace)
 Set maxConsumersPerSubscription configuration on a namespace.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** |  | Required | 
**namespace** | **String** |  | Required | 

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## set_max_consumers_per_topic

> set_max_consumers_per_topic(tenant, namespace)
 Set maxConsumersPerTopic configuration on a namespace.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** |  | Required | 
**namespace** | **String** |  | Required | 

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## set_max_producers_per_topic

> set_max_producers_per_topic(tenant, namespace)
 Set maxProducersPerTopic configuration on a namespace.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** |  | Required | 
**namespace** | **String** |  | Required | 

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## set_namespace_anti_affinity_group

> set_namespace_anti_affinity_group(tenant, namespace)
Set anti-affinity group for a namespace

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** |  | Required | 
**namespace** | **String** |  | Required | 

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## set_namespace_message_ttl

> set_namespace_message_ttl(tenant, namespace)
Set message TTL in seconds for namespace

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** |  | Required | 
**namespace** | **String** |  | Required | 

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## set_namespace_replication_clusters

> set_namespace_replication_clusters(tenant, namespace)
Set the replication clusters for a namespace.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** |  | Required | 
**namespace** | **String** |  | Required | 

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## set_offload_deletion_lag

> set_offload_deletion_lag(tenant, namespace)
Set number of milliseconds to wait before deleting a ledger segment which has been offloaded from the Pulsar cluster's local storage (i.e. BookKeeper)

A negative value disables the deletion completely.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** |  | Required | 
**namespace** | **String** |  | Required | 

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## set_offload_threshold

> set_offload_threshold(tenant, namespace)
Set maximum number of bytes stored on the pulsar cluster for a topic, before the broker will start offloading to longterm storage

A negative value disables automatic offloading

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** |  | Required | 
**namespace** | **String** |  | Required | 

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## set_persistence

> set_persistence(tenant, namespace)
Set the persistence configuration for all the topics on a namespace.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** |  | Required | 
**namespace** | **String** |  | Required | 

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## set_replicator_dispatch_rate

> set_replicator_dispatch_rate(tenant, namespace)
Set replicator dispatch-rate throttling for all topics of the namespace

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** |  | Required | 
**namespace** | **String** |  | Required | 

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## set_retention

> set_retention(tenant, namespace)
 Set retention configuration on a namespace.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** |  | Required | 
**namespace** | **String** |  | Required | 

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## set_schema_auto_update_compatibility_strategy

> set_schema_auto_update_compatibility_strategy(tenant, namespace)
Update the strategy used to check the compatibility of new schemas, provided by producers, before automatically updating the schema

The value AutoUpdateDisabled prevents producers from updating the schema.  If set to AutoUpdateDisabled, schemas must be updated through the REST api

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** |  | Required | 
**namespace** | **String** |  | Required | 

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## set_schema_validtion_enforced

> set_schema_validtion_enforced(tenant, namespace)
Set schema validation enforced flag on namespace.

If the flag is set to true, when a producer without a schema attempts to produce to a topic with schema in this namespace, the producer will be failed to connect. PLEASE be carefully on using this, since non-java clients don't support schema.if you enable this setting, it will cause non-java clients failed to produce.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** |  | Required | 
**namespace** | **String** |  | Required | 

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## set_subscribe_rate

> set_subscribe_rate(tenant, namespace)
Set subscribe-rate throttling for all topics of the namespace

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** |  | Required | 
**namespace** | **String** |  | Required | 

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## set_subscription_auth_mode

> set_subscription_auth_mode(tenant, namespace)
 Set a subscription auth mode for all the topics on a namespace.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** |  | Required | 
**namespace** | **String** |  | Required | 

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## set_subscription_dispatch_rate

> set_subscription_dispatch_rate(tenant, namespace)
Set Subscription dispatch-rate throttling for all topics of the namespace

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** |  | Required | 
**namespace** | **String** |  | Required | 

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## split_namespace_bundle

> split_namespace_bundle(tenant, namespace, bundle, authoritative, unload)
Split a namespace bundle

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** |  | Required | 
**namespace** | **String** |  | Required | 
**bundle** | **String** |  | Required | 
**authoritative** | **bool** |  |  | [default to false]
**unload** | **bool** |  |  | [default to false]

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## unload_namespace

> unload_namespace(tenant, namespace)
Unload namespace

Unload an active namespace from the current broker serving it. Performing this operation will let the brokerremoves all producers, consumers, and connections using this namespace, and close all topics (includingtheir persistent store). During that operation, the namespace is marked as tentatively unavailable until thebroker completes the unloading action. This operation requires strictly super user privileges, since it wouldresult in non-persistent message loss and unexpected connection closure to the clients.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** |  | Required | 
**namespace** | **String** |  | Required | 

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## unload_namespace_bundle

> unload_namespace_bundle(tenant, namespace, bundle, authoritative)
Unload a namespace bundle

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** |  | Required | 
**namespace** | **String** |  | Required | 
**bundle** | **String** |  | Required | 
**authoritative** | **bool** |  |  | [default to false]

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## unsubscribe_namespace

> unsubscribe_namespace(tenant, cluster, namespace, subscription, authoritative)
Unsubscribes the given subscription on all topics on a namespace.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** |  | Required | 
**cluster** | **String** |  | Required | 
**namespace** | **String** |  | Required | 
**subscription** | **String** |  | Required | 
**authoritative** | **bool** |  |  | [default to false]

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## unsubscribe_namespace_bundle

> unsubscribe_namespace_bundle(tenant, namespace, subscription, bundle, authoritative)
Unsubscribes the given subscription on all topics on a namespace bundle.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** |  | Required | 
**namespace** | **String** |  | Required | 
**subscription** | **String** |  | Required | 
**bundle** | **String** |  | Required | 
**authoritative** | **bool** |  |  | [default to false]

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

