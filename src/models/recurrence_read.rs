/*
 * Firefly III API
 *
 * This is the official documentation of the Firefly III API. You can find accompanying documentation on the website of Firefly III itself (see below). Please report any bugs or issues. This version of the API is live from version v4.7.9 and onwards. You may use the \"Authorize\" button to try the API below. 
 *
 * The version of the OpenAPI document: 0.10.4
 * Contact: thegrumpydictator@gmail.com
 * Generated by: https://openapi-generator.tech
 */


use serde::{Deserialize, Serialize};



#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct RecurrenceRead {
    /// Immutable value
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub _type: Option<String>,
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[serde(rename = "attributes", skip_serializing_if = "Option::is_none")]
    pub attributes: Option<crate::models::Recurrence>,
    #[serde(rename = "links", skip_serializing_if = "Option::is_none")]
    pub links: Option<crate::models::ObjectLink>,
}

impl RecurrenceRead {
    pub fn new() -> RecurrenceRead {
        RecurrenceRead {
            _type: None,
            id: None,
            attributes: None,
            links: None,
        }
    }
}


