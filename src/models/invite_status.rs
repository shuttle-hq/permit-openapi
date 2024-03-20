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

/// InviteStatus : An enumeration.
/// An enumeration.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum InviteStatus {
    #[serde(rename = "pending")]
    Pending,
    #[serde(rename = "accepted")]
    Accepted,
    #[serde(rename = "failed")]
    Failed,
    #[serde(rename = "canceled")]
    Canceled,

}

impl ToString for InviteStatus {
    fn to_string(&self) -> String {
        match self {
            Self::Pending => String::from("pending"),
            Self::Accepted => String::from("accepted"),
            Self::Failed => String::from("failed"),
            Self::Canceled => String::from("canceled"),
        }
    }
}

impl Default for InviteStatus {
    fn default() -> InviteStatus {
        Self::Pending
    }
}
