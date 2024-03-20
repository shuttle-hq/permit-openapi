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

/// EmailTemplateType : An enumeration.
/// An enumeration.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum EmailTemplateType {
    #[serde(rename = "approval_flows")]
    ApprovalFlows,
    #[serde(rename = "user_management")]
    UserManagement,
    #[serde(rename = "test_email")]
    TestEmail,

}

impl ToString for EmailTemplateType {
    fn to_string(&self) -> String {
        match self {
            Self::ApprovalFlows => String::from("approval_flows"),
            Self::UserManagement => String::from("user_management"),
            Self::TestEmail => String::from("test_email"),
        }
    }
}

impl Default for EmailTemplateType {
    fn default() -> EmailTemplateType {
        Self::ApprovalFlows
    }
}

