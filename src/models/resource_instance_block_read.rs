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
pub struct ResourceInstanceBlockRead {
    /// A unique identifier by which Permit will identify the resource instance for permission checks. You will later pass this identifier to the `permit.check()` API. A key can be anything: for example the resource db id, a url slug, a UUID or anything else as long as it's unique on your end. The resource instance key must be url-friendly.
    #[serde(rename = "key")]
    pub key: String,
    /// the *key* of the tenant that this resource belongs to, used to enforce tenant boundaries in multi-tenant apps.
    #[serde(rename = "tenant")]
    pub tenant: String,
    /// the *key* of the resource (type) of this resource instance. For example: if this resource instance is the annual budget document, the key of the resource might be `document`.
    #[serde(rename = "resource")]
    pub resource: String,
    /// Arbitrary resource attributes that will be used to enforce attribute-based access control policies.
    #[serde(rename = "attributes", skip_serializing_if = "Option::is_none")]
    pub attributes: Option<serde_json::Value>,
}

impl ResourceInstanceBlockRead {
    pub fn new(key: String, tenant: String, resource: String) -> ResourceInstanceBlockRead {
        ResourceInstanceBlockRead {
            key,
            tenant,
            resource,
            attributes: None,
        }
    }
}

