/*
 * Firefly III API
 *
 * This is the official documentation of the Firefly III API. You can find accompanying documentation on the website of Firefly III itself (see below). Please report any bugs or issues. This version of the API is live from version v4.7.9 and onwards. You may use the \"Authorize\" button to try the API below. 
 *
 * The version of the OpenAPI document: 0.10.4
 * Contact: thegrumpydictator@gmail.com
 * Generated by: https://openapi-generator.tech
 */

use async_trait::async_trait;
use std::sync::Arc;
use std::borrow::Borrow;
#[allow(unused_imports)]
use std::option::Option;

use reqwest;

use super::{Error, configuration};

pub struct PiggyBanksApiClient {
    configuration: Arc<configuration::Configuration>,
}

impl PiggyBanksApiClient {
    pub fn new(configuration: Arc<configuration::Configuration>) -> PiggyBanksApiClient {
        PiggyBanksApiClient {
            configuration,
        }
    }
}

#[async_trait]
pub trait PiggyBanksApi {
    async fn delete_piggy_bank(&self, id: i32) -> Result<(), Error>;
    async fn get_piggy_bank(&self, id: i32) -> Result<crate::models::PiggyBankSingle, Error>;
    async fn list_event_by_piggy_bank(&self, id: i32, page: Option<i32>) -> Result<crate::models::PiggyBankEventArray, Error>;
    async fn list_piggy_bank(&self, page: Option<i32>) -> Result<crate::models::PiggyBankArray, Error>;
    async fn store_piggy_bank(&self, piggy_bank: crate::models::PiggyBank) -> Result<crate::models::PiggyBankSingle, Error>;
    async fn update_piggy_bank(&self, id: i32, piggy_bank: crate::models::PiggyBank) -> Result<crate::models::PiggyBankSingle, Error>;
}

#[async_trait]
impl PiggyBanksApi for PiggyBanksApiClient {
    async fn delete_piggy_bank(&self, id: i32) -> Result<(), Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let client = &configuration.client;

        let uri_str = format!("{}/api/v1/piggy_banks/{id}", configuration.base_path, id=id);
        let mut req_builder = client.request(reqwest::Method::DELETE, uri_str.as_str());

        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }
        if let Some(ref token) = configuration.oauth_access_token {
            req_builder = req_builder.bearer_auth(token.to_owned());
        };

        // send request
        let req = req_builder.build()?;

        client.execute(req).await?.error_for_status()?;
        Ok(())
    }

    async fn get_piggy_bank(&self, id: i32) -> Result<crate::models::PiggyBankSingle, Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let client = &configuration.client;

        let uri_str = format!("{}/api/v1/piggy_banks/{id}", configuration.base_path, id=id);
        let mut req_builder = client.request(reqwest::Method::GET, uri_str.as_str());

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

    async fn list_event_by_piggy_bank(&self, id: i32, page: Option<i32>) -> Result<crate::models::PiggyBankEventArray, Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let client = &configuration.client;

        let uri_str = format!("{}/api/v1/piggy_banks/{id}/events", configuration.base_path, id=id);
        let mut req_builder = client.request(reqwest::Method::GET, uri_str.as_str());

        if let Some(ref s) = page {
            req_builder = req_builder.query(&[("page", &s.to_string())]);
        }
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

    async fn list_piggy_bank(&self, page: Option<i32>) -> Result<crate::models::PiggyBankArray, Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let client = &configuration.client;

        let uri_str = format!("{}/api/v1/piggy_banks", configuration.base_path);
        let mut req_builder = client.request(reqwest::Method::GET, uri_str.as_str());

        if let Some(ref s) = page {
            req_builder = req_builder.query(&[("page", &s.to_string())]);
        }
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

    async fn store_piggy_bank(&self, piggy_bank: crate::models::PiggyBank) -> Result<crate::models::PiggyBankSingle, Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let client = &configuration.client;

        let uri_str = format!("{}/api/v1/piggy_banks", configuration.base_path);
        let mut req_builder = client.request(reqwest::Method::POST, uri_str.as_str());

        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }
        if let Some(ref token) = configuration.oauth_access_token {
            req_builder = req_builder.bearer_auth(token.to_owned());
        };
        req_builder = req_builder.json(&piggy_bank);

        // send request
        let req = req_builder.build()?;

        Ok(client.execute(req).await?.error_for_status()?.json().await?)
    }

    async fn update_piggy_bank(&self, id: i32, piggy_bank: crate::models::PiggyBank) -> Result<crate::models::PiggyBankSingle, Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let client = &configuration.client;

        let uri_str = format!("{}/api/v1/piggy_banks/{id}", configuration.base_path, id=id);
        let mut req_builder = client.request(reqwest::Method::PUT, uri_str.as_str());

        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }
        if let Some(ref token) = configuration.oauth_access_token {
            req_builder = req_builder.bearer_auth(token.to_owned());
        };
        req_builder = req_builder.json(&piggy_bank);

        // send request
        let req = req_builder.build()?;

        Ok(client.execute(req).await?.error_for_status()?.json().await?)
    }

}
