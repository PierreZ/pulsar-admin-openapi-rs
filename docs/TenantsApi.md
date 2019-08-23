# \TenantsApi

All URIs are relative to *http://localhost/admin/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_tenant**](TenantsApi.md#create_tenant) | **Put** /tenants/{tenant} | Create a new tenant.
[**delete_tenant**](TenantsApi.md#delete_tenant) | **Delete** /tenants/{tenant} | Delete a tenant and all namespaces and topics under it.
[**get_tenant_admin**](TenantsApi.md#get_tenant_admin) | **Get** /tenants/{tenant} | Get the admin configuration for a given tenant.
[**get_tenants**](TenantsApi.md#get_tenants) | **Get** /tenants | Get the list of existing tenants.
[**update_tenant**](TenantsApi.md#update_tenant) | **Post** /tenants/{tenant} | Update the admins for a tenant.



## create_tenant

> create_tenant(tenant, body)
Create a new tenant.

This operation requires Pulsar super-user privileges.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** | The tenant name | Required | 
**body** | [**TenantInfo**](TenantInfo.md) | TenantInfo |  | 

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_tenant

> delete_tenant(tenant)
Delete a tenant and all namespaces and topics under it.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** | The tenant name | Required | 

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_tenant_admin

> crate::models::TenantInfo get_tenant_admin(tenant)
Get the admin configuration for a given tenant.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** | The tenant name | Required | 

### Return type

[**crate::models::TenantInfo**](TenantInfo.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_tenants

> Vec<String> get_tenants()
Get the list of existing tenants.

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


## update_tenant

> update_tenant(tenant, body)
Update the admins for a tenant.

This operation requires Pulsar super-user privileges.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** | The tenant name | Required | 
**body** | [**TenantInfo**](TenantInfo.md) | TenantInfo |  | 

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

