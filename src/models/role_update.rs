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
pub struct RoleUpdate {
    /// The name of the role
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// optional description string explaining what this role represents, or what permissions are granted to it.
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// list of action keys that define what actions this resource role is permitted to do
    #[serde(rename = "permissions", skip_serializing_if = "Option::is_none")]
    pub permissions: Option<Vec<String>>,
    /// optional dictionary of key-value pairs that can be used to store arbitrary metadata about this role. This metadata can be used to filter role using query parameters with attr_ prefix, currently supports only 'equals' operator
    #[serde(rename = "attributes", skip_serializing_if = "Option::is_none")]
    pub attributes: Option<serde_json::Value>,
    /// list of role keys that define what roles this role extends. In other words: this role will automatically inherit all the permissions of the given roles in this list.
    #[serde(rename = "extends", skip_serializing_if = "Option::is_none")]
    pub extends: Option<Vec<String>>,
    /// Derived role that inherit will be applied on this role
    #[serde(rename = "granted_to", skip_serializing_if = "Option::is_none")]
    pub granted_to: Option<Box<models::DerivedRoleBlockEdit>>,
}

impl RoleUpdate {
    pub fn new() -> RoleUpdate {
        RoleUpdate {
            name: None,
            description: None,
            permissions: None,
            attributes: None,
            extends: None,
            granted_to: None,
        }
    }
}

