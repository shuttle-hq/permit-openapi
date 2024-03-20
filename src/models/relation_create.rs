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
pub struct RelationCreate {
    /// A URL-friendly name of the relation (i.e: slug). You will be able to query later using this key instead of the id (UUID) of the relation.
    #[serde(rename = "key")]
    pub key: String,
    /// The name of the relation
    #[serde(rename = "name")]
    pub name: String,
    /// An optional longer description of what this relation represents in your system
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The subject resource ID or key
    #[serde(rename = "subject_resource")]
    pub subject_resource: String,
}

impl RelationCreate {
    pub fn new(key: String, name: String, subject_resource: String) -> RelationCreate {
        RelationCreate {
            key,
            name,
            description: None,
            subject_resource,
        }
    }
}
