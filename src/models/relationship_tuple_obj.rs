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
pub struct RelationshipTupleObj {
    #[serde(rename = "subject_str")]
    pub subject_str: String,
    #[serde(rename = "relation_str")]
    pub relation_str: String,
    #[serde(rename = "object_str")]
    pub object_str: String,
}

impl RelationshipTupleObj {
    pub fn new(subject_str: String, relation_str: String, object_str: String) -> RelationshipTupleObj {
        RelationshipTupleObj {
            subject_str,
            relation_str,
            object_str,
        }
    }
}
