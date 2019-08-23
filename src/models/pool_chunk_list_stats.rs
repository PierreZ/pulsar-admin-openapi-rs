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
pub struct PoolChunkListStats {
    #[serde(rename = "minUsage", skip_serializing_if = "Option::is_none")]
    pub min_usage: Option<i32>,
    #[serde(rename = "maxUsage", skip_serializing_if = "Option::is_none")]
    pub max_usage: Option<i32>,
    #[serde(rename = "chunks", skip_serializing_if = "Option::is_none")]
    pub chunks: Option<Vec<crate::models::PoolChunkStats>>,
}

impl PoolChunkListStats {
    pub fn new() -> PoolChunkListStats {
        PoolChunkListStats {
            min_usage: None,
            max_usage: None,
            chunks: None,
        }
    }
}


