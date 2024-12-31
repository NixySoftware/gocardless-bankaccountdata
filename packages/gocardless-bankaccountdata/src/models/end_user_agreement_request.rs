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

/// EndUserAgreementRequest : Represents an end-user agreement.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct EndUserAgreementRequest {
    /// an Institution ID for this EUA
    #[serde(rename = "institution_id")]
    pub institution_id: String,
    /// Maximum number of days of transaction data to retrieve.
    #[serde(
        rename = "max_historical_days",
        skip_serializing_if = "Option::is_none"
    )]
    pub max_historical_days: Option<i16>,
    /// Number of days from acceptance that the access can be used.
    #[serde(
        rename = "access_valid_for_days",
        skip_serializing_if = "Option::is_none"
    )]
    pub access_valid_for_days: Option<i16>,
    /// Array containing one or several values of ['balances', 'details', 'transactions']
    #[serde(rename = "access_scope", skip_serializing_if = "Option::is_none")]
    pub access_scope: Option<Vec<serde_json::Value>>,
}

impl EndUserAgreementRequest {
    /// Represents an end-user agreement.
    pub fn new(institution_id: String) -> EndUserAgreementRequest {
        EndUserAgreementRequest {
            institution_id,
            max_historical_days: None,
            access_valid_for_days: None,
            access_scope: None,
        }
    }
}
