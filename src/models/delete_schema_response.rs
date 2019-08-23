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
pub struct DeleteSchemaResponse {
    #[serde(rename = "version", skip_serializing_if = "Option::is_none")]
    pub version: Option<i64>,
}

impl DeleteSchemaResponse {
    pub fn new() -> DeleteSchemaResponse {
        DeleteSchemaResponse {
            version: None,
        }
    }
}


