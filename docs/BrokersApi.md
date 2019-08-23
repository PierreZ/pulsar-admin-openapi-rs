# \BrokersApi

All URIs are relative to *http://localhost/admin/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_active_brokers**](BrokersApi.md#get_active_brokers) | **Get** /brokers/{cluster} | Get the list of active brokers (web service addresses) in the cluster.If authorization is not enabled, any cluster name is valid.
[**get_all_dynamic_configurations**](BrokersApi.md#get_all_dynamic_configurations) | **Get** /brokers/configuration/values | Get value of all dynamic configurations' value overridden on local config
[**get_dynamic_configuration_name**](BrokersApi.md#get_dynamic_configuration_name) | **Get** /brokers/configuration | Get all updatable dynamic configurations's name
[**get_internal_configuration_data**](BrokersApi.md#get_internal_configuration_data) | **Get** /brokers/internal-configuration | Get the internal configuration data
[**get_owned_namespaes**](BrokersApi.md#get_owned_namespaes) | **Get** /brokers/{clusterName}/{broker-webserviceurl}/ownedNamespaces | Get the list of namespaces served by the specific broker
[**get_runtime_configuration**](BrokersApi.md#get_runtime_configuration) | **Get** /brokers/configuration/runtime | Get all runtime configurations. This operation requires Pulsar super-user privileges.
[**healthcheck**](BrokersApi.md#healthcheck) | **Get** /brokers/health | Run a healthcheck against the broker
[**update_dynamic_configuration**](BrokersApi.md#update_dynamic_configuration) | **Post** /brokers/configuration/{configName}/{configValue} | Update dynamic serviceconfiguration into zk only. This operation requires Pulsar super-user privileges.



## get_active_brokers

> Vec<String> get_active_brokers(cluster)
Get the list of active brokers (web service addresses) in the cluster.If authorization is not enabled, any cluster name is valid.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cluster** | **String** |  | Required | 

### Return type

**Vec<String>**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_all_dynamic_configurations

> ::std::collections::HashMap<String, serde_json::Value> get_all_dynamic_configurations()
Get value of all dynamic configurations' value overridden on local config

### Parameters

This endpoint does not need any parameter.

### Return type

[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_dynamic_configuration_name

> Vec<serde_json::Value> get_dynamic_configuration_name()
Get all updatable dynamic configurations's name

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<serde_json::Value>**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_internal_configuration_data

> crate::models::InternalConfigurationData get_internal_configuration_data()
Get the internal configuration data

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::InternalConfigurationData**](InternalConfigurationData.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_owned_namespaes

> ::std::collections::HashMap<String, crate::models::NamespaceOwnershipStatus> get_owned_namespaes(cluster_name, broker_webserviceurl)
Get the list of namespaces served by the specific broker

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cluster_name** | **String** |  | Required | 
**broker_webserviceurl** | **String** |  | Required | 

### Return type

[**::std::collections::HashMap<String, crate::models::NamespaceOwnershipStatus>**](NamespaceOwnershipStatus.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_runtime_configuration

> ::std::collections::HashMap<String, serde_json::Value> get_runtime_configuration()
Get all runtime configurations. This operation requires Pulsar super-user privileges.

### Parameters

This endpoint does not need any parameter.

### Return type

[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## healthcheck

> healthcheck()
Run a healthcheck against the broker

### Parameters

This endpoint does not need any parameter.

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_dynamic_configuration

> update_dynamic_configuration(config_name, config_value)
Update dynamic serviceconfiguration into zk only. This operation requires Pulsar super-user privileges.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**config_name** | **String** |  | Required | 
**config_value** | **String** |  | Required | 

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

