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
pub struct GetSchemaResponse {
    #[serde(rename = "version", skip_serializing_if = "Option::is_none")]
    pub version: Option<i64>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub _type: Option<Type>,
    #[serde(rename = "timestamp", skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<i64>,
    #[serde(rename = "data", skip_serializing_if = "Option::is_none")]
    pub data: Option<String>,
    #[serde(rename = "properties", skip_serializing_if = "Option::is_none")]
    pub properties: Option<::std::collections::HashMap<String, String>>,
}

impl GetSchemaResponse {
    pub fn new() -> GetSchemaResponse {
        GetSchemaResponse {
            version: None,
            _type: None,
            timestamp: None,
            data: None,
            properties: None,
        }
    }
}

/// 
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "NONE")]
    NONE,
    #[serde(rename = "STRING")]
    STRING,
    #[serde(rename = "JSON")]
    JSON,
    #[serde(rename = "PROTOBUF")]
    PROTOBUF,
    #[serde(rename = "AVRO")]
    AVRO,
    #[serde(rename = "BOOLEAN")]
    BOOLEAN,
    #[serde(rename = "INT8")]
    INT8,
    #[serde(rename = "INT16")]
    INT16,
    #[serde(rename = "INT32")]
    INT32,
    #[serde(rename = "INT64")]
    INT64,
    #[serde(rename = "FLOAT")]
    FLOAT,
    #[serde(rename = "DOUBLE")]
    DOUBLE,
    #[serde(rename = "DATE")]
    DATE,
    #[serde(rename = "TIME")]
    TIME,
    #[serde(rename = "TIMESTAMP")]
    TIMESTAMP,
    #[serde(rename = "KEY_VALUE")]
    KEYVALUE,
    #[serde(rename = "BYTES")]
    BYTES,
    #[serde(rename = "AUTO")]
    AUTO,
    #[serde(rename = "AUTO_CONSUME")]
    AUTOCONSUME,
    #[serde(rename = "AUTO_PUBLISH")]
    AUTOPUBLISH,
}

