/*
 * Firefly III API
 *
 * This is the official documentation of the Firefly III API. You can find accompanying documentation on the website of Firefly III itself (see below). Please report any bugs or issues. This version of the API is live from version v4.7.9 and onwards. You may use the \"Authorize\" button to try the API below. 
 *
 * The version of the OpenAPI document: 1.1.0
 * Contact: james@firefly-iii.org
 * Generated by: https://openapi-generator.tech
 */


use serde::{Deserialize, Serialize};



#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ExchangeRateAttributes {
    #[serde(rename = "created_at", skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(rename = "updated_at", skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
    #[serde(rename = "from_currency_id", skip_serializing_if = "Option::is_none")]
    pub from_currency_id: Option<i32>,
    #[serde(rename = "from_currency_name", skip_serializing_if = "Option::is_none")]
    pub from_currency_name: Option<String>,
    #[serde(rename = "from_currency_code", skip_serializing_if = "Option::is_none")]
    pub from_currency_code: Option<String>,
    #[serde(rename = "from_currency_symbol", skip_serializing_if = "Option::is_none")]
    pub from_currency_symbol: Option<String>,
    #[serde(rename = "from_currency_decimal_places", skip_serializing_if = "Option::is_none")]
    pub from_currency_decimal_places: Option<i32>,
    #[serde(rename = "to_currency_id", skip_serializing_if = "Option::is_none")]
    pub to_currency_id: Option<i32>,
    #[serde(rename = "to_currency_code", skip_serializing_if = "Option::is_none")]
    pub to_currency_code: Option<String>,
    #[serde(rename = "to_currency_symbol", skip_serializing_if = "Option::is_none")]
    pub to_currency_symbol: Option<String>,
    #[serde(rename = "to_currency_decimal_places", skip_serializing_if = "Option::is_none")]
    pub to_currency_decimal_places: Option<i32>,
    #[serde(rename = "date", skip_serializing_if = "Option::is_none")]
    pub date: Option<String>,
    #[serde(rename = "rate", skip_serializing_if = "Option::is_none")]
    pub rate: Option<f32>,
    /// The amount in the \"to\"-currency, if provided in the request.
    #[serde(rename = "amount", skip_serializing_if = "Option::is_none")]
    pub amount: Option<f64>,
}

impl ExchangeRateAttributes {
    pub fn new() -> ExchangeRateAttributes {
        ExchangeRateAttributes {
            created_at: None,
            updated_at: None,
            from_currency_id: None,
            from_currency_name: None,
            from_currency_code: None,
            from_currency_symbol: None,
            from_currency_decimal_places: None,
            to_currency_id: None,
            to_currency_code: None,
            to_currency_symbol: None,
            to_currency_decimal_places: None,
            date: None,
            rate: None,
            amount: None,
        }
    }
}


