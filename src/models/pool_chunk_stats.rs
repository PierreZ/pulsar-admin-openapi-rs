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
pub struct PoolChunkStats {
    #[serde(rename = "usage", skip_serializing_if = "Option::is_none")]
    pub usage: Option<i32>,
    #[serde(rename = "chunkSize", skip_serializing_if = "Option::is_none")]
    pub chunk_size: Option<i32>,
    #[serde(rename = "freeBytes", skip_serializing_if = "Option::is_none")]
    pub free_bytes: Option<i32>,
}

impl PoolChunkStats {
    pub fn new() -> PoolChunkStats {
        PoolChunkStats {
            usage: None,
            chunk_size: None,
            free_bytes: None,
        }
    }
}

