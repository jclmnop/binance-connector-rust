/*
 * Binance Derivatives Trading Portfolio Margin REST API
 *
 * OpenAPI Specification for the Binance Derivatives Trading Portfolio Margin REST API
 *
 * The version of the OpenAPI document: 1.0.0
 *
 *
 * NOTE: This class is auto generated by OpenAPI Generator (https://openapi-generator.tech).
 * https://openapi-generator.tech
 * Do not edit the class manually.
 */

#![allow(unused_imports)]
use crate::derivatives_trading_portfolio_margin::rest_api::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct CmPositionAdlQuantileEstimationResponseInnerAdlQuantile {
    #[serde(rename = "LONG", skip_serializing_if = "Option::is_none")]
    pub long: Option<i64>,
    #[serde(rename = "SHORT", skip_serializing_if = "Option::is_none")]
    pub short: Option<i64>,
    #[serde(rename = "HEDGE", skip_serializing_if = "Option::is_none")]
    pub hedge: Option<i64>,
    #[serde(rename = "BOTH", skip_serializing_if = "Option::is_none")]
    pub both: Option<i64>,
}

impl CmPositionAdlQuantileEstimationResponseInnerAdlQuantile {
    #[must_use]
    pub fn new() -> CmPositionAdlQuantileEstimationResponseInnerAdlQuantile {
        CmPositionAdlQuantileEstimationResponseInnerAdlQuantile {
            long: None,
            short: None,
            hedge: None,
            both: None,
        }
    }
}
