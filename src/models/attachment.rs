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
pub struct Attachment {
    #[serde(rename = "created_at", skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(rename = "updated_at", skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
    #[serde(rename = "filename")]
    pub filename: String,
    /// The object class to which the attachment must be linked.
    #[serde(rename = "attachable_type")]
    pub attachable_type: AttachableType,
    /// ID of the model this attachment is linked to.
    #[serde(rename = "attachable_id")]
    pub attachable_id: i32,
    /// MD5 hash of the file for basic duplicate detection.
    #[serde(rename = "md5", skip_serializing_if = "Option::is_none")]
    pub md5: Option<String>,
    #[serde(rename = "download_uri", skip_serializing_if = "Option::is_none")]
    pub download_uri: Option<String>,
    #[serde(rename = "upload_uri", skip_serializing_if = "Option::is_none")]
    pub upload_uri: Option<String>,
    #[serde(rename = "title", skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(rename = "notes", skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
    #[serde(rename = "mime", skip_serializing_if = "Option::is_none")]
    pub mime: Option<String>,
    #[serde(rename = "size", skip_serializing_if = "Option::is_none")]
    pub size: Option<i32>,
}

impl Attachment {
    pub fn new(filename: String, attachable_type: AttachableType, attachable_id: i32) -> Attachment {
        Attachment {
            created_at: None,
            updated_at: None,
            filename,
            attachable_type,
            attachable_id,
            md5: None,
            download_uri: None,
            upload_uri: None,
            title: None,
            notes: None,
            mime: None,
            size: None,
        }
    }
}

/// The object class to which the attachment must be linked.
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum AttachableType {
    #[serde(rename = "Bill")]
    Bill,
    #[serde(rename = "TransactionJournal")]
    TransactionJournal,
    #[serde(rename = "ImportJob")]
    ImportJob,
}

