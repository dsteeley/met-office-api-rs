// spotdata_three_hourly.rs
// Spotdata example.
use anyhow::{Result, anyhow};
use met_office_api_rs::{apis::{spotdata_three_hourly_api_api::get_three_hourly_forecast_using_get1, configuration::{Configuration, ApiKey}}, models::SpotForecastFeatureCollection};
use std::env;
use dotenv::dotenv;

#[tokio::main]
async fn main() -> Result<()> {
    let mut config: Configuration = Configuration::new();


    dotenv().ok();
    let api_key = env::var("CLIENT_SECRET").expect("CLIENT_SECRET not set");
    let api_client = env::var("CLIENT_ID").expect("CLIENT_ID not set");

    config.api_key = Some(ApiKey{
        prefix: None,
        key: api_key
    });
    config.api_client = Some(ApiKey{
        prefix: None,
        key: api_client
    });
    let lat: f64 = 55.8309;
    let lon: f64 = 3.2245;
    let res: SpotForecastFeatureCollection = get_three_hourly_forecast_using_get1(&config, lat, lon, None, Some(true)).await?;

    // Give a result
    println!("One Time Series\n{:#?}", res.features.iter().next().ok_or(anyhow!("error parsing response"))?.properties.time_series.iter().next().unwrap());
    Ok(())
}
