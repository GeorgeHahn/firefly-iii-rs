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
pub struct TagSingle {
    #[serde(rename = "data")]
    pub data: crate::models::TagRead,
}

impl TagSingle {
    pub fn new(data: crate::models::TagRead) -> TagSingle {
        TagSingle {
            data,
        }
    }
}


