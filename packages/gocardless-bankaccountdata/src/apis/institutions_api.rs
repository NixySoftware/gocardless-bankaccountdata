/*
 * GoCardless Bank Account Data API
 *
 * Securely access your user's bank account information for better lending, accounting, verification and financial management.
 *
 * The version of the OpenAPI document: 2.0.4
 *
 * Generated by: https://openapi-generator.tech
 */

use super::{configuration, Error};
use crate::{apis::ResponseContent, models};
use async_trait::async_trait;
use reqwest;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[async_trait]
pub trait InstitutionsApi: Send + Sync {
    async fn retrieve_all_supported_institutions_in_a_given_country(
        &self,
        params: RetrieveAllSupportedInstitutionsInAGivenCountryParams,
    ) -> Result<Vec<models::Integration>, Error<RetrieveAllSupportedInstitutionsInAGivenCountryError>>;
    async fn retrieve_institution(
        &self,
        params: RetrieveInstitutionParams,
    ) -> Result<models::IntegrationRetrieve, Error<RetrieveInstitutionError>>;
}

pub struct InstitutionsApiClient {
    configuration: Arc<configuration::Configuration>,
}

impl InstitutionsApiClient {
    pub fn new(configuration: Arc<configuration::Configuration>) -> Self {
        Self { configuration }
    }
}

/// struct for passing parameters to the method [`retrieve_all_supported_institutions_in_a_given_country`]
#[derive(Clone, Debug)]
#[cfg_attr(feature = "bon", derive(::bon::Builder))]
pub struct RetrieveAllSupportedInstitutionsInAGivenCountryParams {
    /// Boolean value, indicating if access scopes are supported
    pub access_scopes_supported: Option<String>,
    /// Boolean value, indicating if account selection is supported
    pub account_selection_supported: Option<String>,
    /// Boolean value, indicating if business accounts are supported
    pub business_accounts_supported: Option<String>,
    /// Boolean value, indicating if card accounts are supported
    pub card_accounts_supported: Option<String>,
    /// Boolean value, indicating if corporate accounts are supported
    pub corporate_accounts_supported: Option<String>,
    /// ISO 3166 two-character country code
    pub country: Option<String>,
    /// Boolean value, indicating if pending transactions are supported
    pub pending_transactions_supported: Option<String>,
    /// Boolean value, indicating if private accounts are supported
    pub private_accounts_supported: Option<String>,
    /// Boolean value, indicating if debtor account can be read before submitting payment
    pub read_debtor_account_supported: Option<String>,
    /// Boolean value, indicating if read refund account is supported
    pub read_refund_account_supported: Option<String>,
    /// Boolean value, indicating if separate consent for continuous history is supported
    pub separate_continuous_history_consent_supported: Option<String>,
    /// Boolean value, indicating if ssn verification is supported
    pub ssn_verification_supported: Option<String>,
}

/// struct for passing parameters to the method [`retrieve_institution`]
#[derive(Clone, Debug)]
#[cfg_attr(feature = "bon", derive(::bon::Builder))]
pub struct RetrieveInstitutionParams {
    pub id: String,
}

