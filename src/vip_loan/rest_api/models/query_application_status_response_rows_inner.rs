/*
 * Binance VIP Loan REST API
 *
 * OpenAPI Specification for the Binance VIP Loan REST API
 *
 * The version of the OpenAPI document: 1.0.0
 *
 *
 * NOTE: This class is auto generated by OpenAPI Generator (https://openapi-generator.tech).
 * https://openapi-generator.tech
 * Do not edit the class manually.
 */

#![allow(unused_imports)]
use crate::vip_loan::rest_api::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct QueryApplicationStatusResponseRowsInner {
    #[serde(rename = "loanAccountId", skip_serializing_if = "Option::is_none")]
    pub loan_account_id: Option<String>,
    #[serde(rename = "orderId", skip_serializing_if = "Option::is_none")]
    pub order_id: Option<String>,
    #[serde(rename = "requestId", skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[serde(rename = "loanCoin", skip_serializing_if = "Option::is_none")]
    pub loan_coin: Option<String>,
    #[serde(rename = "loanAmount", skip_serializing_if = "Option::is_none")]
    pub loan_amount: Option<String>,
    #[serde(
        rename = "collateralAccountId",
        skip_serializing_if = "Option::is_none"
    )]
    pub collateral_account_id: Option<String>,
    #[serde(rename = "collateralCoin", skip_serializing_if = "Option::is_none")]
    pub collateral_coin: Option<String>,
    #[serde(rename = "loanTerm", skip_serializing_if = "Option::is_none")]
    pub loan_term: Option<String>,
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "loanDate", skip_serializing_if = "Option::is_none")]
    pub loan_date: Option<String>,
}

impl QueryApplicationStatusResponseRowsInner {
    #[must_use]
    pub fn new() -> QueryApplicationStatusResponseRowsInner {
        QueryApplicationStatusResponseRowsInner {
            loan_account_id: None,
            order_id: None,
            request_id: None,
            loan_coin: None,
            loan_amount: None,
            collateral_account_id: None,
            collateral_coin: None,
            loan_term: None,
            status: None,
            loan_date: None,
        }
    }
}
