# PersistentTopicInternalStats

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**entries_added_counter** | **i64** |  | [optional] 
**number_of_entries** | **i64** |  | [optional] 
**total_size** | **i64** |  | [optional] 
**current_ledger_entries** | **i64** |  | [optional] 
**current_ledger_size** | **i64** |  | [optional] 
**last_ledger_created_timestamp** | **String** |  | [optional] 
**last_ledger_creation_failure_timestamp** | **String** |  | [optional] 
**waiting_cursors_count** | **i32** |  | [optional] 
**pending_add_entries_count** | **i32** |  | [optional] 
**last_confirmed_entry** | **String** |  | [optional] 
**state** | **String** |  | [optional] 
**ledgers** | [**Vec<crate::models::LedgerInfo>**](LedgerInfo.md) |  | [optional] 
**cursors** | [**::std::collections::HashMap<String, crate::models::CursorStats>**](CursorStats.md) |  | [optional] 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


