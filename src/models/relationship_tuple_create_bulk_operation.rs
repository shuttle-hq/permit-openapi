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
pub struct RelationshipTupleCreateBulkOperation {
    #[serde(rename = "operations")]
    pub operations: Vec<models::RelationshipTupleCreate>,
}

impl RelationshipTupleCreateBulkOperation {
    pub fn new(operations: Vec<models::RelationshipTupleCreate>) -> RelationshipTupleCreateBulkOperation {
        RelationshipTupleCreateBulkOperation {
            operations,
        }
    }
}

