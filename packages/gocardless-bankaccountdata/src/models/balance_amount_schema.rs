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

/// BalanceAmountSchema : BalanceAmountSchema.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct BalanceAmountSchema {
    /// amount
    #[serde(rename = "amount")]
    pub amount: String,
    /// currency
    #[serde(rename = "currency")]
    pub currency: String,
}

impl BalanceAmountSchema {
    /// BalanceAmountSchema.
    pub fn new(amount: String, currency: String) -> BalanceAmountSchema {
        BalanceAmountSchema { amount, currency }
    }
}