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

/// EnduserAcceptanceDetailsRequest : Represents end-user details.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct EnduserAcceptanceDetailsRequest {
    #[serde(rename = "user_agent")]
    pub user_agent: String,
    #[serde(rename = "ip_address")]
    pub ip_address: String,
}

impl EnduserAcceptanceDetailsRequest {
    /// Represents end-user details.
    pub fn new(user_agent: String, ip_address: String) -> EnduserAcceptanceDetailsRequest {
        EnduserAcceptanceDetailsRequest {
            user_agent,
            ip_address,
        }
    }
}