/*
 * Binance Margin Trading REST API
 *
 * OpenAPI Specification for the Binance Margin Trading REST API
 *
 * The version of the OpenAPI document: 1.0.0
 *
 *
 * NOTE: This class is auto generated by OpenAPI Generator (https://openapi-generator.tech).
 * https://openapi-generator.tech
 * Do not edit the class manually.
 */

#![allow(unused_imports)]
use crate::margin_trading::rest_api::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct QueryCrossMarginAccountDetailsResponse {
    #[serde(rename = "created", skip_serializing_if = "Option::is_none")]
    pub created: Option<bool>,
    #[serde(rename = "borrowEnabled", skip_serializing_if = "Option::is_none")]
    pub borrow_enabled: Option<bool>,
    #[serde(rename = "marginLevel", skip_serializing_if = "Option::is_none")]
    pub margin_level: Option<String>,
    #[serde(
        rename = "collateralMarginLevel",
        skip_serializing_if = "Option::is_none"
    )]
    pub collateral_margin_level: Option<String>,
    #[serde(rename = "totalAssetOfBtc", skip_serializing_if = "Option::is_none")]
    pub total_asset_of_btc: Option<String>,
    #[serde(
        rename = "totalLiabilityOfBtc",
        skip_serializing_if = "Option::is_none"
    )]
    pub total_liability_of_btc: Option<String>,
    #[serde(rename = "totalNetAssetOfBtc", skip_serializing_if = "Option::is_none")]
    pub total_net_asset_of_btc: Option<String>,
    #[serde(
        rename = "TotalCollateralValueInUSDT",
        skip_serializing_if = "Option::is_none"
    )]
    pub total_collateral_value_in_usdt: Option<String>,
    #[serde(
        rename = "totalOpenOrderLossInUSDT",
        skip_serializing_if = "Option::is_none"
    )]
    pub total_open_order_loss_in_usdt: Option<String>,
    #[serde(rename = "tradeEnabled", skip_serializing_if = "Option::is_none")]
    pub trade_enabled: Option<bool>,
    #[serde(rename = "transferInEnabled", skip_serializing_if = "Option::is_none")]
    pub transfer_in_enabled: Option<bool>,
    #[serde(rename = "transferOutEnabled", skip_serializing_if = "Option::is_none")]
    pub transfer_out_enabled: Option<bool>,
    #[serde(rename = "accountType", skip_serializing_if = "Option::is_none")]
    pub account_type: Option<String>,
    #[serde(rename = "userAssets", skip_serializing_if = "Option::is_none")]
    pub user_assets: Option<Vec<models::QueryCrossMarginAccountDetailsResponseUserAssetsInner>>,
}

impl QueryCrossMarginAccountDetailsResponse {
    #[must_use]
    pub fn new() -> QueryCrossMarginAccountDetailsResponse {
        QueryCrossMarginAccountDetailsResponse {
            created: None,
            borrow_enabled: None,
            margin_level: None,
            collateral_margin_level: None,
            total_asset_of_btc: None,
            total_liability_of_btc: None,
            total_net_asset_of_btc: None,
            total_collateral_value_in_usdt: None,
            total_open_order_loss_in_usdt: None,
            trade_enabled: None,
            transfer_in_enabled: None,
            transfer_out_enabled: None,
            account_type: None,
            user_assets: None,
        }
    }
}
