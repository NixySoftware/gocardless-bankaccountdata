/*
 * GoCardless Bank Account Data API
 *
 * Securely access your user's bank account information for better lending, accounting, verification and financial management.
 *
 * The version of the OpenAPI document: 2.0.0
 *
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ErrorResponse {
    #[serde(rename = "summary")]
    pub summary: String,
    #[serde(rename = "detail")]
    pub detail: String,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(rename = "status_code")]
    pub status_code: i32,
}

impl ErrorResponse {
    pub fn new(summary: String, detail: String, status_code: i32) -> ErrorResponse {
        ErrorResponse {
            summary,
            detail,
            r#type: None,
            status_code,
        }
    }
}
