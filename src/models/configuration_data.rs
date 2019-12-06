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
pub struct ConfigurationData {
    #[serde(rename = "is_demo_site", skip_serializing_if = "Option::is_none")]
    pub is_demo_site: Option<bool>,
    /// If the user has given permission to check for updates. - null = never asked. - -1 = never asked. - 0 = no permission. - 1 = permission 
    #[serde(rename = "permission_update_check", skip_serializing_if = "Option::is_none")]
    pub permission_update_check: Option<i32>,
    #[serde(rename = "last_update_check", skip_serializing_if = "Option::is_none")]
    pub last_update_check: Option<String>,
    /// Whether other users can register.
    #[serde(rename = "single_user_mode", skip_serializing_if = "Option::is_none")]
    pub single_user_mode: Option<bool>,
}

impl ConfigurationData {
    pub fn new() -> ConfigurationData {
        ConfigurationData {
            is_demo_site: None,
            permission_update_check: None,
            last_update_check: None,
            single_user_mode: None,
        }
    }
}


