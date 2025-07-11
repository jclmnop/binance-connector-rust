/*
 * Binance Derivatives Trading COIN Futures REST API
 *
 * OpenAPI Specification for the Binance Derivatives Trading COIN Futures REST API
 *
 * The version of the OpenAPI document: 1.0.0
 *
 *
 * NOTE: This class is auto generated by OpenAPI Generator (https://openapi-generator.tech).
 * https://openapi-generator.tech
 * Do not edit the class manually.
 */

#![allow(unused_imports)]
use crate::derivatives_trading_coin_futures::rest_api::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct GetFundingRateHistoryOfPerpetualFuturesResponseInner {
    #[serde(rename = "symbol", skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,
    #[serde(rename = "fundingTime", skip_serializing_if = "Option::is_none")]
    pub funding_time: Option<i64>,
    #[serde(rename = "fundingRate", skip_serializing_if = "Option::is_none")]
    pub funding_rate: Option<String>,
}

impl GetFundingRateHistoryOfPerpetualFuturesResponseInner {
    #[must_use]
    pub fn new() -> GetFundingRateHistoryOfPerpetualFuturesResponseInner {
        GetFundingRateHistoryOfPerpetualFuturesResponseInner {
            symbol: None,
            funding_time: None,
            funding_rate: None,
        }
    }
}
