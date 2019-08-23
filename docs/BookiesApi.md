# \BookiesApi

All URIs are relative to *http://localhost/admin/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_bookie_rack_info**](BookiesApi.md#delete_bookie_rack_info) | **Delete** /bookies/racks-info/{bookie} | Removed the rack placement information for a specific bookie in the cluster
[**get_bookie_rack_info**](BookiesApi.md#get_bookie_rack_info) | **Get** /bookies/racks-info/{bookie} | Gets the rack placement information for a specific bookie in the cluster
[**get_bookies_rack_info**](BookiesApi.md#get_bookies_rack_info) | **Get** /bookies/racks-info | Gets the rack placement information for all the bookies in the cluster
[**update_bookie_rack_info**](BookiesApi.md#update_bookie_rack_info) | **Post** /bookies/racks-info/{bookie} | Updates the rack placement information for a specific bookie in the cluster



## delete_bookie_rack_info

> delete_bookie_rack_info(bookie)
Removed the rack placement information for a specific bookie in the cluster

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**bookie** | **String** |  | Required | 

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_bookie_rack_info

> crate::models::BookieInfo get_bookie_rack_info(bookie)
Gets the rack placement information for a specific bookie in the cluster

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**bookie** | **String** |  | Required | 

### Return type

[**crate::models::BookieInfo**](BookieInfo.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_bookies_rack_info

> ::std::collections::HashMap<String, ::std::collections::HashMap<String, crate::models::BookieInfo>> get_bookies_rack_info()
Gets the rack placement information for all the bookies in the cluster

### Parameters

This endpoint does not need any parameter.

### Return type

[**::std::collections::HashMap<String, ::std::collections::HashMap<String, crate::models::BookieInfo>>**](map.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_bookie_rack_info

> update_bookie_rack_info(bookie, group)
Updates the rack placement information for a specific bookie in the cluster

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**bookie** | **String** |  | Required | 
**group** | **String** |  |  | 

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

