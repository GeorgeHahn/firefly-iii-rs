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
pub struct TagCloudTag {
    /// The tag
    #[serde(rename = "tag", skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
    /// ID of the tag
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    /// The total amount of money related to this tag. There is no currency information available, and this is a basic sum of all amounts added together.
    #[serde(rename = "size", skip_serializing_if = "Option::is_none")]
    pub size: Option<f64>,
    /// A number between 0 and 1. 1 is given to the largest tag in the tag cloud, and 0 to the smallest. The rest are given a number between 0 and 1, related to their size in comparison to the largest tag.
    #[serde(rename = "relative", skip_serializing_if = "Option::is_none")]
    pub relative: Option<f64>,
}

impl TagCloudTag {
    pub fn new() -> TagCloudTag {
        TagCloudTag {
            tag: None,
            id: None,
            size: None,
            relative: None,
        }
    }
}


