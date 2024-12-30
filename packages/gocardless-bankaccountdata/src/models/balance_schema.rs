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

/// BalanceSchema : BalanceSchema.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct BalanceSchema {
    /// balanceAmount
    #[serde(rename = "balanceAmount")]
    pub balance_amount: Box<models::BalanceAmountSchema>,
    /// balanceType
    #[serde(rename = "balanceType")]
    pub balance_type: String,
    /// creditLimitIncluded
    #[serde(
        rename = "creditLimitIncluded",
        skip_serializing_if = "Option::is_none"
    )]
    pub credit_limit_included: Option<bool>,
    /// lastChangeDateTime
    #[serde(rename = "lastChangeDateTime", skip_serializing_if = "Option::is_none")]
    pub last_change_date_time: Option<String>,
    /// referenceDate
    #[serde(rename = "referenceDate", skip_serializing_if = "Option::is_none")]
    pub reference_date: Option<String>,
    /// lastCommittedTransaction
    #[serde(
        rename = "lastCommittedTransaction",
        skip_serializing_if = "Option::is_none"
    )]
    pub last_committed_transaction: Option<String>,
}

impl BalanceSchema {
    /// BalanceSchema.
    pub fn new(balance_amount: models::BalanceAmountSchema, balance_type: String) -> BalanceSchema {
        BalanceSchema {
            balance_amount: Box::new(balance_amount),
            balance_type,
            credit_limit_included: None,
            last_change_date_time: None,
            reference_date: None,
            last_committed_transaction: None,
        }
    }
}