use common::models_fmp::get_treasury_rates;

use reqwest::Error;

pub async fn get_all_stocks(api_key: String) -> Result<Vec<FmpStock>, Error> {
    let url = format!(
        "https://financialmodelingprep.com/api/v3/available-traded/list?apikey={}",
        &api_key,
    );
    let body = reqwest::get(&url).await?.text().await?;
    let r: Vec<FmpStock> = serde_json::from_str(&body).expect("should work get_all_stocks");
    Ok(r)
}

pub async fn get_all_euronext_stocks(api_key: String) -> Result<Vec<FmpEuroNextStock>, Error> {
    let url = format!(
        "https://financialmodelingprep.com/api/v3/symbol/available-euronext?apikey={}",
        &api_key,
    );
    let body = reqwest::get(&url).await?.text().await?;

    let r: Vec<FmpEuroNextStock> =
        serde_json::from_str(&body).expect("should work get_all_euronext_stocks");
    Ok(r)
}

pub async fn get_all_indices(api_key: String) -> Result<Vec<FmpEuroNextStock>, Error> {
    let url = format!(
        "https://financialmodelingprep.com/api/v3/symbol/available-indexes?apikey={}",
        &api_key,
    );
    let body = reqwest::get(&url).await?.text().await?;

    let r: Vec<FmpEuroNextStock> =
        serde_json::from_str(&body).expect("should work get_all_indices");
    Ok(r)
}

pub async fn get_all_financial_statement_list(api_key: String) -> Result<Vec<String>, Error> {
    let url = format!(
        "https://financialmodelingprep.com/api/v3/financial-statement-symbol-lists?apikey={}",
        &api_key,
    );
    let body = reqwest::get(&url).await?.text().await?;

    let r: Vec<String> =
        serde_json::from_str(&body).expect("should work get_all_financial_statement_list");
    Ok(r)
}

pub async fn get_company_details(api_key: String, symbol: &str) -> Result<Vec<FmpCompany>, Error> {
    let url = format!(
        "https://financialmodelingprep.com/api/v3/profile/{}?apikey={}",
        symbol, &api_key,
    );
    let body = reqwest::get(&url).await?.text().await?;

    let r: Vec<FmpCompany> = serde_json::from_str(&body).expect("should work get_company_details");
    Ok(r)
}

pub async fn get_company_notes(
    api_key: String,
    symbol: &str,
) -> Result<Vec<FmpCompanyNote>, Error> {
    let url = format!(
        "https://financialmodelingprep.com/api/v4/company-notes?symbol={}&apikey={}",
        symbol, &api_key,
    );
    let body = reqwest::get(&url).await?.text().await?;

    println!("body {body:?}");
    let r: Vec<FmpCompanyNote> =
        serde_json::from_str(&body).expect("should work get_company_details");
    Ok(r)
}

pub async fn get_company_grade(
    api_key: String,
    symbol: &str,
) -> Result<Vec<FmpCompanyGrade>, Error> {
    let url = format!(
        "https://financialmodelingprep.com/api/v3/grade/{}?apikey={}",
        symbol, &api_key,
    );
    let body = reqwest::get(&url).await?.text().await?;

    println!("body {body:?}");
    let r: Vec<FmpCompanyGrade> =
        serde_json::from_str(&body).expect("should work get_company_details");
    Ok(r)
}

pub async fn get_all_countries(api_key: String) -> Result<Vec<String>, Error> {
    let url = format!(
        "https://financialmodelingprep.com/api/v3/get-all-countries?apikey={}",
        &api_key,
    );
    let body = reqwest::get(&url).await?.text().await?;

    println!("body {body:?}");
    let r: Vec<String> = serde_json::from_str(&body).expect("should work get_all_countries");
    Ok(r)
}

