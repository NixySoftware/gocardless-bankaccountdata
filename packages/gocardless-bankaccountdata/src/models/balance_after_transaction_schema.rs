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

/// BalanceAfterTransactionSchema : BalanceAfterTransactionSchema.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct BalanceAfterTransactionSchema {
    /// amount
    #[serde(rename = "amount")]
    pub amount: String,
    /// currency
    #[serde(rename = "currency", skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
}

impl BalanceAfterTransactionSchema {
    /// BalanceAfterTransactionSchema.
    pub fn new(amount: String) -> BalanceAfterTransactionSchema {
        BalanceAfterTransactionSchema {
            amount,
            currency: None,
        }
    }
}
