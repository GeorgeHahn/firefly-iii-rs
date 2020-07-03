/*
 * Firefly III API
 *
 * This is the official documentation of the Firefly III API. You can find accompanying documentation on the website of Firefly III itself (see below). Please report any bugs or issues. This version of the API is live from version v4.7.9 and onwards. You may use the \"Authorize\" button to try the API below. 
 *
 * The version of the OpenAPI document: 1.1.0
 * Contact: james@firefly-iii.org
 * Generated by: https://openapi-generator.tech
 */

use async_trait::async_trait;
use std::sync::Arc;
use std::borrow::Borrow;
#[allow(unused_imports)]
use std::option::Option;

use reqwest;

use super::{Error, configuration};

pub struct ConfigurationApiClient {
    configuration: Arc<configuration::Configuration>,
}

impl ConfigurationApiClient {
    pub fn new(configuration: Arc<configuration::Configuration>) -> ConfigurationApiClient {
        ConfigurationApiClient {
            configuration,
        }
    }
}

#[async_trait]
pub trait ConfigurationApi {
    async fn get_configuration(&self, ) -> Result<crate::models::Configuration, Error>;
    async fn set_configuration(&self, name: &str, value: &str) -> Result<crate::models::Configuration, Error>;
}

#[async_trait]
impl ConfigurationApi for ConfigurationApiClient {
    async fn get_configuration(&self, ) -> Result<crate::models::Configuration, Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let client = &configuration.client;

        let uri_str = format!("{}/api/v1/configuration", configuration.base_path);
        let mut req_builder = client.request(::reqwest::Method::GET, uri_str.as_str());

        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }
        if let Some(ref token) = configuration.oauth_access_token {
            req_builder = req_builder.bearer_auth(token.to_owned());
        };

        // send request
        let req = req_builder.build()?;

        Ok(client.execute(req).await?.error_for_status()?.json().await?)
    }

    async fn set_configuration(&self, name: &str, value: &str) -> Result<crate::models::Configuration, Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let client = &configuration.client;

        let uri_str = format!("{}/api/v1/configuration/{name}", configuration.base_path, name=crate::apis::urlencode(name));
        let mut req_builder = client.request(::reqwest::Method::POST, uri_str.as_str());

        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }
        if let Some(ref token) = configuration.oauth_access_token {
            req_builder = req_builder.bearer_auth(token.to_owned());
        };
        let mut form_params = std::collections::HashMap::new();
        form_params.insert("value", value.to_string());
        req_builder = req_builder.form(&form_params);

        // send request
        let req = req_builder.build()?;

        Ok(client.execute(req).await?.error_for_status()?.json().await?)
    }

}
