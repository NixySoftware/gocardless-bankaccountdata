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

/// SpectacularJwtRefresh : Refresh Access token.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct SpectacularJwtRefresh {
    /// Your access token
    #[serde(rename = "access", skip_serializing_if = "Option::is_none")]
    pub access: Option<String>,
    /// Access token expires in seconds
    #[serde(rename = "access_expires", skip_serializing_if = "Option::is_none")]
    pub access_expires: Option<i32>,
}

impl SpectacularJwtRefresh {
    /// Refresh Access token.
    pub fn new() -> SpectacularJwtRefresh {
        SpectacularJwtRefresh {
            access: None,
            access_expires: None,
        }
    }
}
