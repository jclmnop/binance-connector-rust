/*
 * Binance Rebate REST API
 *
 * OpenAPI Specification for the Binance Rebate REST API
 *
 * The version of the OpenAPI document: 1.0.0
 *
 *
 * NOTE: This class is auto generated by OpenAPI Generator (https://openapi-generator.tech).
 * https://openapi-generator.tech
 * Do not edit the class manually.
 */

#![allow(unused_imports)]
use crate::rebate::rest_api::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct GetSpotRebateHistoryRecordsResponse {
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(rename = "code", skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(rename = "data", skip_serializing_if = "Option::is_none")]
    pub data: Option<Box<models::GetSpotRebateHistoryRecordsResponseData>>,
}

impl GetSpotRebateHistoryRecordsResponse {
    #[must_use]
    pub fn new() -> GetSpotRebateHistoryRecordsResponse {
        GetSpotRebateHistoryRecordsResponse {
            status: None,
            r#type: None,
            code: None,
            data: None,
        }
    }
}
