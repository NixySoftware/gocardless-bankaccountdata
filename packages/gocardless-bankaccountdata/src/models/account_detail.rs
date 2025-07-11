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

/// AccountDetail : AccountDetailSerializer.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct AccountDetail {
    /// account
    #[serde(rename = "account")]
    pub account: Box<models::DetailSchema>,
}

impl AccountDetail {
    /// AccountDetailSerializer.
    pub fn new(account: models::DetailSchema) -> AccountDetail {
        AccountDetail {
            account: Box::new(account),
        }
    }
}
