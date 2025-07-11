/*
 * GoCardless Bank Account Data API
 *
 * Securely access your user's bank account information for better lending, accounting, verification and financial management.
 *
 * The version of the OpenAPI document: 2.0.4
 *
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// JwtRefreshRequest : Refresh access token.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct JwtRefreshRequest {
    #[serde(rename = "refresh")]
    pub refresh: String,
}

impl JwtRefreshRequest {
    /// Refresh access token.
    pub fn new(refresh: String) -> JwtRefreshRequest {
        JwtRefreshRequest { refresh }
    }
}
