/*
 * Binance Crypto Loan REST API
 *
 * OpenAPI Specification for the Binance Crypto Loan REST API
 *
 * The version of the OpenAPI document: 1.0.0
 *
 *
 * NOTE: This class is auto generated by OpenAPI Generator (https://openapi-generator.tech).
 * https://openapi-generator.tech
 * Do not edit the class manually.
 */

#![allow(unused_imports)]
use crate::crypto_loan::rest_api::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct GetFlexibleLoanLiquidationHistoryResponseRowsInner {
    #[serde(rename = "loanCoin", skip_serializing_if = "Option::is_none")]
    pub loan_coin: Option<String>,
    #[serde(rename = "liquidationDebt", skip_serializing_if = "Option::is_none")]
    pub liquidation_debt: Option<String>,
    #[serde(rename = "collateralCoin", skip_serializing_if = "Option::is_none")]
    pub collateral_coin: Option<String>,
    #[serde(
        rename = "liquidationCollateralAmount",
        skip_serializing_if = "Option::is_none"
    )]
    pub liquidation_collateral_amount: Option<String>,
    #[serde(
        rename = "returnCollateralAmount",
        skip_serializing_if = "Option::is_none"
    )]
    pub return_collateral_amount: Option<String>,
    #[serde(rename = "liquidationFee", skip_serializing_if = "Option::is_none")]
    pub liquidation_fee: Option<String>,
    #[serde(
        rename = "liquidationStartingPrice",
        skip_serializing_if = "Option::is_none"
    )]
    pub liquidation_starting_price: Option<String>,
    #[serde(
        rename = "liquidationStartingTime",
        skip_serializing_if = "Option::is_none"
    )]
    pub liquidation_starting_time: Option<i64>,
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

impl GetFlexibleLoanLiquidationHistoryResponseRowsInner {
    #[must_use]
    pub fn new() -> GetFlexibleLoanLiquidationHistoryResponseRowsInner {
        GetFlexibleLoanLiquidationHistoryResponseRowsInner {
            loan_coin: None,
            liquidation_debt: None,
            collateral_coin: None,
            liquidation_collateral_amount: None,
            return_collateral_amount: None,
            liquidation_fee: None,
            liquidation_starting_price: None,
            liquidation_starting_time: None,
            status: None,
        }
    }
}
