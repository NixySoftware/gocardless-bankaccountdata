use std::error;
use std::fmt;

#[derive(Debug, Clone)]
pub struct ResponseContent<T> {
    pub status: reqwest::StatusCode,
    pub content: String,
    pub entity: Option<T>,
}

#[derive(Debug)]
pub enum Error<T> {
    Reqwest(reqwest::Error),
    ReqwestMiddleware(reqwest_middleware::Error),
    Serde(serde_json::Error),
    Io(std::io::Error),
    ResponseError(ResponseContent<T>),
}

impl<T> fmt::Display for Error<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (module, e) = match self {
            Error::Reqwest(e) => ("reqwest", e.to_string()),
            Error::ReqwestMiddleware(e) => ("reqwest-middleware", e.to_string()),
            Error::Serde(e) => ("serde", e.to_string()),
            Error::Io(e) => ("IO", e.to_string()),
            Error::ResponseError(e) => ("response", format!("status code {}", e.status)),
        };
        write!(f, "error in {}: {}", module, e)
    }
}

impl<T: fmt::Debug> error::Error for Error<T> {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        Some(match self {
            Error::Reqwest(e) => e,
            Error::ReqwestMiddleware(e) => e,
            Error::Serde(e) => e,
            Error::Io(e) => e,
            Error::ResponseError(_) => return None,
        })
    }
}

impl<T> From<reqwest::Error> for Error<T> {
    fn from(e: reqwest::Error) -> Self {
        Error::Reqwest(e)
    }
}

impl<T> From<reqwest_middleware::Error> for Error<T> {
    fn from(e: reqwest_middleware::Error) -> Self {
        Error::ReqwestMiddleware(e)
    }
}

impl<T> From<serde_json::Error> for Error<T> {
    fn from(e: serde_json::Error) -> Self {
        Error::Serde(e)
    }
}

impl<T> From<std::io::Error> for Error<T> {
    fn from(e: std::io::Error) -> Self {
        Error::Io(e)
    }
}

pub fn urlencode<T: AsRef<str>>(s: T) -> String {
    ::url::form_urlencoded::byte_serialize(s.as_ref().as_bytes()).collect()
}

pub fn parse_deep_object(prefix: &str, value: &serde_json::Value) -> Vec<(String, String)> {
    if let serde_json::Value::Object(object) = value {
        let mut params = vec![];

        for (key, value) in object {
            match value {
                serde_json::Value::Object(_) => params.append(&mut parse_deep_object(
                    &format!("{}[{}]", prefix, key),
                    value,
                )),
                serde_json::Value::Array(array) => {
                    for (i, value) in array.iter().enumerate() {
                        params.append(&mut parse_deep_object(
                            &format!("{}[{}][{}]", prefix, key, i),
                            value,
                        ));
                    }
                }
                serde_json::Value::String(s) => {
                    params.push((format!("{}[{}]", prefix, key), s.clone()))
                }
                _ => params.push((format!("{}[{}]", prefix, key), value.to_string())),
            }
        }

        return params;
    }

    unimplemented!("Only objects are supported with style=deepObject")
}

pub mod accounts_api;
pub mod agreements_api;
pub mod institutions_api;
pub mod requisitions_api;
pub mod token_api;

pub mod configuration;

use std::sync::Arc;

pub trait Api {
    fn accounts_api(&self) -> &dyn accounts_api::AccountsApi;
    fn agreements_api(&self) -> &dyn agreements_api::AgreementsApi;
    fn institutions_api(&self) -> &dyn institutions_api::InstitutionsApi;
    fn requisitions_api(&self) -> &dyn requisitions_api::RequisitionsApi;
    fn token_api(&self) -> &dyn token_api::TokenApi;
}

pub struct ApiClient {
    accounts_api: Box<dyn accounts_api::AccountsApi>,
    agreements_api: Box<dyn agreements_api::AgreementsApi>,
    institutions_api: Box<dyn institutions_api::InstitutionsApi>,
    requisitions_api: Box<dyn requisitions_api::RequisitionsApi>,
    token_api: Box<dyn token_api::TokenApi>,
}

impl ApiClient {
    pub fn new(configuration: Arc<configuration::Configuration>) -> Self {
        Self {
            accounts_api: Box::new(accounts_api::AccountsApiClient::new(configuration.clone())),
            agreements_api: Box::new(agreements_api::AgreementsApiClient::new(
                configuration.clone(),
            )),
            institutions_api: Box::new(institutions_api::InstitutionsApiClient::new(
                configuration.clone(),
            )),
            requisitions_api: Box::new(requisitions_api::RequisitionsApiClient::new(
                configuration.clone(),
            )),
            token_api: Box::new(token_api::TokenApiClient::new(configuration.clone())),
        }
    }
}

impl Api for ApiClient {
    fn accounts_api(&self) -> &dyn accounts_api::AccountsApi {
        self.accounts_api.as_ref()
    }
    fn agreements_api(&self) -> &dyn agreements_api::AgreementsApi {
        self.agreements_api.as_ref()
    }
    fn institutions_api(&self) -> &dyn institutions_api::InstitutionsApi {
        self.institutions_api.as_ref()
    }
    fn requisitions_api(&self) -> &dyn requisitions_api::RequisitionsApi {
        self.requisitions_api.as_ref()
    }
    fn token_api(&self) -> &dyn token_api::TokenApi {
        self.token_api.as_ref()
    }
}
