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
pub struct MappingRule {
    /// The URL to match against the request URL
    #[serde(rename = "url")]
    pub url: String,
    /// The HTTP method to match against the request method
    #[serde(rename = "http_method")]
    pub http_method: models::Methods,
    /// The resource to match against the request resource
    #[serde(rename = "resource")]
    pub resource: String,
    /// The headers to match against the request headers
    #[serde(rename = "headers", skip_serializing_if = "Option::is_none")]
    pub headers: Option<std::collections::HashMap<String, String>>,
    /// The action to match against the request action
    #[serde(rename = "action", skip_serializing_if = "Option::is_none")]
    pub action: Option<String>,
    /// The priority of the mapping rule. The higher the priority, the higher the precedence
    #[serde(rename = "priority", skip_serializing_if = "Option::is_none")]
    pub priority: Option<i32>,
}

impl MappingRule {
    pub fn new(url: String, http_method: models::Methods, resource: String) -> MappingRule {
        MappingRule {
            url,
            http_method,
            resource,
            headers: None,
            action: None,
            priority: None,
        }
    }
}
