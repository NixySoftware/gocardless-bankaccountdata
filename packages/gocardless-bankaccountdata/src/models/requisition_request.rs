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

/// RequisitionRequest : RequisitionSerializer.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct RequisitionRequest {
    /// redirect URL to your application after end-user authorization with ASPSP
    #[serde(rename = "redirect", deserialize_with = "Option::deserialize")]
    pub redirect: Option<String>,
    /// an Institution ID for this Requisition
    #[serde(rename = "institution_id")]
    pub institution_id: String,
    /// EUA associated with this requisition
    #[serde(rename = "agreement", skip_serializing_if = "Option::is_none")]
    pub agreement: Option<uuid::Uuid>,
    /// additional ID to identify the end user
    #[serde(rename = "reference", skip_serializing_if = "Option::is_none")]
    pub reference: Option<String>,
    /// A two-letter country code (ISO 639-1)
    #[serde(rename = "user_language", skip_serializing_if = "Option::is_none")]
    pub user_language: Option<String>,
    /// optional SSN field to verify ownership of the account
    #[serde(rename = "ssn", skip_serializing_if = "Option::is_none")]
    pub ssn: Option<String>,
    /// option to enable account selection view for the end user
    #[serde(rename = "account_selection", skip_serializing_if = "Option::is_none")]
    pub account_selection: Option<bool>,
    /// enable redirect back to the client after account list received
    #[serde(rename = "redirect_immediate", skip_serializing_if = "Option::is_none")]
    pub redirect_immediate: Option<bool>,
}

impl RequisitionRequest {
    /// RequisitionSerializer.
    pub fn new(redirect: Option<String>, institution_id: String) -> RequisitionRequest {
        RequisitionRequest {
            redirect,
            institution_id,
            agreement: None,
            reference: None,
            user_language: None,
            ssn: None,
            account_selection: None,
            redirect_immediate: None,
        }
    }
}
