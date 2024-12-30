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

/// IntegrationRetrieve : IntegrationSerializer for Retrieve endpoint.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct IntegrationRetrieve {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "bic", skip_serializing_if = "Option::is_none")]
    pub bic: Option<String>,
    #[serde(
        rename = "transaction_total_days",
        skip_serializing_if = "Option::is_none"
    )]
    pub transaction_total_days: Option<String>,
    #[serde(
        rename = "max_access_valid_for_days",
        skip_serializing_if = "Option::is_none"
    )]
    pub max_access_valid_for_days: Option<String>,
    #[serde(rename = "countries")]
    pub countries: Vec<String>,
    #[serde(rename = "logo")]
    pub logo: String,
    #[serde(rename = "supported_payments")]
    pub supported_payments: std::collections::HashMap<String, serde_json::Value>,
    #[serde(rename = "supported_features")]
    pub supported_features: Vec<serde_json::Value>,
    #[serde(rename = "identification_codes")]
    pub identification_codes: Vec<serde_json::Value>,
}

impl IntegrationRetrieve {
    /// IntegrationSerializer for Retrieve endpoint.
    pub fn new(
        id: String,
        name: String,
        countries: Vec<String>,
        logo: String,
        supported_payments: std::collections::HashMap<String, serde_json::Value>,
        supported_features: Vec<serde_json::Value>,
        identification_codes: Vec<serde_json::Value>,
    ) -> IntegrationRetrieve {
        IntegrationRetrieve {
            id,
            name,
            bic: None,
            transaction_total_days: None,
            max_access_valid_for_days: None,
            countries,
            logo,
            supported_payments,
            supported_features,
            identification_codes,
        }
    }
}