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
pub struct Rule {
    #[serde(rename = "created_at", skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(rename = "updated_at", skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
    #[serde(rename = "title")]
    pub title: String,
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// ID of the rule group under which the rule must be stored. Either this field or rule_group_title is mandatory.
    #[serde(rename = "rule_group_id")]
    pub rule_group_id: i32,
    /// Title of the rule group under which the rule must be stored. Either this field or rule_group_id is mandatory.
    #[serde(rename = "rule_group_title", skip_serializing_if = "Option::is_none")]
    pub rule_group_title: Option<String>,
    #[serde(rename = "order", skip_serializing_if = "Option::is_none")]
    pub order: Option<i32>,
    /// Which action is necessary for the rule to fire? Use either store-journal or update-journal.
    #[serde(rename = "trigger")]
    pub trigger: Trigger,
    /// Whether or not the rule is even active. Default is true.
    #[serde(rename = "active", skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
    /// If the rule is set to be strict, ALL triggers must hit in order for the rule to fire. Otherwise, just one is enough. Default value is true.
    #[serde(rename = "strict", skip_serializing_if = "Option::is_none")]
    pub strict: Option<bool>,
    /// If this value is true and the rule is triggered, other rules  after this one in the group will be skipped. Default value is false.
    #[serde(rename = "stop_processing", skip_serializing_if = "Option::is_none")]
    pub stop_processing: Option<bool>,
    #[serde(rename = "triggers")]
    pub triggers: Vec<crate::models::RuleTrigger>,
    #[serde(rename = "actions")]
    pub actions: Vec<crate::models::RuleAction>,
}

impl Rule {
    pub fn new(title: String, rule_group_id: i32, trigger: Trigger, triggers: Vec<crate::models::RuleTrigger>, actions: Vec<crate::models::RuleAction>) -> Rule {
        Rule {
            created_at: None,
            updated_at: None,
            title,
            description: None,
            rule_group_id,
            rule_group_title: None,
            order: None,
            trigger,
            active: None,
            strict: None,
            stop_processing: None,
            triggers,
            actions,
        }
    }
}

/// Which action is necessary for the rule to fire? Use either store-journal or update-journal.
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum Trigger {
    #[serde(rename = "store-journal")]
    StoreJournal,
    #[serde(rename = "update-journal")]
    UpdateJournal,
}

