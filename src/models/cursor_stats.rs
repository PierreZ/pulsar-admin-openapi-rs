/*
 * Pulsar Admin REST API
 *
 * This provides the REST API for admin operations
 *
 * The version of the OpenAPI document: v2
 * 
 * Generated by: https://openapi-generator.tech
 */



#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct CursorStats {
    #[serde(rename = "markDeletePosition", skip_serializing_if = "Option::is_none")]
    pub mark_delete_position: Option<String>,
    #[serde(rename = "readPosition", skip_serializing_if = "Option::is_none")]
    pub read_position: Option<String>,
    #[serde(rename = "waitingReadOp", skip_serializing_if = "Option::is_none")]
    pub waiting_read_op: Option<bool>,
    #[serde(rename = "pendingReadOps", skip_serializing_if = "Option::is_none")]
    pub pending_read_ops: Option<i32>,
    #[serde(rename = "messagesConsumedCounter", skip_serializing_if = "Option::is_none")]
    pub messages_consumed_counter: Option<i64>,
    #[serde(rename = "cursorLedger", skip_serializing_if = "Option::is_none")]
    pub cursor_ledger: Option<i64>,
    #[serde(rename = "cursorLedgerLastEntry", skip_serializing_if = "Option::is_none")]
    pub cursor_ledger_last_entry: Option<i64>,
    #[serde(rename = "individuallyDeletedMessages", skip_serializing_if = "Option::is_none")]
    pub individually_deleted_messages: Option<String>,
    #[serde(rename = "lastLedgerSwitchTimestamp", skip_serializing_if = "Option::is_none")]
    pub last_ledger_switch_timestamp: Option<String>,
    #[serde(rename = "state", skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(rename = "numberOfEntriesSinceFirstNotAckedMessage", skip_serializing_if = "Option::is_none")]
    pub number_of_entries_since_first_not_acked_message: Option<i64>,
    #[serde(rename = "totalNonContiguousDeletedMessagesRange", skip_serializing_if = "Option::is_none")]
    pub total_non_contiguous_deleted_messages_range: Option<i32>,
    #[serde(rename = "properties", skip_serializing_if = "Option::is_none")]
    pub properties: Option<::std::collections::HashMap<String, i64>>,
}

impl CursorStats {
    pub fn new() -> CursorStats {
        CursorStats {
            mark_delete_position: None,
            read_position: None,
            waiting_read_op: None,
            pending_read_ops: None,
            messages_consumed_counter: None,
            cursor_ledger: None,
            cursor_ledger_last_entry: None,
            individually_deleted_messages: None,
            last_ledger_switch_timestamp: None,
            state: None,
            number_of_entries_since_first_not_acked_message: None,
            total_non_contiguous_deleted_messages_range: None,
            properties: None,
        }
    }
}


