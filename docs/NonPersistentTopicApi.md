# \NonPersistentTopicApi

All URIs are relative to *http://localhost/admin/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**compact**](NonPersistentTopicApi.md#compact) | **Put** /non-persistent/{tenant}/{namespace}/{topic}/compaction | Trigger a compaction operation on a topic.
[**compaction_status**](NonPersistentTopicApi.md#compaction_status) | **Get** /non-persistent/{tenant}/{namespace}/{topic}/compaction | Get the status of a compaction operation for a topic.
[**create_non_partitioned_topic**](NonPersistentTopicApi.md#create_non_partitioned_topic) | **Put** /non-persistent/{tenant}/{namespace}/{topic} | Create a non-partitioned topic.
[**create_partitioned_topic**](NonPersistentTopicApi.md#create_partitioned_topic) | **Put** /non-persistent/{tenant}/{namespace}/{topic}/partitions | Create a partitioned topic.
[**create_subscription**](NonPersistentTopicApi.md#create_subscription) | **Put** /non-persistent/{tenant}/{namespace}/{topic}/subscription/{subscriptionName} | Reset subscription to message position closest to given position.
[**delete_partitioned_topic**](NonPersistentTopicApi.md#delete_partitioned_topic) | **Delete** /non-persistent/{tenant}/{namespace}/{topic}/partitions | Delete a partitioned topic.
[**delete_subscription**](NonPersistentTopicApi.md#delete_subscription) | **Delete** /non-persistent/{tenant}/{namespace}/{topic}/subscription/{subName} | Delete a subscription.
[**delete_topic**](NonPersistentTopicApi.md#delete_topic) | **Delete** /non-persistent/{tenant}/{namespace}/{topic} | Delete a topic.
[**expire_messages_for_all_subscriptions**](NonPersistentTopicApi.md#expire_messages_for_all_subscriptions) | **Post** /non-persistent/{tenant}/{namespace}/{topic}/all_subscription/expireMessages/{expireTimeInSeconds} | Expiry messages on all subscriptions of topic.
[**expire_topic_messages**](NonPersistentTopicApi.md#expire_topic_messages) | **Post** /non-persistent/{tenant}/{namespace}/{topic}/subscription/{subName}/expireMessages/{expireTimeInSeconds} | Expiry messages on a topic subscription.
[**get_backlog**](NonPersistentTopicApi.md#get_backlog) | **Get** /non-persistent/{tenant}/{namespace}/{topic}/backlog | Get estimated backlog for offline topic.
[**get_internal_stats**](NonPersistentTopicApi.md#get_internal_stats) | **Get** /non-persistent/{tenant}/{namespace}/{topic}/internalStats | Get the internal stats for the topic.
[**get_last_message_id**](NonPersistentTopicApi.md#get_last_message_id) | **Get** /non-persistent/{tenant}/{namespace}/{topic}/lastMessageId | Return the last commit message id of topic
[**get_list**](NonPersistentTopicApi.md#get_list) | **Get** /non-persistent/{tenant}/{namespace} | Get the list of non-persistent topics under a namespace.
[**get_list_from_bundle**](NonPersistentTopicApi.md#get_list_from_bundle) | **Get** /non-persistent/{tenant}/{namespace}/{bundle} | Get the list of non-persistent topics under a namespace bundle.
[**get_managed_ledger_info**](NonPersistentTopicApi.md#get_managed_ledger_info) | **Get** /non-persistent/{tenant}/{namespace}/{topic}/internal-info | Get the internal stats for the topic.
[**get_partitioned_metadata**](NonPersistentTopicApi.md#get_partitioned_metadata) | **Get** /non-persistent/{tenant}/{namespace}/{topic}/partitions | Get partitioned topic metadata.
[**get_partitioned_stats**](NonPersistentTopicApi.md#get_partitioned_stats) | **Get** /non-persistent/{tenant}/{namespace}/{topic}/partitioned-stats | Get the stats for the partitioned topic.
[**get_partitioned_topic_list**](NonPersistentTopicApi.md#get_partitioned_topic_list) | **Get** /non-persistent/{tenant}/{namespace}/partitioned | Get the list of partitioned topics under a namespace.
[**get_permissions_on_topic**](NonPersistentTopicApi.md#get_permissions_on_topic) | **Get** /non-persistent/{tenant}/{namespace}/{topic}/permissions | Get permissions on a topic.
[**get_stats**](NonPersistentTopicApi.md#get_stats) | **Get** /non-persistent/{tenant}/{namespace}/{topic}/stats | Get the stats for the topic.
[**get_subscriptions**](NonPersistentTopicApi.md#get_subscriptions) | **Get** /non-persistent/{tenant}/{namespace}/{topic}/subscriptions | Get the list of persistent subscriptions for a given topic.
[**grant_permissions_on_topic**](NonPersistentTopicApi.md#grant_permissions_on_topic) | **Post** /non-persistent/{tenant}/{namespace}/{topic}/permissions/{role} | Grant a new permission to a role on a single topic.
[**offload_status**](NonPersistentTopicApi.md#offload_status) | **Get** /non-persistent/{tenant}/{namespace}/{topic}/offload | Offload a prefix of a topic to long term storage
[**peek_nth_message**](NonPersistentTopicApi.md#peek_nth_message) | **Get** /non-persistent/{tenant}/{namespace}/{topic}/subscription/{subName}/position/{messagePosition} | Peek nth message on a topic subscription.
[**reset_cursor**](NonPersistentTopicApi.md#reset_cursor) | **Post** /non-persistent/{tenant}/{namespace}/{topic}/subscription/{subName}/resetcursor/{timestamp} | Reset subscription to message position closest to absolute timestamp (in ms).
[**reset_cursor_on_position**](NonPersistentTopicApi.md#reset_cursor_on_position) | **Post** /non-persistent/{tenant}/{namespace}/{topic}/subscription/{subName}/resetcursor | Reset subscription to message position closest to given position.
[**revoke_permissions_on_topic**](NonPersistentTopicApi.md#revoke_permissions_on_topic) | **Delete** /non-persistent/{tenant}/{namespace}/{topic}/permissions/{role} | Revoke permissions on a topic.
[**skip_all_messages**](NonPersistentTopicApi.md#skip_all_messages) | **Post** /non-persistent/{tenant}/{namespace}/{topic}/subscription/{subName}/skip_all | Skip all messages on a topic subscription.
[**skip_messages**](NonPersistentTopicApi.md#skip_messages) | **Post** /non-persistent/{tenant}/{namespace}/{topic}/subscription/{subName}/skip/{numMessages} | Skipping messages on a topic subscription.
[**terminate**](NonPersistentTopicApi.md#terminate) | **Post** /non-persistent/{tenant}/{namespace}/{topic}/terminate | Terminate a topic. A topic that is terminated will not accept any more messages to be published and will let consumer to drain existing messages in backlog
[**trigger_offload**](NonPersistentTopicApi.md#trigger_offload) | **Put** /non-persistent/{tenant}/{namespace}/{topic}/offload | Offload a prefix of a topic to long term storage
[**unload_topic**](NonPersistentTopicApi.md#unload_topic) | **Put** /non-persistent/{tenant}/{namespace}/{topic}/unload | Unload a topic
[**update_partitioned_topic**](NonPersistentTopicApi.md#update_partitioned_topic) | **Post** /non-persistent/{tenant}/{namespace}/{topic}/partitions | Increment partitons of an existing partitioned topic.



## compact

> compact(tenant, namespace, topic, authoritative)
Trigger a compaction operation on a topic.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** | Specify the tenant | Required | 
**namespace** | **String** | Specify the namespace | Required | 
**topic** | **String** | Specify topic name | Required | 
**authoritative** | **bool** | Is authentication required to perform this operation |  | [default to false]

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## compaction_status

> crate::models::LongRunningProcessStatus compaction_status(tenant, namespace, topic, authoritative)
Get the status of a compaction operation for a topic.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** | Specify the tenant | Required | 
**namespace** | **String** | Specify the namespace | Required | 
**topic** | **String** | Specify topic name | Required | 
**authoritative** | **bool** | Is authentication required to perform this operation |  | [default to false]

### Return type

[**crate::models::LongRunningProcessStatus**](LongRunningProcessStatus.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_non_partitioned_topic

> create_non_partitioned_topic(tenant, namespace, topic, authoritative)
Create a non-partitioned topic.

This is the only REST endpoint from which non-partitioned topics could be created.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** | Specify the tenant | Required | 
**namespace** | **String** | Specify the namespace | Required | 
**topic** | **String** | Specify topic name | Required | 
**authoritative** | **bool** | Is authentication required to perform this operation |  | [default to false]

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_partitioned_topic

> create_partitioned_topic(tenant, namespace, topic, body)
Create a partitioned topic.

It needs to be called before creating a producer on a partitioned topic.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** | Specify the tenant | Required | 
**namespace** | **String** | Specify the namespace | Required | 
**topic** | **String** | Specify topic name | Required | 
**body** | **i32** | The number of partitions for the topic | Required | 

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_subscription

> create_subscription(tenant, namespace, topic, subscription_name, authoritative, replicated)
Reset subscription to message position closest to given position.

Creates a subscription on the topic at the specified message id

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** | Specify the tenant | Required | 
**namespace** | **String** | Specify the namespace | Required | 
**topic** | **String** | Specify topic name | Required | 
**subscription_name** | **String** | Subscription to create position on | Required | 
**authoritative** | **bool** | messageId where to create the subscription. It can be 'latest', 'earliest' or (ledgerId:entryId) |  | [default to false]
**replicated** | **bool** | Is authentication required to perform this operation |  | 

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_partitioned_topic

> delete_partitioned_topic(tenant, namespace, topic, force, authoritative)
Delete a partitioned topic.

It will also delete all the partitions of the topic if it exists.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** | Specify the tenant | Required | 
**namespace** | **String** | Specify the namespace | Required | 
**topic** | **String** | Specify topic name | Required | 
**force** | **bool** | Stop all producer/consumer/replicator and delete topic forcefully |  | [default to false]
**authoritative** | **bool** | Is authentication required to perform this operation |  | [default to false]

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_subscription

> delete_subscription(tenant, namespace, topic, sub_name, authoritative)
Delete a subscription.

There should not be any active consumers on the subscription.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** | Specify the tenant | Required | 
**namespace** | **String** | Specify the namespace | Required | 
**topic** | **String** | Specify topic name | Required | 
**sub_name** | **String** | Subscription to be deleted | Required | 
**authoritative** | **bool** | Is authentication required to perform this operation |  | [default to false]

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_topic

> delete_topic(tenant, namespace, topic, force, authoritative)
Delete a topic.

The topic cannot be deleted if delete is not forcefully and there's any active subscription or producer connected to the it. Force delete ignores connected clients and deletes topic by explicitly closing them.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** | Specify the tenant | Required | 
**namespace** | **String** | Specify the namespace | Required | 
**topic** | **String** | Specify topic name | Required | 
**force** | **bool** | Stop all producer/consumer/replicator and delete topic forcefully |  | [default to false]
**authoritative** | **bool** | Is authentication required to perform this operation |  | [default to false]

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## expire_messages_for_all_subscriptions

> expire_messages_for_all_subscriptions(tenant, namespace, topic, expire_time_in_seconds, authoritative)
Expiry messages on all subscriptions of topic.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** | Specify the tenant | Required | 
**namespace** | **String** | Specify the namespace | Required | 
**topic** | **String** | Specify topic name | Required | 
**expire_time_in_seconds** | **i32** | Expires beyond the specified number of seconds | Required | [default to 0]
**authoritative** | **bool** | Is authentication required to perform this operation |  | [default to false]

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## expire_topic_messages

> expire_topic_messages(tenant, namespace, topic, sub_name, expire_time_in_seconds, authoritative)
Expiry messages on a topic subscription.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** | Specify the tenant | Required | 
**namespace** | **String** | Specify the namespace | Required | 
**topic** | **String** | Specify topic name | Required | 
**sub_name** | **String** | Subscription to be Expiry messages on | Required | 
**expire_time_in_seconds** | **i32** | Expires beyond the specified number of seconds | Required | [default to 0]
**authoritative** | **bool** | Is authentication required to perform this operation |  | [default to false]

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_backlog

> crate::models::PersistentOfflineTopicStats get_backlog(tenant, namespace, topic, authoritative)
Get estimated backlog for offline topic.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** | Specify the tenant | Required | 
**namespace** | **String** | Specify the namespace | Required | 
**topic** | **String** | Specify topic name | Required | 
**authoritative** | **bool** | Is authentication required to perform this operation |  | [default to false]

### Return type

[**crate::models::PersistentOfflineTopicStats**](PersistentOfflineTopicStats.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_internal_stats

> crate::models::PersistentTopicInternalStats get_internal_stats(tenant, namespace, topic, authoritative)
Get the internal stats for the topic.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** | Specify the tenant | Required | 
**namespace** | **String** | Specify the namespace | Required | 
**topic** | **String** | Specify topic name | Required | 
**authoritative** | **bool** | Is authentication required to perform this operation |  | [default to false]

### Return type

[**crate::models::PersistentTopicInternalStats**](PersistentTopicInternalStats.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_last_message_id

> serde_json::Value get_last_message_id(tenant, namespace, topic, authoritative)
Return the last commit message id of topic

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** | Specify the tenant | Required | 
**namespace** | **String** | Specify the namespace | Required | 
**topic** | **String** | Specify topic name | Required | 
**authoritative** | **bool** | Is authentication required to perform this operation |  | [default to false]

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_list

> Vec<String> get_list(tenant, namespace)
Get the list of non-persistent topics under a namespace.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** | Specify the tenant | Required | 
**namespace** | **String** | Specify the namespace | Required | 

### Return type

**Vec<String>**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_list_from_bundle

> Vec<String> get_list_from_bundle(tenant, namespace, bundle)
Get the list of non-persistent topics under a namespace bundle.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** |  | Required | 
**namespace** | **String** |  | Required | 
**bundle** | **String** |  | Required | 

### Return type

**Vec<String>**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_managed_ledger_info

> get_managed_ledger_info(tenant, namespace, topic)
Get the internal stats for the topic.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** | Specify the tenant | Required | 
**namespace** | **String** | Specify the namespace | Required | 
**topic** | **String** | Specify topic name | Required | 

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_partitioned_metadata

> crate::models::PartitionedTopicMetadata get_partitioned_metadata(tenant, namespace, topic, authoritative)
Get partitioned topic metadata.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** | Specify the tenant | Required | 
**namespace** | **String** | Specify the namespace | Required | 
**topic** | **String** | Specify topic name | Required | 
**authoritative** | **bool** | Is authentication required to perform this operation |  | [default to false]

### Return type

[**crate::models::PartitionedTopicMetadata**](PartitionedTopicMetadata.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_partitioned_stats

> get_partitioned_stats(tenant, namespace, topic, authoritative)
Get the stats for the partitioned topic.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** | Specify the tenant | Required | 
**namespace** | **String** | Specify the namespace | Required | 
**topic** | **String** | Specify topic name | Required | 
**authoritative** | **bool** | Is authentication required to perform this operation |  | [default to false]

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_partitioned_topic_list

> Vec<String> get_partitioned_topic_list(tenant, namespace)
Get the list of partitioned topics under a namespace.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** | Specify the tenant | Required | 
**namespace** | **String** | Specify the namespace | Required | 

### Return type

**Vec<String>**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_permissions_on_topic

> ::std::collections::HashMap<String, serde_json::Value> get_permissions_on_topic(tenant, namespace, topic)
Get permissions on a topic.

Retrieve the effective permissions for a topic. These permissions are defined by the permissions set at thenamespace level combined (union) with any eventual specific permission set on the topic.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** | Specify the tenant | Required | 
**namespace** | **String** | Specify the namespace | Required | 
**topic** | **String** | Specify topic name | Required | 

### Return type

[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_stats

> crate::models::TopicStats get_stats(tenant, namespace, topic, authoritative)
Get the stats for the topic.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** | Specify the tenant | Required | 
**namespace** | **String** | Specify the namespace | Required | 
**topic** | **String** | Specify topic name | Required | 
**authoritative** | **bool** | Is authentication required to perform this operation |  | [default to false]

### Return type

[**crate::models::TopicStats**](TopicStats.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_subscriptions

> Vec<serde_json::Value> get_subscriptions(tenant, namespace, topic, authoritative)
Get the list of persistent subscriptions for a given topic.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** | Specify the tenant | Required | 
**namespace** | **String** | Specify the namespace | Required | 
**topic** | **String** | Specify topic name | Required | 
**authoritative** | **bool** | Is authentication required to perform this operation |  | [default to false]

### Return type

[**Vec<serde_json::Value>**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## grant_permissions_on_topic

> grant_permissions_on_topic(tenant, namespace, topic, role, body)
Grant a new permission to a role on a single topic.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** | Specify the tenant | Required | 
**namespace** | **String** | Specify the namespace | Required | 
**topic** | **String** | Specify topic name | Required | 
**role** | **String** | Client role to which grant permissions | Required | 
**body** | [**Vec<String>**](String.md) | Actions to be granted (produce,functions,consume) |  | 

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## offload_status

> crate::models::OffloadProcessStatus offload_status(tenant, namespace, topic, authoritative)
Offload a prefix of a topic to long term storage

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** | Specify the tenant | Required | 
**namespace** | **String** | Specify the namespace | Required | 
**topic** | **String** | Specify topic name | Required | 
**authoritative** | **bool** | Is authentication required to perform this operation |  | [default to false]

### Return type

[**crate::models::OffloadProcessStatus**](OffloadProcessStatus.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## peek_nth_message

> peek_nth_message(tenant, namespace, topic, sub_name, message_position, authoritative)
Peek nth message on a topic subscription.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** | Specify the tenant | Required | 
**namespace** | **String** | Specify the namespace | Required | 
**topic** | **String** | Specify topic name | Required | 
**sub_name** | **String** | Subscribed message expired | Required | 
**message_position** | **i32** | The number of messages (default 1) | Required | [default to 1]
**authoritative** | **bool** | Is authentication required to perform this operation |  | [default to false]

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## reset_cursor

> reset_cursor(tenant, namespace, topic, sub_name, timestamp, authoritative)
Reset subscription to message position closest to absolute timestamp (in ms).

It fence cursor and disconnects all active consumers before reseting cursor.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** | Specify the tenant | Required | 
**namespace** | **String** | Specify the namespace | Required | 
**topic** | **String** | Specify topic name | Required | 
**sub_name** | **String** | Subscription to reset position on | Required | 
**timestamp** | **i64** | time in minutes to reset back to (or minutes, hours,days,weeks eg:100m, 3h, 2d, 5w) | Required | 
**authoritative** | **bool** | Is authentication required to perform this operation |  | [default to false]

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## reset_cursor_on_position

> reset_cursor_on_position(tenant, namespace, topic, sub_name, authoritative, message_id)
Reset subscription to message position closest to given position.

It fence cursor and disconnects all active consumers before reseting cursor.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** | Specify the tenant | Required | 
**namespace** | **String** | Specify the namespace | Required | 
**topic** | **String** | Specify topic name | Required | 
**sub_name** | **String** | Subscription to reset position on | Required | 
**authoritative** | **bool** | Is authentication required to perform this operation |  | [default to false]
**message_id** | [**MessageIdImpl**](MessageIdImpl.md) | messageId to reset back to (ledgerId:entryId) |  | 

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## revoke_permissions_on_topic

> revoke_permissions_on_topic(tenant, namespace, topic, role)
Revoke permissions on a topic.

Revoke permissions to a role on a single topic. If the permission was not set at the topiclevel, but rather at the namespace level, this operation will return an error (HTTP status code 412).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** | Specify the tenant | Required | 
**namespace** | **String** | Specify the namespace | Required | 
**topic** | **String** | Specify topic name | Required | 
**role** | **String** | Client role to which grant permissions | Required | 

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## skip_all_messages

> skip_all_messages(tenant, namespace, topic, sub_name, authoritative)
Skip all messages on a topic subscription.

Completely clears the backlog on the subscription.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** | Specify the tenant | Required | 
**namespace** | **String** | Specify the namespace | Required | 
**topic** | **String** | Specify topic name | Required | 
**sub_name** | **String** | Name of subscription | Required | 
**authoritative** | **bool** | Is authentication required to perform this operation |  | [default to false]

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## skip_messages

> skip_messages(tenant, namespace, topic, sub_name, num_messages, authoritative)
Skipping messages on a topic subscription.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** | Specify the tenant | Required | 
**namespace** | **String** | Specify the namespace | Required | 
**topic** | **String** | Specify topic name | Required | 
**sub_name** | **String** | Name of subscription | Required | 
**num_messages** | **i32** | The number of messages to skip | Required | [default to 0]
**authoritative** | **bool** | Is authentication required to perform this operation |  | [default to false]

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## terminate

> serde_json::Value terminate(tenant, namespace, topic, authoritative)
Terminate a topic. A topic that is terminated will not accept any more messages to be published and will let consumer to drain existing messages in backlog

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** | Specify the tenant | Required | 
**namespace** | **String** | Specify the namespace | Required | 
**topic** | **String** | Specify topic name | Required | 
**authoritative** | **bool** | Is authentication required to perform this operation |  | [default to false]

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## trigger_offload

> trigger_offload(tenant, namespace, topic, authoritative)
Offload a prefix of a topic to long term storage

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** | Specify the tenant | Required | 
**namespace** | **String** | Specify the namespace | Required | 
**topic** | **String** | Specify topic name | Required | 
**authoritative** | **bool** | Is authentication required to perform this operation |  | [default to false]

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## unload_topic

> unload_topic(tenant, namespace, topic, authoritative)
Unload a topic

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** | Specify the tenant | Required | 
**namespace** | **String** | Specify the namespace | Required | 
**topic** | **String** | Specify topic name | Required | 
**authoritative** | **bool** | Is authentication required to perform this operation |  | [default to false]

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_partitioned_topic

> update_partitioned_topic(tenant, namespace, topic, body)
Increment partitons of an existing partitioned topic.

It only increments partitions of existing non-global partitioned-topic

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** | Specify the tenant | Required | 
**namespace** | **String** | Specify the namespace | Required | 
**topic** | **String** | Specify topic name | Required | 
**body** | **i32** | The number of partitions for the topic | Required | 

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

