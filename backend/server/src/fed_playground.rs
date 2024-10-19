use common::models_fmp::get_treasury_rates;

use reqwest::Error;

pub async fn get_series(api_key: String, series_id: &str) -> Result<FedSeries, Error> {
    let file_type = "json";
    let url = format!(
        "https://api.stlouisfed.org/fred/series?series_id={}&api_key={}&file_type={}",
        series_id, &api_key, &file_type
    );
    let body = reqwest::get(&url).await?.text().await?;

    let r: FedSeries = serde_json::from_str(&body).expect("should work");
    Ok(r)
}

pub async fn get_observation(api_key: String, series_id: &str) -> Result<ObservationResult, Error> {
    let file_type = "json";
    let url = format!(
        "https://api.stlouisfed.org/fred/series/observations?series_id={}&api_key={}&file_type={}",
        series_id, &api_key, &file_type
    );
    let body = reqwest::get(&url).await?.text().await?;

    let r: ObservationResult = serde_json::from_str(&body).expect("should work");
    Ok(r)
}
