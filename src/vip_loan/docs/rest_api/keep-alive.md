# Keep-Alive Configuration

```rust
use binance_sdk::vip_loan;
use binance_sdk::config;

let configuration = config::ConfigurationRestApi::builder()
    .api_key("your-api-key")
    .api_secret("your-api-secret")
    .keep_alive(false) // default is true
    .build()?;

let client = vip_loan::VIPLoanRestApi::production(configuration);
let params = vip_loan::rest_api::GetCollateralAssetDataParams::default();
let response = client.get_collateral_asset_data(params).await?;
```
