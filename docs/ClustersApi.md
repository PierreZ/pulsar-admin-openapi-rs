# \ClustersApi

All URIs are relative to *http://localhost/admin/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_cluster**](ClustersApi.md#create_cluster) | **Put** /clusters/{cluster} | Create a new cluster.
[**delete_cluster**](ClustersApi.md#delete_cluster) | **Delete** /clusters/{cluster} | Delete an existing cluster.
[**delete_failure_domain**](ClustersApi.md#delete_failure_domain) | **Delete** /clusters/{cluster}/failureDomains/{domainName} | Delete the failure domain of the cluster
[**delete_namespace_isolation_policy**](ClustersApi.md#delete_namespace_isolation_policy) | **Delete** /clusters/{cluster}/namespaceIsolationPolicies/{policyName} | Delete namespace isolation policy.
[**get_broker_with_namespace_isolation_policy**](ClustersApi.md#get_broker_with_namespace_isolation_policy) | **Get** /clusters/{cluster}/namespaceIsolationPolicies/brokers/{broker} | Get a broker with namespace-isolation policies attached to it.
[**get_brokers_with_namespace_isolation_policy**](ClustersApi.md#get_brokers_with_namespace_isolation_policy) | **Get** /clusters/{cluster}/namespaceIsolationPolicies/brokers | Get list of brokers with namespace-isolation policies attached to them.
[**get_cluster**](ClustersApi.md#get_cluster) | **Get** /clusters/{cluster} | Get the configuration for the specified cluster.
[**get_clusters**](ClustersApi.md#get_clusters) | **Get** /clusters | Get the list of all the Pulsar clusters.
[**get_domain**](ClustersApi.md#get_domain) | **Get** /clusters/{cluster}/failureDomains/{domainName} | Get a domain in a cluster
[**get_failure_domains**](ClustersApi.md#get_failure_domains) | **Get** /clusters/{cluster}/failureDomains | Get the cluster failure domains.
[**get_namespace_isolation_policies**](ClustersApi.md#get_namespace_isolation_policies) | **Get** /clusters/{cluster}/namespaceIsolationPolicies | Get the namespace isolation policies assigned to the cluster.
[**get_namespace_isolation_policy**](ClustersApi.md#get_namespace_isolation_policy) | **Get** /clusters/{cluster}/namespaceIsolationPolicies/{policyName} | Get the single namespace isolation policy assigned to the cluster.
[**get_peer_cluster**](ClustersApi.md#get_peer_cluster) | **Get** /clusters/{cluster}/peers | Get the peer-cluster data for the specified cluster.
[**set_failure_domain**](ClustersApi.md#set_failure_domain) | **Post** /clusters/{cluster}/failureDomains/{domainName} | Set the failure domain of the cluster.
[**set_namespace_isolation_policy**](ClustersApi.md#set_namespace_isolation_policy) | **Post** /clusters/{cluster}/namespaceIsolationPolicies/{policyName} | Set namespace isolation policy.
[**set_peer_cluster_names**](ClustersApi.md#set_peer_cluster_names) | **Post** /clusters/{cluster}/peers | Update peer-cluster-list for a cluster.
[**update_cluster**](ClustersApi.md#update_cluster) | **Post** /clusters/{cluster} | Update the configuration for a cluster.



## create_cluster

> create_cluster(cluster, body)
Create a new cluster.

This operation requires Pulsar superuser privileges, and the name cannot contain the '/' characters.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cluster** | **String** | The cluster name | Required | 
**body** | [**ClusterData**](ClusterData.md) | The cluster data | Required | 

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_cluster

> delete_cluster(cluster)
Delete an existing cluster.

This operation requires Pulsar superuser privileges.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cluster** | **String** | The cluster name | Required | 

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_failure_domain

> delete_failure_domain(cluster, domain_name)
Delete the failure domain of the cluster

This operation requires Pulsar superuser privileges.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cluster** | **String** | The cluster name | Required | 
**domain_name** | **String** | The failure domain name | Required | 

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_namespace_isolation_policy

> delete_namespace_isolation_policy(cluster, policy_name)
Delete namespace isolation policy.

This operation requires Pulsar superuser privileges.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cluster** | **String** | The cluster name | Required | 
**policy_name** | **String** | The namespace isolation policy name | Required | 

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_broker_with_namespace_isolation_policy

> crate::models::BrokerNamespaceIsolationData get_broker_with_namespace_isolation_policy(cluster, broker)
Get a broker with namespace-isolation policies attached to it.

This operation requires Pulsar superuser privileges.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cluster** | **String** | The cluster name | Required | 
**broker** | **String** | The broker name (<broker-hostname>:<web-service-port>) | Required | 

### Return type

[**crate::models::BrokerNamespaceIsolationData**](BrokerNamespaceIsolationData.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_brokers_with_namespace_isolation_policy

> Vec<crate::models::BrokerNamespaceIsolationData> get_brokers_with_namespace_isolation_policy(cluster)
Get list of brokers with namespace-isolation policies attached to them.

This operation requires Pulsar superuser privileges.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cluster** | **String** | The cluster name | Required | 

### Return type

[**Vec<crate::models::BrokerNamespaceIsolationData>**](BrokerNamespaceIsolationData.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_cluster

> crate::models::ClusterData get_cluster(cluster)
Get the configuration for the specified cluster.

This operation requires Pulsar superuser privileges.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cluster** | **String** | The cluster name | Required | 

### Return type

[**crate::models::ClusterData**](ClusterData.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_clusters

> Vec<String> get_clusters()
Get the list of all the Pulsar clusters.

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


## get_domain

> crate::models::FailureDomain get_domain(cluster, domain_name)
Get a domain in a cluster

This operation requires Pulsar superuser privileges.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cluster** | **String** | The cluster name | Required | 
**domain_name** | **String** | The failure domain name | Required | 

### Return type

[**crate::models::FailureDomain**](FailureDomain.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_failure_domains

> ::std::collections::HashMap<String, crate::models::FailureDomain> get_failure_domains(cluster)
Get the cluster failure domains.

This operation requires Pulsar superuser privileges.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cluster** | **String** | The cluster name | Required | 

### Return type

[**::std::collections::HashMap<String, crate::models::FailureDomain>**](FailureDomain.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_namespace_isolation_policies

> ::std::collections::HashMap<String, crate::models::NamespaceIsolationData> get_namespace_isolation_policies(cluster)
Get the namespace isolation policies assigned to the cluster.

This operation requires Pulsar superuser privileges.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cluster** | **String** | The cluster name | Required | 

### Return type

[**::std::collections::HashMap<String, crate::models::NamespaceIsolationData>**](NamespaceIsolationData.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_namespace_isolation_policy

> crate::models::NamespaceIsolationData get_namespace_isolation_policy(cluster, policy_name)
Get the single namespace isolation policy assigned to the cluster.

This operation requires Pulsar superuser privileges.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cluster** | **String** | The cluster name | Required | 
**policy_name** | **String** | The name of the namespace isolation policy | Required | 

### Return type

[**crate::models::NamespaceIsolationData**](NamespaceIsolationData.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_peer_cluster

> Vec<String> get_peer_cluster(cluster)
Get the peer-cluster data for the specified cluster.

This operation requires Pulsar superuser privileges.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cluster** | **String** | The cluster name | Required | 

### Return type

**Vec<String>**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## set_failure_domain

> set_failure_domain(cluster, domain_name, body)
Set the failure domain of the cluster.

This operation requires Pulsar superuser privileges.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cluster** | **String** | The cluster name | Required | 
**domain_name** | **String** | The failure domain name | Required | 
**body** | [**FailureDomain**](FailureDomain.md) | The configuration data of a failure domain | Required | 

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## set_namespace_isolation_policy

> set_namespace_isolation_policy(cluster, policy_name, body)
Set namespace isolation policy.

This operation requires Pulsar superuser privileges.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cluster** | **String** | The cluster name | Required | 
**policy_name** | **String** | The namespace isolation policy name | Required | 
**body** | [**NamespaceIsolationData**](NamespaceIsolationData.md) | The namespace isolation policy data | Required | 

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## set_peer_cluster_names

> set_peer_cluster_names(cluster, body)
Update peer-cluster-list for a cluster.

This operation requires Pulsar superuser privileges.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cluster** | **String** | The cluster name | Required | 
**body** | [**Vec<String>**](String.md) | The list of peer cluster names | Required | 

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_cluster

> update_cluster(cluster, body)
Update the configuration for a cluster.

This operation requires Pulsar superuser privileges.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cluster** | **String** | The cluster name | Required | 
**body** | [**ClusterData**](ClusterData.md) | The cluster data | Required | 

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