pub async fn get_analyst_estimates(
    api_key: String,
    symbol: &str,
) -> Result<Vec<FmpAnalystEstimate>, Error> {
    let period = "quarter"; // annual
    let url = format!(
        "https://financialmodelingprep.com/api/v3/analyst-estimates/{}?apikey={}&period={}",
        symbol, &api_key, &period
    );
    let body = reqwest::get(&url).await?.text().await?;

    println!("body {body:?}");
    let r: Vec<FmpAnalystEstimate> =
        serde_json::from_str(&body).expect("should work get_all_countries");
    Ok(r)
}

pub async fn get_stock_peers(api_key: String, symbol: &str) -> Result<Vec<FmpStockPeer>, Error> {
    let url = format!(
        "https://financialmodelingprep.com/api/v4/stock-peers/symbol={}?apikey={}",
        symbol, &api_key,
    );
    let body = reqwest::get(&url).await?.text().await?;

    println!("body {body:?}");
    let r: Vec<FmpStockPeer> = serde_json::from_str(&body).expect("should work get_all_countries");
    Ok(r)
}

pub async fn get_share_float(api_key: String, symbol: &str) -> Result<Vec<FmpShareFloat>, Error> {
    let url = format!(
        "https://financialmodelingprep.com/api/v4/shares_float/symbol={}?apikey={}",
        symbol, &api_key,
    );
    let body = reqwest::get(&url).await?.text().await?;
    let r: Vec<FmpShareFloat> = serde_json::from_str(&body).expect("should work get_all_countries");
    Ok(r)
}

pub async fn get_share_float_all(api_key: String) -> Result<Vec<FmpShareFloat>, Error> {
    let url = format!(
        "https://financialmodelingprep.com/api/v4/shares_float/all?apikey={}",
        &api_key,
    );
    let body = reqwest::get(&url).await?.text().await?;

    let r: Vec<FmpShareFloat> =
        serde_json::from_str(&body).expect("should work get_share_float_all");
    Ok(r)
}

pub async fn get_sectors(api_key: String) -> Result<Vec<String>, Error> {
    let url = format!(
        "https://financialmodelingprep.com/api/v3/sectors-list?apikey={}",
        &api_key,
    );
    let body = reqwest::get(&url).await?.text().await?;

    let r: Vec<String> = serde_json::from_str(&body).expect("should work get_sectors");
    Ok(r)
}

pub async fn get_industries(api_key: String) -> Result<Vec<String>, Error> {
    let url = format!(
        "https://financialmodelingprep.com/api/v3/industries-list?apikey={}",
        &api_key,
    );
    let body = reqwest::get(&url).await?.text().await?;

    let r: Vec<String> = serde_json::from_str(&body).expect("should work get_industries");
    Ok(r)
}

pub async fn get_exchanges(api_key: String) -> Result<Vec<String>, Error> {
    let url = format!(
        "https://financialmodelingprep.com/api/v3/exchanges-list?apikey={}",
        &api_key,
    );
    let body = reqwest::get(&url).await?.text().await?;

    let r: Vec<String> = serde_json::from_str(&body).expect("should work get_industries");
    Ok(r)
}

pub async fn get_full_quote_real_time(
    api_key: String,
    symbol: &str,
) -> Result<Vec<FmpStockFullQuote>, Error> {
    let url = format!(
        "https://financialmodelingprep.com/api/v3/quote/{}?apikey={}",
        symbol, &api_key,
    );
    let body = reqwest::get(&url).await?.text().await?;
    let r: Vec<FmpStockFullQuote> =
        serde_json::from_str(&body).expect("should work get_all_countries");
    Ok(r)
}

pub async fn get_quote_order(
    api_key: String,
    symbol: &str,
) -> Result<Vec<FmpStockFullQuote>, Error> {
    let url = format!(
        "https://financialmodelingprep.com/api/v3/quote-order/{}?apikey={}",
        symbol, &api_key,
    );
    let body = reqwest::get(&url).await?.text().await?;
    let r: Vec<FmpStockFullQuote> =
        serde_json::from_str(&body).expect("should work get_all_countries");
    Ok(r)
}

