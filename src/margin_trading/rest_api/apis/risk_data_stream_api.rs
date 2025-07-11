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
use async_trait::async_trait;
use derive_builder::Builder;
use reqwest;
use rust_decimal::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use std::collections::BTreeMap;

use crate::common::{
    config::ConfigurationRestApi,
    models::{ParamBuildError, RestApiResponse},
    utils::send_request,
};
use crate::margin_trading::rest_api::models;

const HAS_TIME_UNIT: bool = false;

#[async_trait]
pub trait RiskDataStreamApi: Send + Sync {
    async fn close_user_data_stream(&self) -> anyhow::Result<RestApiResponse<Value>>;
    async fn keepalive_user_data_stream(
        &self,
        params: KeepaliveUserDataStreamParams,
    ) -> anyhow::Result<RestApiResponse<Value>>;
    async fn start_user_data_stream(
        &self,
    ) -> anyhow::Result<RestApiResponse<models::StartUserDataStreamResponse>>;
}

#[derive(Debug, Clone)]
pub struct RiskDataStreamApiClient {
    configuration: ConfigurationRestApi,
}

impl RiskDataStreamApiClient {
    pub fn new(configuration: ConfigurationRestApi) -> Self {
        Self { configuration }
    }
}

/// Request parameters for the [`keepalive_user_data_stream`] operation.
///
/// This struct holds all of the inputs you can pass when calling
/// [`keepalive_user_data_stream`](#method.keepalive_user_data_stream).
#[derive(Clone, Debug, Builder)]
#[builder(pattern = "owned", build_fn(error = "ParamBuildError"))]
pub struct KeepaliveUserDataStreamParams {
    ///
    /// The `listen_key` parameter.
    ///
    /// This field is **required.
    #[builder(setter(into))]
    pub listen_key: String,
}

impl KeepaliveUserDataStreamParams {
    /// Create a builder for [`keepalive_user_data_stream`].
    ///
    /// Required parameters:
    ///
    /// * `listen_key` — String
    ///
    #[must_use]
    pub fn builder(listen_key: String) -> KeepaliveUserDataStreamParamsBuilder {
        KeepaliveUserDataStreamParamsBuilder::default().listen_key(listen_key)
    }
}

#[async_trait]
impl RiskDataStreamApi for RiskDataStreamApiClient {
    async fn close_user_data_stream(&self) -> anyhow::Result<RestApiResponse<Value>> {
        let query_params = BTreeMap::new();

        send_request::<Value>(
            &self.configuration,
            "/sapi/v1/margin/listen-key",
            reqwest::Method::DELETE,
            query_params,
            if HAS_TIME_UNIT {
                self.configuration.time_unit
            } else {
                None
            },
            false,
        )
        .await
    }

    async fn keepalive_user_data_stream(
        &self,
        params: KeepaliveUserDataStreamParams,
    ) -> anyhow::Result<RestApiResponse<Value>> {
        let KeepaliveUserDataStreamParams { listen_key } = params;

        let mut query_params = BTreeMap::new();

        query_params.insert("listenKey".to_string(), json!(listen_key));

        send_request::<Value>(
            &self.configuration,
            "/sapi/v1/margin/listen-key",
            reqwest::Method::PUT,
            query_params,
            if HAS_TIME_UNIT {
                self.configuration.time_unit
            } else {
                None
            },
            false,
        )
        .await
    }

    async fn start_user_data_stream(
        &self,
    ) -> anyhow::Result<RestApiResponse<models::StartUserDataStreamResponse>> {
        let query_params = BTreeMap::new();

        send_request::<models::StartUserDataStreamResponse>(
            &self.configuration,
            "/sapi/v1/margin/listen-key",
            reqwest::Method::POST,
            query_params,
            if HAS_TIME_UNIT {
                self.configuration.time_unit
            } else {
                None
            },
            false,
        )
        .await
    }
}

#[cfg(all(test, feature = "margin_trading"))]
mod tests {
    use super::*;
    use crate::TOKIO_SHARED_RT;
    use crate::{errors::ConnectorError, models::DataFuture, models::RestApiRateLimit};
    use async_trait::async_trait;
    use std::collections::HashMap;

