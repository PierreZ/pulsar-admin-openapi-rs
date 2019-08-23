# \ResourceQuotasApi

All URIs are relative to *http://localhost/admin/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_default_resource_quota**](ResourceQuotasApi.md#get_default_resource_quota) | **Get** /resource-quotas | Get the default quota
[**get_namespace_bundle_resource_quota**](ResourceQuotasApi.md#get_namespace_bundle_resource_quota) | **Get** /resource-quotas/{tenant}/{namespace}/{bundle} | Get resource quota of a namespace bundle.
[**remove_namespace_bundle_resource_quota**](ResourceQuotasApi.md#remove_namespace_bundle_resource_quota) | **Delete** /resource-quotas/{tenant}/{namespace}/{bundle} | Remove resource quota for a namespace.
[**set_default_resource_quota**](ResourceQuotasApi.md#set_default_resource_quota) | **Post** /resource-quotas | Set the default quota
[**set_namespace_bundle_resource_quota**](ResourceQuotasApi.md#set_namespace_bundle_resource_quota) | **Post** /resource-quotas/{tenant}/{namespace}/{bundle} | Set resource quota on a namespace.



## get_default_resource_quota

> Vec<String> get_default_resource_quota()
Get the default quota

### Parameters

This endpoint does not need any parameter.

### Return type

**Vec<String>**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_namespace_bundle_resource_quota

> crate::models::ResourceQuota get_namespace_bundle_resource_quota(tenant, namespace, bundle)
Get resource quota of a namespace bundle.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** |  | Required | 
**namespace** | **String** |  | Required | 
**bundle** | **String** |  | Required | 

### Return type

[**crate::models::ResourceQuota**](ResourceQuota.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## remove_namespace_bundle_resource_quota

> remove_namespace_bundle_resource_quota(tenant, namespace, bundle)
Remove resource quota for a namespace.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** |  | Required | 
**namespace** | **String** |  | Required | 
**bundle** | **String** |  | Required | 

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## set_default_resource_quota

> Vec<String> set_default_resource_quota()
Set the default quota

### Parameters

This endpoint does not need any parameter.

### Return type

**Vec<String>**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## set_namespace_bundle_resource_quota

> set_namespace_bundle_resource_quota(tenant, namespace, bundle)
Set resource quota on a namespace.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** |  | Required | 
**namespace** | **String** |  | Required | 
**bundle** | **String** |  | Required | 

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

