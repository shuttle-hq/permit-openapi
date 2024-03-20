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
pub struct AccessRequestUserCreate {
    /// details of the access request, including the resource and tenant
    #[serde(rename = "access_request_details")]
    pub access_request_details: Box<models::AccessRequestCreateDetails>,
    /// Optional business justification provided by the user requesting access
    #[serde(rename = "reason", skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
}

impl AccessRequestUserCreate {
    pub fn new(access_request_details: models::AccessRequestCreateDetails) -> AccessRequestUserCreate {
        AccessRequestUserCreate {
            access_request_details: Box::new(access_request_details),
            reason: None,
        }
    }
}

