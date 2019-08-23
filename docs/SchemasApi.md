# \SchemasApi

All URIs are relative to *http://localhost/admin/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_schema**](SchemasApi.md#delete_schema) | **Delete** /schemas/{tenant}/{namespace}/{topic}/schema | Delete the schema of a topic
[**get_schema**](SchemasApi.md#get_schema) | **Get** /schemas/{tenant}/{namespace}/{topic}/schema | Get the schema of a topic
[**get_schema_0**](SchemasApi.md#get_schema_0) | **Get** /schemas/{tenant}/{namespace}/{topic}/schema/{version} | Get the schema of a topic at a given version
[**post_schema**](SchemasApi.md#post_schema) | **Post** /schemas/{tenant}/{namespace}/{topic}/schema | Update the schema of a topic



## delete_schema

> crate::models::DeleteSchemaResponse delete_schema(tenant, namespace, topic, authoritative)
Delete the schema of a topic

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** |  | Required | 
**namespace** | **String** |  | Required | 
**topic** | **String** |  | Required | 
**authoritative** | **bool** |  |  | [default to false]

### Return type

[**crate::models::DeleteSchemaResponse**](DeleteSchemaResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_schema

> crate::models::GetSchemaResponse get_schema(tenant, namespace, topic, authoritative)
Get the schema of a topic

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** |  | Required | 
**namespace** | **String** |  | Required | 
**topic** | **String** |  | Required | 
**authoritative** | **bool** |  |  | [default to false]

### Return type

[**crate::models::GetSchemaResponse**](GetSchemaResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_schema_0

> crate::models::GetSchemaResponse get_schema_0(tenant, namespace, topic, version, authoritative)
Get the schema of a topic at a given version

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** |  | Required | 
**namespace** | **String** |  | Required | 
**topic** | **String** |  | Required | 
**version** | **String** |  | Required | 
**authoritative** | **bool** |  |  | [default to false]

### Return type

[**crate::models::GetSchemaResponse**](GetSchemaResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_schema

> crate::models::PostSchemaResponse post_schema(tenant, namespace, topic, authoritative, body)
Update the schema of a topic

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** |  | Required | 
**namespace** | **String** |  | Required | 
**topic** | **String** |  | Required | 
**authoritative** | **bool** |  |  | [default to false]
**body** | [**PostSchemaPayload**](PostSchemaPayload.md) | A JSON value presenting a schema playload. An example of the expected schema can be found down here. |  | 

### Return type

[**crate::models::PostSchemaResponse**](PostSchemaResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

