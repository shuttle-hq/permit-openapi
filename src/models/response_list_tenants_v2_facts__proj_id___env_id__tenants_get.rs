/*
 * Permit.io API
 *
 *  Authorization as a service 
 *
 * The version of the OpenAPI document: 2.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */

use crate::models;

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ResponseListTenantsV2FactsProjIdEnvIdTenantsGet {
    /// List of Tenants
    #[serde(rename = "data", deserialize_with = "Option::deserialize")]
    pub data: Option<serde_json::Value>,
    #[serde(rename = "total_count", deserialize_with = "Option::deserialize")]
    pub total_count: Option<serde_json::Value>,
    #[serde(rename = "page_count", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub page_count: Option<Option<serde_json::Value>>,
}

impl ResponseListTenantsV2FactsProjIdEnvIdTenantsGet {
    pub fn new(data: Option<serde_json::Value>, total_count: Option<serde_json::Value>) -> ResponseListTenantsV2FactsProjIdEnvIdTenantsGet {
        ResponseListTenantsV2FactsProjIdEnvIdTenantsGet {
            data,
            total_count,
            page_count: None,
        }
    }
}
