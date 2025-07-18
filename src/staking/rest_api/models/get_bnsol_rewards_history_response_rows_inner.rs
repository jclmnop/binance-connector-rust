/*
 * Binance Staking REST API
 *
 * OpenAPI Specification for the Binance Staking REST API
 *
 * The version of the OpenAPI document: 1.0.0
 *
 *
 * NOTE: This class is auto generated by OpenAPI Generator (https://openapi-generator.tech).
 * https://openapi-generator.tech
 * Do not edit the class manually.
 */

#![allow(unused_imports)]
use crate::staking::rest_api::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct GetBnsolRewardsHistoryResponseRowsInner {
    #[serde(rename = "time", skip_serializing_if = "Option::is_none")]
    pub time: Option<i64>,
    #[serde(rename = "amountInSOL", skip_serializing_if = "Option::is_none")]
    pub amount_in_sol: Option<String>,
    #[serde(rename = "holding", skip_serializing_if = "Option::is_none")]
    pub holding: Option<String>,
    #[serde(rename = "holdingInSOL", skip_serializing_if = "Option::is_none")]
    pub holding_in_sol: Option<String>,
    #[serde(
        rename = "annualPercentageRate",
        skip_serializing_if = "Option::is_none"
    )]
    pub annual_percentage_rate: Option<String>,
}

impl GetBnsolRewardsHistoryResponseRowsInner {
    #[must_use]
    pub fn new() -> GetBnsolRewardsHistoryResponseRowsInner {
        GetBnsolRewardsHistoryResponseRowsInner {
            time: None,
            amount_in_sol: None,
            holding: None,
            holding_in_sol: None,
            annual_percentage_rate: None,
        }
    }
}
