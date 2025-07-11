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

/// DetailSchema : DetailSchema.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct DetailSchema {
    /// resourceId
    #[serde(rename = "resourceId", skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
    /// iban
    #[serde(rename = "iban", skip_serializing_if = "Option::is_none")]
    pub iban: Option<String>,
    /// bban
    #[serde(rename = "bban", skip_serializing_if = "Option::is_none")]
    pub bban: Option<String>,
    /// SortCodeAccountNumber returned by some UK banks (6 digit Sort Code and 8 digit Account Number)
    #[serde(rename = "scan", skip_serializing_if = "Option::is_none")]
    pub scan: Option<String>,
    /// msisdn
    #[serde(rename = "msisdn", skip_serializing_if = "Option::is_none")]
    pub msisdn: Option<String>,
    /// currency
    #[serde(rename = "currency", skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    /// ownerName
    #[serde(rename = "ownerName", skip_serializing_if = "Option::is_none")]
    pub owner_name: Option<String>,
    /// name
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// displayName
    #[serde(rename = "displayName", skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    /// product
    #[serde(rename = "product", skip_serializing_if = "Option::is_none")]
    pub product: Option<String>,
    /// cashAccountType
    #[serde(rename = "cashAccountType", skip_serializing_if = "Option::is_none")]
    pub cash_account_type: Option<String>,
    /// status
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// bic
    #[serde(rename = "bic", skip_serializing_if = "Option::is_none")]
    pub bic: Option<String>,
    /// linkedAccounts
    #[serde(rename = "linkedAccounts", skip_serializing_if = "Option::is_none")]
    pub linked_accounts: Option<String>,
    /// maskedPan
    #[serde(rename = "maskedPan", skip_serializing_if = "Option::is_none")]
    pub masked_pan: Option<String>,
    /// usage
    #[serde(rename = "usage", skip_serializing_if = "Option::is_none")]
    pub usage: Option<String>,
    /// details
    #[serde(rename = "details", skip_serializing_if = "Option::is_none")]
    pub details: Option<String>,
    /// ownerAddressUnstructured
    #[serde(
        rename = "ownerAddressUnstructured",
        skip_serializing_if = "Option::is_none"
    )]
    pub owner_address_unstructured: Option<Vec<String>>,
    /// ownerAddressStructured
    #[serde(
        rename = "ownerAddressStructured",
        skip_serializing_if = "Option::is_none"
    )]
    pub owner_address_structured: Option<Box<models::OwnerAddressStructuredSchema>>,
    /// additionalAccountData used for information that is outside of Berlin Group specification, such as bank or country-specific fields
    #[serde(
        rename = "additionalAccountData",
        skip_serializing_if = "Option::is_none"
    )]
    pub additional_account_data: Option<Box<models::AdditionalAccountDataSchema>>,
}

impl DetailSchema {
    /// DetailSchema.
    pub fn new() -> DetailSchema {
        DetailSchema {
            resource_id: None,
            iban: None,
            bban: None,
            scan: None,
            msisdn: None,
            currency: None,
            owner_name: None,
            name: None,
            display_name: None,
            product: None,
            cash_account_type: None,
            status: None,
            bic: None,
            linked_accounts: None,
            masked_pan: None,
            usage: None,
            details: None,
            owner_address_unstructured: None,
            owner_address_structured: None,
            additional_account_data: None,
        }
    }
}
