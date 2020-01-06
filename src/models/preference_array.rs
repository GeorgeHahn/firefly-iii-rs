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
pub struct PreferenceArray {
    #[serde(rename = "data")]
    pub data: Vec<crate::models::PreferenceRead>,
    #[serde(rename = "meta")]
    pub meta: crate::models::Meta,
    #[serde(rename = "links")]
    pub links: crate::models::PageLink,
}

impl PreferenceArray {
    pub fn new(data: Vec<crate::models::PreferenceRead>, meta: crate::models::Meta, links: crate::models::PageLink) -> PreferenceArray {
        PreferenceArray {
            data,
            meta,
            links,
        }
    }
}


