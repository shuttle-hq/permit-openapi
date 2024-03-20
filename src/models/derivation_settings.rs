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

/// DerivationSettings : Settings for a derived role or a derived role rule
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct DerivationSettings {
    /// If true, the derived role or the specific rule will not apply if the resource has any direct role
    #[serde(rename = "no_direct_roles_on_object", skip_serializing_if = "Option::is_none")]
    pub no_direct_roles_on_object: Option<bool>,
}

impl DerivationSettings {
    /// Settings for a derived role or a derived role rule
    pub fn new() -> DerivationSettings {
        DerivationSettings {
            no_direct_roles_on_object: None,
        }
    }
}
