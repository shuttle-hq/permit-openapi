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
pub struct TenantRead {
    /// A unique id by which Permit will identify the tenant. The tenant key must be url-friendly (slugified).
    #[serde(rename = "key")]
    pub key: String,
    /// Unique id of the tenant
    #[serde(rename = "id")]
    pub id: uuid::Uuid,
    /// Unique id of the organization that the tenant belongs to.
    #[serde(rename = "organization_id")]
    pub organization_id: uuid::Uuid,
    /// Unique id of the project that the tenant belongs to.
    #[serde(rename = "project_id")]
    pub project_id: uuid::Uuid,
    /// Unique id of the environment that the tenant belongs to.
    #[serde(rename = "environment_id")]
    pub environment_id: uuid::Uuid,
    /// Date and time when the tenant was created (ISO_8601 format).
    #[serde(rename = "created_at")]
    pub created_at: String,
    /// Date and time when the tenant was last updated/modified (ISO_8601 format).
    #[serde(rename = "updated_at")]
    pub updated_at: String,
    /// Date and time when the tenant was last active (ISO_8601 format). In other words, this is the last time a permission check was done on a resource belonging to this tenant.
    #[serde(rename = "last_action_at")]
    pub last_action_at: String,
    /// A descriptive name for the tenant
    #[serde(rename = "name")]
    pub name: String,
    /// an optional longer description of the tenant
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Arbitraty tenant attributes that will be used to enforce attribute-based access control policies.
    #[serde(rename = "attributes", skip_serializing_if = "Option::is_none")]
    pub attributes: Option<serde_json::Value>,
}

impl TenantRead {
    pub fn new(key: String, id: uuid::Uuid, organization_id: uuid::Uuid, project_id: uuid::Uuid, environment_id: uuid::Uuid, created_at: String, updated_at: String, last_action_at: String, name: String) -> TenantRead {
        TenantRead {
            key,
            id,
            organization_id,
            project_id,
            environment_id,
            created_at,
            updated_at,
            last_action_at,
            name,
            description: None,
            attributes: None,
        }
    }
}