pub async fn get_exchange_quote_order(
    api_key: String,
    exchange: &str,
) -> Result<Vec<FmpStockFullQuote>, Error> {
    let url = format!(
        "https://financialmodelingprep.com/api/v3/quotes/{}?apikey={}",
        exchange, &api_key,
    );
    let body = reqwest::get(&url).await?.text().await?;
    println!("body {body:?}");

    let r: Vec<FmpStockFullQuote> =
        serde_json::from_str(&body).expect("should work get_all_countries");
    Ok(r)
}

pub async fn get_stock_price_change(
    api_key: String,
    symbol: &str,
) -> Result<Vec<FmpStockPriceChange>, Error> {
    let url = format!(
        "https://financialmodelingprep.com/api/v3/stock-price-change/{}?apikey={}",
        symbol, &api_key,
    );
    let body = reqwest::get(&url).await?.text().await?;
    println!("body {body:?}");

    let r: Vec<FmpStockPriceChange> =
        serde_json::from_str(&body).expect("should work get_all_countries");
    Ok(r)
}

pub async fn get_historical_dividends(
    api_key: String,
    symbol: &str,
) -> Result<FmpHistoricalDividendsResult, Error> {
    let url = format!(
        "https://financialmodelingprep.com/api/v3/historical-price-full/stock_dividend/{}?apikey={}",
        symbol, &api_key,
    );
    let body = reqwest::get(&url).await?.text().await?;
    println!("body {body:?}");

    let r: FmpHistoricalDividendsResult =
        serde_json::from_str(&body).expect("should work get_all_countries");
    Ok(r)
}

pub async fn get_market_index(api_key: String) -> Result<Vec<FmpIndexQuote>, Error> {
    let url = format!(
        "https://financialmodelingprep.com/api/v3/quotes/index?apikey={}",
        &api_key,
    );
    let body = reqwest::get(&url).await?.text().await?;
    println!("body {body:?}");

    let r: Vec<FmpIndexQuote> = serde_json::from_str(&body).expect("should work get_market_index");
    Ok(r)
}

pub async fn get_economic_indicator(
    api_key: String,
    indicator: &str,
) -> Result<Vec<FmpEconomicIndicator>, Error> {
    let url = format!(
        "https://financialmodelingprep.com/api/v4/economic?name={}&apikey={}",
        indicator, &api_key,
    );
    let body = reqwest::get(&url).await?.text().await?;
    println!("body {body:?}");

    let r: Vec<FmpEconomicIndicator> =
        serde_json::from_str(&body).expect("should work get_economic_indicator");
    Ok(r)
}

pub async fn get_economic_indicator(
    api_key: String,
    indicator: &str,
) -> Result<Vec<FmpEconomicIndicator>, Error> {
    let url = format!(
        "https://financialmodelingprep.com/api/v4/economic?name={}&apikey={}",
        indicator, &api_key,
    );
    let body = reqwest::get(&url).await?.text().await?;
    println!("body {body:?}");

    let r: Vec<FmpEconomicIndicator> =
        serde_json::from_str(&body).expect("should work get_economic_indicator");
    Ok(r)
}

