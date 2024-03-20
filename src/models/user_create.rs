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
pub struct UserCreate {
    /// A unique id by which Permit will identify the user for permission checks.
    #[serde(rename = "key")]
    pub key: String,
    /// The email of the user. If synced, will be unique inside the environment.
    #[serde(rename = "email", skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// First name of the user.
    #[serde(rename = "first_name", skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,
    /// Last name of the user.
    #[serde(rename = "last_name", skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    /// Arbitrary user attributes that will be used to enforce attribute-based access control policies.
    #[serde(rename = "attributes", skip_serializing_if = "Option::is_none")]
    pub attributes: Option<serde_json::Value>,
}

impl UserCreate {
    pub fn new(key: String) -> UserCreate {
        UserCreate {
            key,
            email: None,
            first_name: None,
            last_name: None,
            attributes: None,
        }
    }
}

