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

pub struct RulesApiClient {
    configuration: Arc<configuration::Configuration>,
}

impl RulesApiClient {
    pub fn new(configuration: Arc<configuration::Configuration>) -> RulesApiClient {
        RulesApiClient {
            configuration,
        }
    }
}

#[async_trait]
pub trait RulesApi {
    async fn delete_rule(&self, id: i32) -> Result<(), Error>;
    async fn fire_rule(&self, id: i32, start: Option<String>, end: Option<String>, accounts: Option<&str>) -> Result<(), Error>;
    async fn get_rule(&self, id: i32) -> Result<crate::models::RuleSingle, Error>;
    async fn list_rule(&self, page: Option<i32>) -> Result<crate::models::RuleArray, Error>;
    async fn store_rule(&self, rule: crate::models::Rule) -> Result<crate::models::RuleSingle, Error>;
    async fn test_rule(&self, id: i32, page: Option<i32>, start: Option<String>, end: Option<String>, search_limit: Option<i32>, triggered_limit: Option<i32>, accounts: Option<&str>) -> Result<crate::models::TransactionArray, Error>;
    async fn update_rule(&self, id: i32, rule: crate::models::Rule) -> Result<crate::models::RuleSingle, Error>;
}

#[async_trait]
impl RulesApi for RulesApiClient {
    async fn delete_rule(&self, id: i32) -> Result<(), Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let client = &configuration.client;

        let uri_str = format!("{}/api/v1/rules/{id}", configuration.base_path, id=id);
        let mut req_builder = client.request(::reqwest::Method::DELETE, uri_str.as_str());

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

    async fn fire_rule(&self, id: i32, start: Option<String>, end: Option<String>, accounts: Option<&str>) -> Result<(), Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let client = &configuration.client;

        let uri_str = format!("{}/api/v1/rules/{id}/trigger", configuration.base_path, id=id);
        let mut req_builder = client.request(::reqwest::Method::POST, uri_str.as_str());

        if let Some(ref s) = start {
            req_builder = req_builder.query(&[("start", &s.to_string())]);
        }
        if let Some(ref s) = end {
            req_builder = req_builder.query(&[("end", &s.to_string())]);
        }
        if let Some(ref s) = accounts {
            req_builder = req_builder.query(&[("accounts", &s.to_string())]);
        }
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

    async fn get_rule(&self, id: i32) -> Result<crate::models::RuleSingle, Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let client = &configuration.client;

        let uri_str = format!("{}/api/v1/rules/{id}", configuration.base_path, id=id);
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

    async fn list_rule(&self, page: Option<i32>) -> Result<crate::models::RuleArray, Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let client = &configuration.client;

        let uri_str = format!("{}/api/v1/rules", configuration.base_path);
        let mut req_builder = client.request(::reqwest::Method::GET, uri_str.as_str());

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

    async fn store_rule(&self, rule: crate::models::Rule) -> Result<crate::models::RuleSingle, Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let client = &configuration.client;

        let uri_str = format!("{}/api/v1/rules", configuration.base_path);
        let mut req_builder = client.request(::reqwest::Method::POST, uri_str.as_str());

        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }
        if let Some(ref token) = configuration.oauth_access_token {
            req_builder = req_builder.bearer_auth(token.to_owned());
        };
        req_builder = req_builder.json(&rule);

        // send request
        let req = req_builder.build()?;

        Ok(client.execute(req).await?.error_for_status()?.json().await?)
    }

    async fn test_rule(&self, id: i32, page: Option<i32>, start: Option<String>, end: Option<String>, search_limit: Option<i32>, triggered_limit: Option<i32>, accounts: Option<&str>) -> Result<crate::models::TransactionArray, Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let client = &configuration.client;

        let uri_str = format!("{}/api/v1/rules/{id}/test", configuration.base_path, id=id);
        let mut req_builder = client.request(::reqwest::Method::GET, uri_str.as_str());

        if let Some(ref s) = page {
            req_builder = req_builder.query(&[("page", &s.to_string())]);
        }
        if let Some(ref s) = start {
            req_builder = req_builder.query(&[("start", &s.to_string())]);
        }
        if let Some(ref s) = end {
            req_builder = req_builder.query(&[("end", &s.to_string())]);
        }
        if let Some(ref s) = search_limit {
            req_builder = req_builder.query(&[("search_limit", &s.to_string())]);
        }
        if let Some(ref s) = triggered_limit {
            req_builder = req_builder.query(&[("triggered_limit", &s.to_string())]);
        }
        if let Some(ref s) = accounts {
            req_builder = req_builder.query(&[("accounts", &s.to_string())]);
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

    async fn update_rule(&self, id: i32, rule: crate::models::Rule) -> Result<crate::models::RuleSingle, Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let client = &configuration.client;

        let uri_str = format!("{}/api/v1/rules/{id}", configuration.base_path, id=id);
        let mut req_builder = client.request(::reqwest::Method::PUT, uri_str.as_str());

        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }
        if let Some(ref token) = configuration.oauth_access_token {
            req_builder = req_builder.bearer_auth(token.to_owned());
        };
        req_builder = req_builder.json(&rule);

        // send request
        let req = req_builder.build()?;

        Ok(client.execute(req).await?.error_for_status()?.json().await?)
    }

}
