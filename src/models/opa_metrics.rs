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
pub struct OpaMetrics {
    #[serde(rename = "timer_rego_input_parse_ns", skip_serializing_if = "Option::is_none")]
    pub timer_rego_input_parse_ns: Option<i32>,
    #[serde(rename = "timer_rego_query_parse_ns", skip_serializing_if = "Option::is_none")]
    pub timer_rego_query_parse_ns: Option<i32>,
    #[serde(rename = "timer_rego_query_compile_ns", skip_serializing_if = "Option::is_none")]
    pub timer_rego_query_compile_ns: Option<i32>,
    #[serde(rename = "timer_rego_query_eval_ns", skip_serializing_if = "Option::is_none")]
    pub timer_rego_query_eval_ns: Option<i32>,
    #[serde(rename = "timer_rego_module_parse_ns", skip_serializing_if = "Option::is_none")]
    pub timer_rego_module_parse_ns: Option<i32>,
    #[serde(rename = "timer_rego_module_compile_ns", skip_serializing_if = "Option::is_none")]
    pub timer_rego_module_compile_ns: Option<i32>,
    #[serde(rename = "timer_server_handler_ns", skip_serializing_if = "Option::is_none")]
    pub timer_server_handler_ns: Option<i32>,
}

impl OpaMetrics {
    pub fn new() -> OpaMetrics {
        OpaMetrics {
            timer_rego_input_parse_ns: None,
            timer_rego_query_parse_ns: None,
            timer_rego_query_compile_ns: None,
            timer_rego_query_eval_ns: None,
            timer_rego_module_parse_ns: None,
            timer_rego_module_compile_ns: None,
            timer_server_handler_ns: None,
        }
    }
}

