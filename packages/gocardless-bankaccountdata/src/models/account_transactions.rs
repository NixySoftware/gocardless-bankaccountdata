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

/// AccountTransactions : AccountTransactionsSerializer.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct AccountTransactions {
    /// transactions
    #[serde(rename = "transactions")]
    pub transactions: Box<models::BankTransaction>,
}

impl AccountTransactions {
    /// AccountTransactionsSerializer.
    pub fn new(transactions: models::BankTransaction) -> AccountTransactions {
        AccountTransactions {
            transactions: Box::new(transactions),
        }
    }
}