#[async_trait]
impl InstitutionsApi for InstitutionsApiClient {
    /// List all available institutions
    async fn retrieve_all_supported_institutions_in_a_given_country(
        &self,
        params: RetrieveAllSupportedInstitutionsInAGivenCountryParams,
    ) -> Result<Vec<models::Integration>, Error<RetrieveAllSupportedInstitutionsInAGivenCountryError>>
    {
        let RetrieveAllSupportedInstitutionsInAGivenCountryParams {
            access_scopes_supported,
            account_selection_supported,
            business_accounts_supported,
            card_accounts_supported,
            corporate_accounts_supported,
            country,
            pending_transactions_supported,
            private_accounts_supported,
            read_debtor_account_supported,
            read_refund_account_supported,
            separate_continuous_history_consent_supported,
            ssn_verification_supported,
        } = params;

        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str =
            format!("{}/api/v2/institutions/", local_var_configuration.base_path);
        let mut local_var_req_builder =
            local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

        if let Some(ref local_var_str) = access_scopes_supported {
            local_var_req_builder = local_var_req_builder
                .query(&[("access_scopes_supported", &local_var_str.to_string())]);
        }
        if let Some(ref local_var_str) = account_selection_supported {
            local_var_req_builder = local_var_req_builder
                .query(&[("account_selection_supported", &local_var_str.to_string())]);
        }
        if let Some(ref local_var_str) = business_accounts_supported {
            local_var_req_builder = local_var_req_builder
                .query(&[("business_accounts_supported", &local_var_str.to_string())]);
        }
        if let Some(ref local_var_str) = card_accounts_supported {
            local_var_req_builder = local_var_req_builder
                .query(&[("card_accounts_supported", &local_var_str.to_string())]);
        }
        if let Some(ref local_var_str) = corporate_accounts_supported {
            local_var_req_builder = local_var_req_builder
                .query(&[("corporate_accounts_supported", &local_var_str.to_string())]);
        }
        if let Some(ref local_var_str) = country {
            local_var_req_builder =
                local_var_req_builder.query(&[("country", &local_var_str.to_string())]);
        }
        if let Some(ref local_var_str) = pending_transactions_supported {
            local_var_req_builder = local_var_req_builder
                .query(&[("pending_transactions_supported", &local_var_str.to_string())]);
        }
        if let Some(ref local_var_str) = private_accounts_supported {
            local_var_req_builder = local_var_req_builder
                .query(&[("private_accounts_supported", &local_var_str.to_string())]);
        }
        if let Some(ref local_var_str) = read_debtor_account_supported {
            local_var_req_builder = local_var_req_builder
                .query(&[("read_debtor_account_supported", &local_var_str.to_string())]);
        }
        if let Some(ref local_var_str) = read_refund_account_supported {
            local_var_req_builder = local_var_req_builder
                .query(&[("read_refund_account_supported", &local_var_str.to_string())]);
        }
        if let Some(ref local_var_str) = separate_continuous_history_consent_supported {
            local_var_req_builder = local_var_req_builder.query(&[(
                "separate_continuous_history_consent_supported",
                &local_var_str.to_string(),
            )]);
        }
        if let Some(ref local_var_str) = ssn_verification_supported {
            local_var_req_builder = local_var_req_builder
                .query(&[("ssn_verification_supported", &local_var_str.to_string())]);
        }
        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder
                .header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
            local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
        };

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            serde_json::from_str(&local_var_content).map_err(Error::from)
        } else {
            let local_var_entity: Option<RetrieveAllSupportedInstitutionsInAGivenCountryError> =
                serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent {
                status: local_var_status,
                content: local_var_content,
                entity: local_var_entity,
            };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Get details about a specific Institution and its supported features
    async fn retrieve_institution(
        &self,
        params: RetrieveInstitutionParams,
    ) -> Result<models::IntegrationRetrieve, Error<RetrieveInstitutionError>> {
        let RetrieveInstitutionParams { id } = params;

        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!(
            "{}/api/v2/institutions/{id}/",
            local_var_configuration.base_path,
            id = crate::apis::urlencode(id)
        );
        let mut local_var_req_builder =
            local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder
                .header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
            local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
        };

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            serde_json::from_str(&local_var_content).map_err(Error::from)
        } else {
            let local_var_entity: Option<RetrieveInstitutionError> =
                serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent {
                status: local_var_status,
                content: local_var_content,
                entity: local_var_entity,
            };
            Err(Error::ResponseError(local_var_error))
        }
    }
}

/// struct for typed errors of method [`retrieve_all_supported_institutions_in_a_given_country`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RetrieveAllSupportedInstitutionsInAGivenCountryError {
    Status400(models::ErrorResponse),
    Status404(models::ErrorResponse),
    Status429(models::ErrorResponse),
    Status401(models::ErrorResponse),
    Status403(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`retrieve_institution`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RetrieveInstitutionError {
    Status404(models::ErrorResponse),
    Status429(models::ErrorResponse),
    Status401(models::ErrorResponse),
    Status403(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}
