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
pub struct ScopeConfigSet {
    #[serde(rename = "data", skip_serializing_if = "Option::is_none")]
    pub data: Option<Box<models::DataSourceConfig>>,
}

impl ScopeConfigSet {
    pub fn new() -> ScopeConfigSet {
        ScopeConfigSet {
            data: None,
        }
    }
}
