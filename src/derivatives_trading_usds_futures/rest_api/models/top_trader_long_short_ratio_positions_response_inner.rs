/*
 * Binance Derivatives Trading USDS Futures REST API
 *
 * OpenAPI Specification for the Binance Derivatives Trading USDS Futures REST API
 *
 * The version of the OpenAPI document: 1.0.0
 *
 *
 * NOTE: This class is auto generated by OpenAPI Generator (https://openapi-generator.tech).
 * https://openapi-generator.tech
 * Do not edit the class manually.
 */

#![allow(unused_imports)]
use crate::derivatives_trading_usds_futures::rest_api::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct TopTraderLongShortRatioPositionsResponseInner {
    #[serde(rename = "symbol", skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,
    #[serde(rename = "longShortRatio", skip_serializing_if = "Option::is_none")]
    pub long_short_ratio: Option<String>,
    #[serde(rename = "longAccount", skip_serializing_if = "Option::is_none")]
    pub long_account: Option<String>,
    #[serde(rename = "shortAccount", skip_serializing_if = "Option::is_none")]
    pub short_account: Option<String>,
    #[serde(rename = "timestamp", skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<String>,
}

impl TopTraderLongShortRatioPositionsResponseInner {
    #[must_use]
    pub fn new() -> TopTraderLongShortRatioPositionsResponseInner {
        TopTraderLongShortRatioPositionsResponseInner {
            symbol: None,
            long_short_ratio: None,
            long_account: None,
            short_account: None,
            timestamp: None,
        }
    }
}
