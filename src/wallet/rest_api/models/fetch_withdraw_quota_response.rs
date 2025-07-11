/*
 * Binance Wallet REST API
 *
 * OpenAPI Specification for the Binance Wallet REST API
 *
 * The version of the OpenAPI document: 1.0.0
 *
 *
 * NOTE: This class is auto generated by OpenAPI Generator (https://openapi-generator.tech).
 * https://openapi-generator.tech
 * Do not edit the class manually.
 */

#![allow(unused_imports)]
use crate::wallet::rest_api::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct FetchWithdrawQuotaResponse {
    #[serde(rename = "wdQuota", skip_serializing_if = "Option::is_none")]
    pub wd_quota: Option<String>,
    #[serde(rename = "usedWdQuota", skip_serializing_if = "Option::is_none")]
    pub used_wd_quota: Option<String>,
}

impl FetchWithdrawQuotaResponse {
    #[must_use]
    pub fn new() -> FetchWithdrawQuotaResponse {
        FetchWithdrawQuotaResponse {
            wd_quota: None,
            used_wd_quota: None,
        }
    }
}
