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
pub struct PaginatedResultResourceRoleRead {
    /// List of Resource Roles
    #[serde(rename = "data")]
    pub data: Vec<models::ResourceRoleRead>,
    #[serde(rename = "total_count")]
    pub total_count: i32,
    #[serde(rename = "page_count", skip_serializing_if = "Option::is_none")]
    pub page_count: Option<i32>,
}

impl PaginatedResultResourceRoleRead {
    pub fn new(data: Vec<models::ResourceRoleRead>, total_count: i32) -> PaginatedResultResourceRoleRead {
        PaginatedResultResourceRoleRead {
            data,
            total_count,
            page_count: None,
        }
    }
}

