# AutoFailoverPolicyData

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**policy_type** | **String** | The auto failover policy type | [optional] 
**parameters** | **::std::collections::HashMap<String, String>** | The parameters applied to the auto failover policy specified by `policy_type`. The parameters for 'min_available' are :   - 'min_limit': the limit of minimal number of available brokers in primary group before auto failover   - 'usage_threshold': the resource usage threshold. If the usage of a broker is beyond this value, it would be marked as unavailable  | [optional] 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


