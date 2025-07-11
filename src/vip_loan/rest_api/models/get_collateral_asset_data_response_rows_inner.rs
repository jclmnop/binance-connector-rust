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
pub struct GetCollateralAssetDataResponseRowsInner {
    #[serde(rename = "collateralCoin", skip_serializing_if = "Option::is_none")]
    pub collateral_coin: Option<String>,
    #[serde(
        rename = "_1stCollateralRatio",
        skip_serializing_if = "Option::is_none"
    )]
    pub _1st_collateral_ratio: Option<String>,
    #[serde(
        rename = "_1stCollateralRange",
        skip_serializing_if = "Option::is_none"
    )]
    pub _1st_collateral_range: Option<String>,
    #[serde(
        rename = "_2ndCollateralRatio",
        skip_serializing_if = "Option::is_none"
    )]
    pub _2nd_collateral_ratio: Option<String>,
    #[serde(
        rename = "_2ndCollateralRange",
        skip_serializing_if = "Option::is_none"
    )]
    pub _2nd_collateral_range: Option<String>,
    #[serde(
        rename = "_3rdCollateralRatio",
        skip_serializing_if = "Option::is_none"
    )]
    pub _3rd_collateral_ratio: Option<String>,
    #[serde(
        rename = "_3rdCollateralRange",
        skip_serializing_if = "Option::is_none"
    )]
    pub _3rd_collateral_range: Option<String>,
    #[serde(
        rename = "_4thCollateralRatio",
        skip_serializing_if = "Option::is_none"
    )]
    pub _4th_collateral_ratio: Option<String>,
    #[serde(
        rename = "_4thCollateralRange",
        skip_serializing_if = "Option::is_none"
    )]
    pub _4th_collateral_range: Option<String>,
}

impl GetCollateralAssetDataResponseRowsInner {
    #[must_use]
    pub fn new() -> GetCollateralAssetDataResponseRowsInner {
        GetCollateralAssetDataResponseRowsInner {
            collateral_coin: None,
            _1st_collateral_ratio: None,
            _1st_collateral_range: None,
            _2nd_collateral_ratio: None,
            _2nd_collateral_range: None,
            _3rd_collateral_ratio: None,
            _3rd_collateral_range: None,
            _4th_collateral_ratio: None,
            _4th_collateral_range: None,
        }
    }
}
