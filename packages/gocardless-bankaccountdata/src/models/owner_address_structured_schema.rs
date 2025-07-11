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

/// OwnerAddressStructuredSchema : OwnerAddressStructuredSchema.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct OwnerAddressStructuredSchema {
    /// streetName
    #[serde(rename = "streetName", skip_serializing_if = "Option::is_none")]
    pub street_name: Option<String>,
    /// buildingNumber
    #[serde(rename = "buildingNumber", skip_serializing_if = "Option::is_none")]
    pub building_number: Option<String>,
    /// townName
    #[serde(rename = "townName", skip_serializing_if = "Option::is_none")]
    pub town_name: Option<String>,
    /// postCode
    #[serde(rename = "postCode", skip_serializing_if = "Option::is_none")]
    pub post_code: Option<String>,
    /// country
    #[serde(rename = "country", skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
}

impl OwnerAddressStructuredSchema {
    /// OwnerAddressStructuredSchema.
    pub fn new() -> OwnerAddressStructuredSchema {
        OwnerAddressStructuredSchema {
            street_name: None,
            building_number: None,
            town_name: None,
            post_code: None,
            country: None,
        }
    }
}
