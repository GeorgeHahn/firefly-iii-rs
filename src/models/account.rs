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
pub struct Account {
    #[serde(rename = "created_at", skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(rename = "updated_at", skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
    #[serde(rename = "name")]
    pub name: String,
    /// Can only be one one these account types. import, initial-balance and reconciliation cannot be set manually.
    #[serde(rename = "type")]
    pub _type: Type,
    #[serde(rename = "iban", skip_serializing_if = "Option::is_none")]
    pub iban: Option<String>,
    #[serde(rename = "bic", skip_serializing_if = "Option::is_none")]
    pub bic: Option<String>,
    #[serde(rename = "account_number", skip_serializing_if = "Option::is_none")]
    pub account_number: Option<String>,
    #[serde(rename = "opening_balance", skip_serializing_if = "Option::is_none")]
    pub opening_balance: Option<f64>,
    #[serde(rename = "opening_balance_date", skip_serializing_if = "Option::is_none")]
    pub opening_balance_date: Option<String>,
    #[serde(rename = "virtual_balance", skip_serializing_if = "Option::is_none")]
    pub virtual_balance: Option<f64>,
    #[serde(rename = "current_balance", skip_serializing_if = "Option::is_none")]
    pub current_balance: Option<f64>,
    #[serde(rename = "current_balance_date", skip_serializing_if = "Option::is_none")]
    pub current_balance_date: Option<String>,
    /// Use either currency_id or currency_code. Defaults to the user's default currency.
    #[serde(rename = "currency_id", skip_serializing_if = "Option::is_none")]
    pub currency_id: Option<i32>,
    /// Use either currency_id or currency_code. Defaults to the user's default currency.
    #[serde(rename = "currency_code", skip_serializing_if = "Option::is_none")]
    pub currency_code: Option<String>,
    #[serde(rename = "currency_symbol", skip_serializing_if = "Option::is_none")]
    pub currency_symbol: Option<String>,
    #[serde(rename = "currency_decimal_places", skip_serializing_if = "Option::is_none")]
    pub currency_decimal_places: Option<i32>,
    /// If omitted, defaults to true.
    #[serde(rename = "active", skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
    /// If omitted, defaults to true.
    #[serde(rename = "include_net_worth", skip_serializing_if = "Option::is_none")]
    pub include_net_worth: Option<bool>,
    /// Is only mandatory when the type is asset.
    #[serde(rename = "account_role", skip_serializing_if = "Option::is_none")]
    pub account_role: Option<AccountRole>,
    /// Mandatory when the account_role is ccAsset. Can only be monthlyFull.
    #[serde(rename = "credit_card_type", skip_serializing_if = "Option::is_none")]
    pub credit_card_type: Option<CreditCardType>,
    /// Mandatory when the account_role is ccAsset. Moment at which CC payment installments are asked for by the bank.
    #[serde(rename = "monthly_payment_date", skip_serializing_if = "Option::is_none")]
    pub monthly_payment_date: Option<String>,
    /// Mandatory when type is liability. Specifies the exact type.
    #[serde(rename = "liability_type", skip_serializing_if = "Option::is_none")]
    pub liability_type: Option<LiabilityType>,
    /// Mandatory when type is liability. Amount of money in the liability. Must be positive.
    #[serde(rename = "liability_amount", skip_serializing_if = "Option::is_none")]
    pub liability_amount: Option<f64>,
    /// Mandatory when type is liability. Start date for the liability.
    #[serde(rename = "liability_start_date", skip_serializing_if = "Option::is_none")]
    pub liability_start_date: Option<String>,
    /// Mandatory when type is liability. Interest percentage.
    #[serde(rename = "interest", skip_serializing_if = "Option::is_none")]
    pub interest: Option<String>,
    /// Mandatory when type is liability. Period over which the interest is calculated.
    #[serde(rename = "interest_period", skip_serializing_if = "Option::is_none")]
    pub interest_period: Option<InterestPeriod>,
    #[serde(rename = "notes", skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
}

impl Account {
    pub fn new(name: String, _type: Type) -> Account {
        Account {
            created_at: None,
            updated_at: None,
            name,
            _type,
            iban: None,
            bic: None,
            account_number: None,
            opening_balance: None,
            opening_balance_date: None,
            virtual_balance: None,
            current_balance: None,
            current_balance_date: None,
            currency_id: None,
            currency_code: None,
            currency_symbol: None,
            currency_decimal_places: None,
            active: None,
            include_net_worth: None,
            account_role: None,
            credit_card_type: None,
            monthly_payment_date: None,
            liability_type: None,
            liability_amount: None,
            liability_start_date: None,
            interest: None,
            interest_period: None,
            notes: None,
        }
    }
}

/// Can only be one one these account types. import, initial-balance and reconciliation cannot be set manually.
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "asset")]
    Asset,
    #[serde(rename = "expense")]
    Expense,
    #[serde(rename = "import")]
    Import,
    #[serde(rename = "revenue")]
    Revenue,
    #[serde(rename = "cash")]
    Cash,
    #[serde(rename = "liability")]
    Liability,
    #[serde(rename = "liabilities")]
    Liabilities,
    #[serde(rename = "initial-balance")]
    InitialBalance,
    #[serde(rename = "reconciliation")]
    Reconciliation,
}
/// Is only mandatory when the type is asset.
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum AccountRole {
    #[serde(rename = "defaultAsset")]
    DefaultAsset,
    #[serde(rename = "sharedAsset")]
    SharedAsset,
    #[serde(rename = "savingAsset")]
    SavingAsset,
    #[serde(rename = "ccAsset")]
    CcAsset,
    #[serde(rename = "cashWalletAsset")]
    CashWalletAsset,
}
/// Mandatory when the account_role is ccAsset. Can only be monthlyFull.
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum CreditCardType {
    #[serde(rename = "monthlyFull")]
    MonthlyFull,
}
/// Mandatory when type is liability. Specifies the exact type.
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum LiabilityType {
    #[serde(rename = "loan")]
    Loan,
    #[serde(rename = "debt")]
    Debt,
    #[serde(rename = "mortgage")]
    Mortgage,
}
/// Mandatory when type is liability. Period over which the interest is calculated.
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum InterestPeriod {
    #[serde(rename = "daily")]
    Daily,
    #[serde(rename = "monthly")]
    Monthly,
    #[serde(rename = "yearly")]
    Yearly,
}

