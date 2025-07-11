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
pub trait TokenApi: Send + Sync {
    async fn get_a_new_access_token(
        &self,
        params: GetANewAccessTokenParams,
    ) -> Result<models::SpectacularJwtRefresh, Error<GetANewAccessTokenError>>;
    async fn obtain_new_access_slash_refresh_token_pair(
        &self,
        params: ObtainNewAccessSlashRefreshTokenPairParams,
    ) -> Result<models::SpectacularJwtObtain, Error<ObtainNewAccessSlashRefreshTokenPairError>>;
}

pub struct TokenApiClient {
    configuration: Arc<configuration::Configuration>,
}

impl TokenApiClient {
    pub fn new(configuration: Arc<configuration::Configuration>) -> Self {
        Self { configuration }
    }
}

/// struct for passing parameters to the method [`get_a_new_access_token`]
#[derive(Clone, Debug)]
#[cfg_attr(feature = "bon", derive(::bon::Builder))]
pub struct GetANewAccessTokenParams {
    pub jwt_refresh_request: models::JwtRefreshRequest,
}

/// struct for passing parameters to the method [`obtain_new_access_slash_refresh_token_pair`]
#[derive(Clone, Debug)]
#[cfg_attr(feature = "bon", derive(::bon::Builder))]
pub struct ObtainNewAccessSlashRefreshTokenPairParams {
    pub jwt_obtain_pair_request: models::JwtObtainPairRequest,
}

#[async_trait]
impl TokenApi for TokenApiClient {
    /// Refresh access token
    async fn get_a_new_access_token(
        &self,
        params: GetANewAccessTokenParams,
    ) -> Result<models::SpectacularJwtRefresh, Error<GetANewAccessTokenError>> {
        let GetANewAccessTokenParams {
            jwt_refresh_request,
        } = params;

        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!(
            "{}/api/v2/token/refresh/",
            local_var_configuration.base_path
        );
        let mut local_var_req_builder =
            local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder
                .header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        local_var_req_builder = local_var_req_builder.json(&jwt_refresh_request);

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            serde_json::from_str(&local_var_content).map_err(Error::from)
        } else {
            let local_var_entity: Option<GetANewAccessTokenError> =
                serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent {
                status: local_var_status,
                content: local_var_content,
                entity: local_var_entity,
            };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Obtain JWT pair
    async fn obtain_new_access_slash_refresh_token_pair(
        &self,
        params: ObtainNewAccessSlashRefreshTokenPairParams,
    ) -> Result<models::SpectacularJwtObtain, Error<ObtainNewAccessSlashRefreshTokenPairError>>
    {
        let ObtainNewAccessSlashRefreshTokenPairParams {
            jwt_obtain_pair_request,
        } = params;

        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/api/v2/token/new/", local_var_configuration.base_path);
        let mut local_var_req_builder =
            local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder
                .header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        local_var_req_builder = local_var_req_builder.json(&jwt_obtain_pair_request);

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            serde_json::from_str(&local_var_content).map_err(Error::from)
        } else {
            let local_var_entity: Option<ObtainNewAccessSlashRefreshTokenPairError> =
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

/// struct for typed errors of method [`get_a_new_access_token`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetANewAccessTokenError {
    Status403(models::ErrorResponse),
    Status401(models::ErrorResponse),
    Status429(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`obtain_new_access_slash_refresh_token_pair`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ObtainNewAccessSlashRefreshTokenPairError {
    Status401(models::ErrorResponse),
    Status403(models::ErrorResponse),
    Status429(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}
