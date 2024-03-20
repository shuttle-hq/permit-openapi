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
pub struct AuditLogModel {
    #[serde(rename = "id")]
    pub id: uuid::Uuid,
    #[serde(rename = "timestamp")]
    pub timestamp: String,
    #[serde(rename = "query", skip_serializing_if = "Option::is_none")]
    pub query: Option<String>,
    #[serde(rename = "user_key", skip_serializing_if = "Option::is_none")]
    pub user_key: Option<String>,
    #[serde(rename = "user_email", skip_serializing_if = "Option::is_none")]
    pub user_email: Option<String>,
    #[serde(rename = "user_name", skip_serializing_if = "Option::is_none")]
    pub user_name: Option<String>,
    #[serde(rename = "resource_type", skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
    #[serde(rename = "tenant", skip_serializing_if = "Option::is_none")]
    pub tenant: Option<String>,
    #[serde(rename = "action", skip_serializing_if = "Option::is_none")]
    pub action: Option<String>,
    #[serde(rename = "decision", skip_serializing_if = "Option::is_none")]
    pub decision: Option<bool>,
    #[serde(rename = "reason", skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    #[serde(rename = "org_id")]
    pub org_id: uuid::Uuid,
    #[serde(rename = "project_id")]
    pub project_id: uuid::Uuid,
    #[serde(rename = "env_id")]
    pub env_id: uuid::Uuid,
    #[serde(rename = "pdp_config_id")]
    pub pdp_config_id: uuid::Uuid,
    #[serde(rename = "input", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub input: Option<Option<serde_json::Value>>,
    #[serde(rename = "result", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub result: Option<Option<serde_json::Value>>,
    #[serde(rename = "context", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub context: Option<Option<serde_json::Value>>,
}

impl AuditLogModel {
    pub fn new(id: uuid::Uuid, timestamp: String, org_id: uuid::Uuid, project_id: uuid::Uuid, env_id: uuid::Uuid, pdp_config_id: uuid::Uuid) -> AuditLogModel {
        AuditLogModel {
            id,
            timestamp,
            query: None,
            user_key: None,
            user_email: None,
            user_name: None,
            resource_type: None,
            tenant: None,
            action: None,
            decision: None,
            reason: None,
            org_id,
            project_id,
            env_id,
            pdp_config_id,
            input: None,
            result: None,
            context: None,
        }
    }
}

