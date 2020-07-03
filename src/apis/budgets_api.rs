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

pub struct BudgetsApiClient {
    configuration: Arc<configuration::Configuration>,
}

impl BudgetsApiClient {
    pub fn new(configuration: Arc<configuration::Configuration>) -> BudgetsApiClient {
        BudgetsApiClient {
            configuration,
        }
    }
}

#[async_trait]
pub trait BudgetsApi {
    async fn delete_budget(&self, id: i32) -> Result<(), Error>;
    async fn delete_budget_limit(&self, id: i32) -> Result<(), Error>;
    async fn get_budget(&self, id: i32, start_date: Option<String>, end_date: Option<String>) -> Result<crate::models::BudgetSingle, Error>;
    async fn get_budget_limit(&self, id: i32) -> Result<crate::models::BudgetLimitSingle, Error>;
    async fn list_attachment_by_budget(&self, id: i32, page: Option<i32>) -> Result<crate::models::AttachmentArray, Error>;
    async fn list_budget(&self, page: Option<i32>, start: Option<String>, end: Option<String>) -> Result<crate::models::BudgetArray, Error>;
    async fn list_budget_limit_by_budget(&self, id: i32, start: Option<String>, end: Option<String>) -> Result<crate::models::BudgetLimitArray, Error>;
    async fn list_transaction_by_budget(&self, id: i32, limit: Option<i32>, page: Option<i32>, start: Option<String>, end: Option<String>, _type: Option<crate::models::TransactionTypeFilter>) -> Result<crate::models::TransactionArray, Error>;
    async fn list_transaction_by_budget_limit(&self, id: i32, page: Option<i32>, _type: Option<crate::models::TransactionTypeFilter>) -> Result<crate::models::TransactionArray, Error>;
    async fn store_budget(&self, budget: crate::models::Budget) -> Result<crate::models::BudgetSingle, Error>;
    async fn store_budget_limit(&self, id: i32, budget_limit: crate::models::BudgetLimit) -> Result<crate::models::BudgetLimitSingle, Error>;
    async fn update_budget(&self, id: i32, budget: crate::models::Budget) -> Result<crate::models::BudgetSingle, Error>;
    async fn update_budget_limit(&self, id: i32, budget_limit: crate::models::BudgetLimit) -> Result<crate::models::BudgetLimitSingle, Error>;
}

#[async_trait]
impl BudgetsApi for BudgetsApiClient {
    async fn delete_budget(&self, id: i32) -> Result<(), Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let client = &configuration.client;

        let uri_str = format!("{}/api/v1/budgets/{id}", configuration.base_path, id=id);
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

    async fn delete_budget_limit(&self, id: i32) -> Result<(), Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let client = &configuration.client;

        let uri_str = format!("{}/api/v1/budgets/limits/{id}", configuration.base_path, id=id);
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

    async fn get_budget(&self, id: i32, start_date: Option<String>, end_date: Option<String>) -> Result<crate::models::BudgetSingle, Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let client = &configuration.client;

        let uri_str = format!("{}/api/v1/budgets/{id}", configuration.base_path, id=id);
        let mut req_builder = client.request(::reqwest::Method::GET, uri_str.as_str());

