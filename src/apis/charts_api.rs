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

pub struct ChartsApiClient {
    configuration: Arc<configuration::Configuration>,
}

impl ChartsApiClient {
    pub fn new(configuration: Arc<configuration::Configuration>) -> ChartsApiClient {
        ChartsApiClient {
            configuration,
        }
    }
}

#[async_trait]
pub trait ChartsApi {
    async fn get_chart_ab_overview(&self, id: i32, start: String, end: String) -> Result<Vec<crate::models::ChartDataSet>, Error>;
    async fn get_chart_account_expense(&self, start: String, end: String) -> Result<Vec<crate::models::ChartDataSet>, Error>;
    async fn get_chart_account_overview(&self, start: String, end: String) -> Result<Vec<crate::models::ChartDataSet>, Error>;
    async fn get_chart_account_revenue(&self, start: String, end: String) -> Result<Vec<crate::models::ChartDataSet>, Error>;
    async fn get_chart_category_overview(&self, start: String, end: String) -> Result<Vec<crate::models::ChartDataSet>, Error>;
}

#[async_trait]
impl ChartsApi for ChartsApiClient {
    async fn get_chart_ab_overview(&self, id: i32, start: String, end: String) -> Result<Vec<crate::models::ChartDataSet>, Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let client = &configuration.client;

        let uri_str = format!("{}/api/v1/chart/ab/overview/{id}", configuration.base_path, id=id);
        let mut req_builder = client.request(reqwest::Method::GET, uri_str.as_str());

        req_builder = req_builder.query(&[("start", &start.to_string())]);
        req_builder = req_builder.query(&[("end", &end.to_string())]);
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

    async fn get_chart_account_expense(&self, start: String, end: String) -> Result<Vec<crate::models::ChartDataSet>, Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let client = &configuration.client;

        let uri_str = format!("{}/api/v1/chart/account/expense", configuration.base_path);
        let mut req_builder = client.request(reqwest::Method::GET, uri_str.as_str());

        req_builder = req_builder.query(&[("start", &start.to_string())]);
        req_builder = req_builder.query(&[("end", &end.to_string())]);
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

    async fn get_chart_account_overview(&self, start: String, end: String) -> Result<Vec<crate::models::ChartDataSet>, Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let client = &configuration.client;

        let uri_str = format!("{}/api/v1/chart/account/overview", configuration.base_path);
        let mut req_builder = client.request(reqwest::Method::GET, uri_str.as_str());

        req_builder = req_builder.query(&[("start", &start.to_string())]);
        req_builder = req_builder.query(&[("end", &end.to_string())]);
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

    async fn get_chart_account_revenue(&self, start: String, end: String) -> Result<Vec<crate::models::ChartDataSet>, Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let client = &configuration.client;

        let uri_str = format!("{}/api/v1/chart/account/revenue", configuration.base_path);
        let mut req_builder = client.request(reqwest::Method::GET, uri_str.as_str());

        req_builder = req_builder.query(&[("start", &start.to_string())]);
        req_builder = req_builder.query(&[("end", &end.to_string())]);
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

    async fn get_chart_category_overview(&self, start: String, end: String) -> Result<Vec<crate::models::ChartDataSet>, Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let client = &configuration.client;

        let uri_str = format!("{}/api/v1/chart/category/overview", configuration.base_path);
        let mut req_builder = client.request(reqwest::Method::GET, uri_str.as_str());

        req_builder = req_builder.query(&[("start", &start.to_string())]);
        req_builder = req_builder.query(&[("end", &end.to_string())]);
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

}