pub async fn get_treasury_rates(
    api_key: String,
    from: &str,
    to: &str,
) -> Result<Vec<FmpTreasuryRates>, Error> {
    let url = format!(
        "https://financialmodelingprep.com/api/v4/treasury?from={}&to={}&apikey={}",
        from, to, &api_key,
    );
    let body = reqwest::get(&url).await?.text().await?;
    println!("body {body:?}");

    let r: Vec<FmpTreasuryRates> =
        serde_json::from_str(&body).expect("should work get_economic_indicator");
    Ok(r)
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    // let series_id = "USACPIALLMINMEI".to_string();
    // let r = get_series(&series_id).await?;
    // println!("{series_id} = {r:?}");

    // let series_id = "UNRATE".to_string();
    // let r = get_series(&series_id).await?;
    // println!("{series_id} = {r:?}");

    // let series_id = "FPCPITOTLZGUSA".to_string();
    // let r = get_series(&series_id).await?;
    // println!("{series_id} = {r:?}");

    // let series_id = "FPCPITOTLZGUSA".to_string();
    // let r = get_observation(&series_id).await?;
    // println!("observation {series_id} = {r:?}");

    // let r = get_all_stocks().await?;
    // println!("FMP all stocks = {r:?}");

    // let r = get_all_euronext_stocks().await?;
    // println!("FMP EuroNext all stocks = {r:?}");

    // let r = get_all_indices().await?;
    // println!("FMP all indices = {r:?}");

    // let r = get_all_financial_statement_list().await?;
    // println!("FMP all financial statements = {r:?}");

    // let symbol = "AAPL,MSFT";
    // let r = get_company_details(symbol).await?;
    // println!("FMP all financial statements = {r:?}");

    // let symbol = "AAPL,MSFT";
    // let r = get_company_notes(symbol).await?;
    // println!("FMP company notes = {r:?}");

    // let symbol = "AAPL,MSFT";
    // let r = get_company_grade(symbol).await?;
    // println!("FMP company grades = {r:?}");

    // let r = get_all_countries().await?;
    // println!("FMP all countries = {r:?}");

    // let symbol = "AAPL,MSFT";
    // let r = get_analyst_estimates(symbol).await?;
    // println!("FMP analst estimates = {r:?}");

    // let symbol = "AAPL,MSFT";
    // let r = get_stock_peers(symbol).await?;
    // println!("FMP analst estimates = {r:?}");

    // let symbol = "AAPL,MSFT";
    // let r = get_share_float(symbol).await?;
    // println!("FMP analst estimates = {r:?}");

    // let symbol = "AAPL,MSFT";
    // let r = get_share_float_all().await?;
    // println!("FMP share float all = {r:?}");

    // let symbol = "AAPL,MSFT";
    // let r = get_sectors().await?;
    // println!("FMP sectors = {r:?}");

    // let symbol = "AAPL,MSFT";
    // let r = get_industries().await?;
    // println!("FMP industries = {r:?}");

    // let symbol = "AAPL,MSFT";
    // let r = get_exchanges().await?;
    // println!("FMP exchanges = {r:?}");

    // let symbol = "AAPL,MSFT";
    // let r = get_full_quote_real_time(symbol).await?;
    // println!("FMP full quote = {r:?}");

    // let symbol = "AAPL,MSFT";
    // let r = get_quote_order(symbol).await?;
    // println!("FMP full quote = {r:?}");

    // let symbol = "NYSE";
    // let r = get_exchange_quote_order(symbol).await?;
    // println!("FMP exchange quote quote = {r:?}");

    // let symbol =  "AAPL,MSFT";
    // let r = get_stock_price_change(symbol).await?;
    // println!("FMP exchange quote quote = {r:?}");

    // let symbol =  "AAPL,MSFT";
    // let r = get_historical_dividends(symbol).await?;
    // println!("FMP historical dividends = {r:?}");

    // let symbol =  "AAPL,MSFT";
    // let r = get_market_index().await?;
    // println!("FMP market index = {r:?}");

    // let indicator = "inflation";
    // let r = get_economic_indicator(indicator).await?;
    // println!("FMP economic indicator: inflation = {r:?}");

    // let indicator = "inflationRate";
    // let r = get_economic_indicator(indicator).await?;
    // println!("FMP economic indicator: inflationRate = {r:?}");

    // let indicator = "CPI";
    // let r = get_economic_indicator(indicator).await?;
    // println!("FMP economic indicator: CPI = {r:?}");

    let from = "2023-08-10";
    let to = "2024-08-10";
    let r = get_treasury_rates(from, to).await?;
    println!("FMP treasyury rates = {r:?}");

    Ok(())
}


 