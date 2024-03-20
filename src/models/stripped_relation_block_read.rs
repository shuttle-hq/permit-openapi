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
pub struct StrippedRelationBlockRead {
    /// A URL-friendly name of the relation (i.e: slug). You will be able to query later using this key instead of the id (UUID) of the relation.
    #[serde(rename = "key")]
    pub key: String,
    /// The name of the relation
    #[serde(rename = "name")]
    pub name: String,
    /// An optional longer description of what this relation represents in your system
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

impl StrippedRelationBlockRead {
    pub fn new(key: String, name: String) -> StrippedRelationBlockRead {
        StrippedRelationBlockRead {
            key,
            name,
            description: None,
        }
    }
}
