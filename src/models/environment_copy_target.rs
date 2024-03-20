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
pub struct EnvironmentCopyTarget {
    /// Identifier of an existing environment to copy into
    #[serde(rename = "existing", skip_serializing_if = "Option::is_none")]
    pub existing: Option<String>,
    /// Description of the environment to create. This environment must not already exist.
    #[serde(rename = "new", skip_serializing_if = "Option::is_none")]
    pub new: Option<Box<models::EnvironmentCreate>>,
}

impl EnvironmentCopyTarget {
    pub fn new() -> EnvironmentCopyTarget {
        EnvironmentCopyTarget {
            existing: None,
            new: None,
        }
    }
}

