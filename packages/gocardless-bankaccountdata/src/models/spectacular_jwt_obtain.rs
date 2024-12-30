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

/// SpectacularJwtObtain : Obtain new JWT pair.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct SpectacularJwtObtain {
    /// Your access token
    #[serde(rename = "access", skip_serializing_if = "Option::is_none")]
    pub access: Option<String>,
    /// Access token expires in seconds
    #[serde(rename = "access_expires", skip_serializing_if = "Option::is_none")]
    pub access_expires: Option<i32>,
    /// Your refresh token
    #[serde(rename = "refresh", skip_serializing_if = "Option::is_none")]
    pub refresh: Option<String>,
    /// Refresh token expires in seconds
    #[serde(rename = "refresh_expires", skip_serializing_if = "Option::is_none")]
    pub refresh_expires: Option<i32>,
}

impl SpectacularJwtObtain {
    /// Obtain new JWT pair.
    pub fn new() -> SpectacularJwtObtain {
        SpectacularJwtObtain {
            access: None,
            access_expires: None,
            refresh: None,
            refresh_expires: None,
        }
    }
}