        if let Some(ref s) = start_date {
            req_builder = req_builder.query(&[("start_date", &s.to_string())]);
        }
        if let Some(ref s) = end_date {
            req_builder = req_builder.query(&[("end_date", &s.to_string())]);
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

    async fn get_budget_limit(&self, id: i32) -> Result<crate::models::BudgetLimitSingle, Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let client = &configuration.client;

        let uri_str = format!("{}/api/v1/budgets/limits/{id}", configuration.base_path, id=id);
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

    async fn list_attachment_by_budget(&self, id: i32, page: Option<i32>) -> Result<crate::models::AttachmentArray, Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let client = &configuration.client;

        let uri_str = format!("{}/api/v1/budgets/{id}/attachments", configuration.base_path, id=id);
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

    async fn list_budget(&self, page: Option<i32>, start: Option<String>, end: Option<String>) -> Result<crate::models::BudgetArray, Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let client = &configuration.client;

        let uri_str = format!("{}/api/v1/budgets", configuration.base_path);
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

    async fn list_budget_limit_by_budget(&self, id: i32, start: Option<String>, end: Option<String>) -> Result<crate::models::BudgetLimitArray, Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let client = &configuration.client;

        let uri_str = format!("{}/api/v1/budgets/{id}/limits", configuration.base_path, id=id);
        let mut req_builder = client.request(::reqwest::Method::GET, uri_str.as_str());

        if let Some(ref s) = start {
            req_builder = req_builder.query(&[("start", &s.to_string())]);
        }
        if let Some(ref s) = end {
            req_builder = req_builder.query(&[("end", &s.to_string())]);
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

    async fn list_transaction_by_budget(&self, id: i32, limit: Option<i32>, page: Option<i32>, start: Option<String>, end: Option<String>, _type: Option<crate::models::TransactionTypeFilter>) -> Result<crate::models::TransactionArray, Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let client = &configuration.client;

        let uri_str = format!("{}/api/v1/budgets/{id}/transactions", configuration.base_path, id=id);
        let mut req_builder = client.request(::reqwest::Method::GET, uri_str.as_str());

        if let Some(ref s) = limit {
            req_builder = req_builder.query(&[("limit", &s.to_string())]);
        }
        if let Some(ref s) = page {
            req_builder = req_builder.query(&[("page", &s.to_string())]);
        }
        if let Some(ref s) = start {
            req_builder = req_builder.query(&[("start", &s.to_string())]);
        }
        if let Some(ref s) = end {
            req_builder = req_builder.query(&[("end", &s.to_string())]);
        }
        if let Some(ref s) = _type {
            req_builder = req_builder.query(&[("type", &s.to_string())]);
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

    async fn list_transaction_by_budget_limit(&self, id: i32, page: Option<i32>, _type: Option<crate::models::TransactionTypeFilter>) -> Result<crate::models::TransactionArray, Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let client = &configuration.client;

        let uri_str = format!("{}/api/v1/budgets/limits/{id}/transactions", configuration.base_path, id=id);
        let mut req_builder = client.request(::reqwest::Method::GET, uri_str.as_str());

        if let Some(ref s) = page {
            req_builder = req_builder.query(&[("page", &s.to_string())]);
        }
        if let Some(ref s) = _type {
            req_builder = req_builder.query(&[("type", &s.to_string())]);
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

    async fn store_budget(&self, budget: crate::models::Budget) -> Result<crate::models::BudgetSingle, Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let client = &configuration.client;

        let uri_str = format!("{}/api/v1/budgets", configuration.base_path);
        let mut req_builder = client.request(::reqwest::Method::POST, uri_str.as_str());

        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }
        if let Some(ref token) = configuration.oauth_access_token {
            req_builder = req_builder.bearer_auth(token.to_owned());
        };
        req_builder = req_builder.json(&budget);

        // send request
        let req = req_builder.build()?;

        Ok(client.execute(req).await?.error_for_status()?.json().await?)
    }

    async fn store_budget_limit(&self, id: i32, budget_limit: crate::models::BudgetLimit) -> Result<crate::models::BudgetLimitSingle, Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let client = &configuration.client;

        let uri_str = format!("{}/api/v1/budgets/{id}/limits", configuration.base_path, id=id);
        let mut req_builder = client.request(::reqwest::Method::POST, uri_str.as_str());

        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }
        if let Some(ref token) = configuration.oauth_access_token {
            req_builder = req_builder.bearer_auth(token.to_owned());
        };
        req_builder = req_builder.json(&budget_limit);

        // send request
        let req = req_builder.build()?;

        Ok(client.execute(req).await?.error_for_status()?.json().await?)
    }

    async fn update_budget(&self, id: i32, budget: crate::models::Budget) -> Result<crate::models::BudgetSingle, Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let client = &configuration.client;

        let uri_str = format!("{}/api/v1/budgets/{id}", configuration.base_path, id=id);
        let mut req_builder = client.request(::reqwest::Method::PUT, uri_str.as_str());

        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }
        if let Some(ref token) = configuration.oauth_access_token {
            req_builder = req_builder.bearer_auth(token.to_owned());
        };
        req_builder = req_builder.json(&budget);

        // send request
        let req = req_builder.build()?;

        Ok(client.execute(req).await?.error_for_status()?.json().await?)
    }

    async fn update_budget_limit(&self, id: i32, budget_limit: crate::models::BudgetLimit) -> Result<crate::models::BudgetLimitSingle, Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let client = &configuration.client;

        let uri_str = format!("{}/api/v1/budgets/limits/{id}", configuration.base_path, id=id);
        let mut req_builder = client.request(::reqwest::Method::PUT, uri_str.as_str());

        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }
        if let Some(ref token) = configuration.oauth_access_token {
            req_builder = req_builder.bearer_auth(token.to_owned());
        };
        req_builder = req_builder.json(&budget_limit);

        // send request
        let req = req_builder.build()?;

        Ok(client.execute(req).await?.error_for_status()?.json().await?)
    }

}
