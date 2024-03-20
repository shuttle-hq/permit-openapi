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
pub struct AccessRequestCreateDetails {
    /// tenant id or key that the user is requesting access to
    #[serde(rename = "tenant")]
    pub tenant: String,
    /// resource id or key that the user is requesting access to
    #[serde(rename = "resource", skip_serializing_if = "Option::is_none")]
    pub resource: Option<String>,
    /// resource instance id or key that the user is requesting access to
    #[serde(rename = "resource_instance", skip_serializing_if = "Option::is_none")]
    pub resource_instance: Option<String>,
    /// role id or key that the user is requesting access to
    #[serde(rename = "role")]
    pub role: String,
}

impl AccessRequestCreateDetails {
    pub fn new(tenant: String, role: String) -> AccessRequestCreateDetails {
        AccessRequestCreateDetails {
            tenant,
            resource: None,
            resource_instance: None,
            role,
        }
    }
}