    struct DummyRestApiResponse<T> {
        inner: Box<dyn FnOnce() -> DataFuture<Result<T, ConnectorError>> + Send + Sync>,
        status: u16,
        headers: HashMap<String, String>,
        rate_limits: Option<Vec<RestApiRateLimit>>,
    }

    impl<T> From<DummyRestApiResponse<T>> for RestApiResponse<T> {
        fn from(dummy: DummyRestApiResponse<T>) -> Self {
            Self {
                data_fn: dummy.inner,
                status: dummy.status,
                headers: dummy.headers,
                rate_limits: dummy.rate_limits,
            }
        }
    }

    struct MockRiskDataStreamApiClient {
        force_error: bool,
    }

    #[async_trait]
    impl RiskDataStreamApi for MockRiskDataStreamApiClient {
        async fn close_user_data_stream(&self) -> anyhow::Result<RestApiResponse<Value>> {
            if self.force_error {
                return Err(
                    ConnectorError::ConnectorClientError("ResponseError".to_string()).into(),
                );
            }

            let dummy_response = Value::Null;

            let dummy = DummyRestApiResponse {
                inner: Box::new(move || Box::pin(async move { Ok(dummy_response) })),
                status: 200,
                headers: HashMap::new(),
                rate_limits: None,
            };

            Ok(dummy.into())
        }

        async fn keepalive_user_data_stream(
            &self,
            _params: KeepaliveUserDataStreamParams,
        ) -> anyhow::Result<RestApiResponse<Value>> {
            if self.force_error {
                return Err(
                    ConnectorError::ConnectorClientError("ResponseError".to_string()).into(),
                );
            }

            let dummy_response = Value::Null;

            let dummy = DummyRestApiResponse {
                inner: Box::new(move || Box::pin(async move { Ok(dummy_response) })),
                status: 200,
                headers: HashMap::new(),
                rate_limits: None,
            };

            Ok(dummy.into())
        }

        async fn start_user_data_stream(
            &self,
        ) -> anyhow::Result<RestApiResponse<models::StartUserDataStreamResponse>> {
            if self.force_error {
                return Err(
                    ConnectorError::ConnectorClientError("ResponseError".to_string()).into(),
                );
            }

            let resp_json: Value = serde_json::from_str(
                r#"{"listenKey":"T3ee22BIYuWqmvne0HNq2A2WsFlEtLhvWCtItw6ffhhd"}"#,
            )
            .unwrap();
            let dummy_response: models::StartUserDataStreamResponse =
                serde_json::from_value(resp_json.clone())
                    .expect("should parse into models::StartUserDataStreamResponse");

            let dummy = DummyRestApiResponse {
                inner: Box::new(move || Box::pin(async move { Ok(dummy_response) })),
                status: 200,
                headers: HashMap::new(),
                rate_limits: None,
            };

            Ok(dummy.into())
        }
    }

    #[test]
    fn close_user_data_stream_required_params_success() {
        TOKIO_SHARED_RT.block_on(async {
            let client = MockRiskDataStreamApiClient { force_error: false };

            let expected_response = Value::Null;

            let resp = client
                .close_user_data_stream()
                .await
                .expect("Expected a response");
            let data_future = resp.data();
            let actual_response = data_future.await.unwrap();
            assert_eq!(actual_response, expected_response);
        });
    }

    #[test]
    fn close_user_data_stream_optional_params_success() {
        TOKIO_SHARED_RT.block_on(async {
            let client = MockRiskDataStreamApiClient { force_error: false };

            let expected_response = Value::Null;

            let resp = client
                .close_user_data_stream()
                .await
                .expect("Expected a response");
            let data_future = resp.data();
            let actual_response = data_future.await.unwrap();
            assert_eq!(actual_response, expected_response);
        });
    }

