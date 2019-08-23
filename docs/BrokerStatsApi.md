# \BrokerStatsApi

All URIs are relative to *http://localhost/admin/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_allocator_stats**](BrokerStatsApi.md#get_allocator_stats) | **Get** /broker-stats/allocator-stats/{allocator} | Get the stats for the Netty allocator. Available allocators are 'default' and 'ml-cache'
[**get_broker_resource_availability**](BrokerStatsApi.md#get_broker_resource_availability) | **Get** /broker-stats/broker-resource-availability/{tenant}/{namespace} | Broker availability report
[**get_load_report**](BrokerStatsApi.md#get_load_report) | **Get** /broker-stats/load-report | Get Load for this broker
[**get_m_beans**](BrokerStatsApi.md#get_m_beans) | **Get** /broker-stats/mbeans | Get all the mbean details of this broker JVM
[**get_metrics**](BrokerStatsApi.md#get_metrics) | **Get** /broker-stats/metrics | Gets the metrics for Monitoring
[**get_pending_bookie_ops_stats**](BrokerStatsApi.md#get_pending_bookie_ops_stats) | **Get** /broker-stats/bookieops | Get pending bookie client op stats by namesapce
[**get_topics2**](BrokerStatsApi.md#get_topics2) | **Get** /broker-stats/topics | Get all the topic stats by namespace



## get_allocator_stats

> crate::models::AllocatorStats get_allocator_stats(allocator)
Get the stats for the Netty allocator. Available allocators are 'default' and 'ml-cache'

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**allocator** | **String** |  | Required | 

### Return type

[**crate::models::AllocatorStats**](AllocatorStats.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_broker_resource_availability

> ::std::collections::HashMap<String, crate::models::ResourceUnit> get_broker_resource_availability(tenant, namespace)
Broker availability report

This API gives the current broker availability in percent, each resource percentage usage is calculated and thensum of all of the resource usage percent is called broker-resource-availability<br/><br/>THIS API IS ONLY FOR USE BY TESTING FOR CONFIRMING NAMESPACE ALLOCATION ALGORITHM

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** |  | Required | 
**namespace** | **String** |  | Required | 

### Return type

[**::std::collections::HashMap<String, crate::models::ResourceUnit>**](ResourceUnit.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_load_report

> crate::models::LoadReport get_load_report()
Get Load for this broker

consists of topics stats & systemResourceUsage

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::LoadReport**](LoadReport.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_m_beans

> Vec<crate::models::Metrics> get_m_beans()
Get all the mbean details of this broker JVM

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<crate::models::Metrics>**](Metrics.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_metrics

> Vec<crate::models::Metrics> get_metrics()
Gets the metrics for Monitoring

Requested should be executed by Monitoring agent on each broker to fetch the metrics

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<crate::models::Metrics>**](Metrics.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_pending_bookie_ops_stats

> ::std::collections::HashMap<String, crate::models::PendingBookieOpsStats> get_pending_bookie_ops_stats()
Get pending bookie client op stats by namesapce

### Parameters

This endpoint does not need any parameter.

### Return type

[**::std::collections::HashMap<String, crate::models::PendingBookieOpsStats>**](PendingBookieOpsStats.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_topics2

> serde_json::Value get_topics2()
Get all the topic stats by namespace

### Parameters

This endpoint does not need any parameter.

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

