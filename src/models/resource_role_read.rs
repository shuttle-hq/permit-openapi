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
pub struct ResourceRoleRead {
    /// The name of the role
    #[serde(rename = "name")]
    pub name: String,
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
    pub granted_to: Option<Box<models::DerivedRoleBlockRead>>,
    /// A URL-friendly name of the role (i.e: slug). You will be able to query later using this key instead of the id (UUID) of the role.
    #[serde(rename = "key")]
    pub key: String,
    /// Unique id of the role
    #[serde(rename = "id")]
    pub id: uuid::Uuid,
    /// Unique id of the organization that the role belongs to.
    #[serde(rename = "organization_id")]
    pub organization_id: uuid::Uuid,
    /// Unique id of the project that the role belongs to.
    #[serde(rename = "project_id")]
    pub project_id: uuid::Uuid,
    /// Unique id of the environment that the role belongs to.
    #[serde(rename = "environment_id")]
    pub environment_id: uuid::Uuid,
    /// Unique id of the resource that the role belongs to.
    #[serde(rename = "resource_id")]
    pub resource_id: uuid::Uuid,
    /// The unique resource key that the role belongs to.
    #[serde(rename = "resource")]
    pub resource: String,
    /// Date and time when the role was created (ISO_8601 format).
    #[serde(rename = "created_at")]
    pub created_at: String,
    /// Date and time when the role was last updated/modified (ISO_8601 format).
    #[serde(rename = "updated_at")]
    pub updated_at: String,
}

impl ResourceRoleRead {
    pub fn new(name: String, key: String, id: uuid::Uuid, organization_id: uuid::Uuid, project_id: uuid::Uuid, environment_id: uuid::Uuid, resource_id: uuid::Uuid, resource: String, created_at: String, updated_at: String) -> ResourceRoleRead {
        ResourceRoleRead {
            name,
            description: None,
            permissions: None,
            attributes: None,
            extends: None,
            granted_to: None,
            key,
            id,
            organization_id,
            project_id,
            environment_id,
            resource_id,
            resource,
            created_at,
            updated_at,
        }
    }
}