    #[test]
    fn close_user_data_stream_response_error() {
        TOKIO_SHARED_RT.block_on(async {
            let client = MockRiskDataStreamApiClient { force_error: true };

            match client.close_user_data_stream().await {
                Ok(_) => panic!("Expected an error"),
                Err(err) => {
                    assert_eq!(err.to_string(), "Connector client error: ResponseError");
                }
            }
        });
    }

    #[test]
    fn keepalive_user_data_stream_required_params_success() {
        TOKIO_SHARED_RT.block_on(async {
            let client = MockRiskDataStreamApiClient { force_error: false };

            let params = KeepaliveUserDataStreamParams::builder("listen_key_example".to_string())
                .build()
                .unwrap();

            let expected_response = Value::Null;

            let resp = client
                .keepalive_user_data_stream(params)
                .await
                .expect("Expected a response");
            let data_future = resp.data();
            let actual_response = data_future.await.unwrap();
            assert_eq!(actual_response, expected_response);
        });
    }

    #[test]
    fn keepalive_user_data_stream_optional_params_success() {
        TOKIO_SHARED_RT.block_on(async {
            let client = MockRiskDataStreamApiClient { force_error: false };

            let params = KeepaliveUserDataStreamParams::builder("listen_key_example".to_string())
                .build()
                .unwrap();

            let expected_response = Value::Null;

            let resp = client
                .keepalive_user_data_stream(params)
                .await
                .expect("Expected a response");
            let data_future = resp.data();
            let actual_response = data_future.await.unwrap();
            assert_eq!(actual_response, expected_response);
        });
    }

    #[test]
    fn keepalive_user_data_stream_response_error() {
        TOKIO_SHARED_RT.block_on(async {
            let client = MockRiskDataStreamApiClient { force_error: true };

            let params = KeepaliveUserDataStreamParams::builder("listen_key_example".to_string())
                .build()
                .unwrap();

            match client.keepalive_user_data_stream(params).await {
                Ok(_) => panic!("Expected an error"),
                Err(err) => {
                    assert_eq!(err.to_string(), "Connector client error: ResponseError");
                }
            }
        });
    }

    #[test]
    fn start_user_data_stream_required_params_success() {
        TOKIO_SHARED_RT.block_on(async {
            let client = MockRiskDataStreamApiClient { force_error: false };

            let resp_json: Value = serde_json::from_str(
                r#"{"listenKey":"T3ee22BIYuWqmvne0HNq2A2WsFlEtLhvWCtItw6ffhhd"}"#,
            )
            .unwrap();
            let expected_response: models::StartUserDataStreamResponse =
                serde_json::from_value(resp_json.clone())
                    .expect("should parse into models::StartUserDataStreamResponse");

            let resp = client
                .start_user_data_stream()
                .await
                .expect("Expected a response");
            let data_future = resp.data();
            let actual_response = data_future.await.unwrap();
            assert_eq!(actual_response, expected_response);
        });
    }

    #[test]
    fn start_user_data_stream_optional_params_success() {
        TOKIO_SHARED_RT.block_on(async {
            let client = MockRiskDataStreamApiClient { force_error: false };

            let resp_json: Value = serde_json::from_str(
                r#"{"listenKey":"T3ee22BIYuWqmvne0HNq2A2WsFlEtLhvWCtItw6ffhhd"}"#,
            )
            .unwrap();
            let expected_response: models::StartUserDataStreamResponse =
                serde_json::from_value(resp_json.clone())
                    .expect("should parse into models::StartUserDataStreamResponse");

            let resp = client
                .start_user_data_stream()
                .await
                .expect("Expected a response");
            let data_future = resp.data();
            let actual_response = data_future.await.unwrap();
            assert_eq!(actual_response, expected_response);
        });
    }

    #[test]
    fn start_user_data_stream_response_error() {
        TOKIO_SHARED_RT.block_on(async {
            let client = MockRiskDataStreamApiClient { force_error: true };

            match client.start_user_data_stream().await {
                Ok(_) => panic!("Expected an error"),
                Err(err) => {
                    assert_eq!(err.to_string(), "Connector client error: ResponseError");
                }
            }
        });
    }
}
