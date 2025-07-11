/*
 * Binance Mining REST API
 *
 * OpenAPI Specification for the Binance Mining REST API
 *
 * The version of the OpenAPI document: 1.0.0
 *
 *
 * NOTE: This class is auto generated by OpenAPI Generator (https://openapi-generator.tech).
 * https://openapi-generator.tech
 * Do not edit the class manually.
 */

#![allow(unused_imports)]
use crate::mining::rest_api::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct StatisticListResponseDataProfitToday {
    #[serde(rename = "BTC", skip_serializing_if = "Option::is_none")]
    pub btc: Option<String>,
    #[serde(rename = "BSV", skip_serializing_if = "Option::is_none")]
    pub bsv: Option<String>,
    #[serde(rename = "BCH", skip_serializing_if = "Option::is_none")]
    pub bch: Option<String>,
}

impl StatisticListResponseDataProfitToday {
    #[must_use]
    pub fn new() -> StatisticListResponseDataProfitToday {
        StatisticListResponseDataProfitToday {
            btc: None,
            bsv: None,
            bch: None,
        }
    }
}
