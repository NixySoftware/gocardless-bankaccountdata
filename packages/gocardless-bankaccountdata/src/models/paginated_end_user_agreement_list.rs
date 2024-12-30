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
pub struct PaginatedEndUserAgreementList {
    #[serde(rename = "count")]
    pub count: i32,
    #[serde(
        rename = "next",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub next: Option<Option<String>>,
    #[serde(
        rename = "previous",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub previous: Option<Option<String>>,
    #[serde(rename = "results")]
    pub results: Vec<models::EndUserAgreement>,
}

impl PaginatedEndUserAgreementList {
    pub fn new(
        count: i32,
        results: Vec<models::EndUserAgreement>,
    ) -> PaginatedEndUserAgreementList {
        PaginatedEndUserAgreementList {
            count,
            next: None,
            previous: None,
            results,
        }
    }
}