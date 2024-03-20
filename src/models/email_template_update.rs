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
pub struct EmailTemplateUpdate {
    /// The from address the mails will be sent from
    #[serde(rename = "from_address")]
    pub from_address: String,
    /// The redirect url the user will be redirected to after clicking the link in the email
    #[serde(rename = "redirect_to")]
    pub redirect_to: String,
    /// The time to live of the url in the email, in seconds
    #[serde(rename = "url_ttl")]
    pub url_ttl: String,
    /// The subject of the email template
    #[serde(rename = "subject")]
    pub subject: String,
    /// The messages of the email template
    #[serde(rename = "messages")]
    pub messages: Vec<models::EmailTemplateMessage>,
}

impl EmailTemplateUpdate {
    pub fn new(from_address: String, redirect_to: String, url_ttl: String, subject: String, messages: Vec<models::EmailTemplateMessage>) -> EmailTemplateUpdate {
        EmailTemplateUpdate {
            from_address,
            redirect_to,
            url_ttl,
            subject,
            messages,
        }
    }
}
