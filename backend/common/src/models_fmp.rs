use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct FmpStock {
    symbol: Option<String>,
    name: Option<String>,
    price: Option<f64>,
    exchange: Option<String>,
    #[serde(alias = "exchangeShortName")]
    exchange_short_name: Option<String>,
    #[serde(alias = "type")]
    typ: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FmpEuroNextStock {
    symbol: Option<String>,
    name: Option<String>,
    currency: Option<String>,
    #[serde(alias = "stockExchange")]
    stock_exchange: Option<String>,
    #[serde(alias = "exchangeShortName")]
    exchange_short_name: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FmpCompany {
    symbol: Option<String>,
    price: Option<f64>,
    beta: Option<f64>,
    #[serde(alias = "volAvg")]
    vol_avg: Option<i64>,
    #[serde(alias = "mktCap")]
    mkt_cap: Option<f64>,
    #[serde(alias = "lastDiv")]
    last_div: Option<f64>,
    range: Option<String>,
    changes: Option<f64>,
    #[serde(alias = "companyName")]
    company_name: Option<String>,
    currency: Option<String>,
    cik: Option<String>,
    isin: Option<String>,
    cusip: Option<String>,
    exchange: Option<String>,
    #[serde(alias = "exchangeShortName")]
    exchange_short_name: Option<String>,
    industry: Option<String>,
    website: Option<String>,
    description: Option<String>,
    ceo: Option<String>,
    sector: Option<String>,
    country: Option<String>,
    #[serde(alias = "fullTimeEmployees")]
    full_time_employees: Option<String>,
    phone: Option<String>,
    address: Option<String>,
    city: Option<String>,
    state: Option<String>,
    zip: Option<String>,
    #[serde(alias = "dcfDiff")]
    dcf_diff: Option<f64>,
    dcf: Option<f64>,
    image: Option<String>,
    #[serde(alias = "ipoDate")]
    ipo_date: Option<String>,
    #[serde(alias = "defaultImage")]
    default_image: Option<bool>,
    #[serde(alias = "isEtf")]
    is_etf: Option<bool>,
    #[serde(alias = "isActivelyTrading")]
    is_actively_trading: Option<bool>,
    #[serde(alias = "isAdr")]
    is_adr: Option<bool>,
    #[serde(alias = "isFund")]
    is_fund: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FmpCompanyNote {
    cik: Option<String>,
    symbol: Option<String>,
    title: Option<String>,
    exchange: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FmpCompanyGrade {
    symbol: Option<String>,
    date: Option<String>,
    #[serde(alias = "gradingCompany")]
    grading_company: Option<String>,
    #[serde(alias = "previousGrade")]
    previous_grade: Option<String>,
    #[serde(alias = "newGrade")]
    new_grade: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FmpAnalystEstimate {
    symbol: Option<String>,
    date: Option<String>,
    #[serde(alias = "estimatedRevenueLow")]
    estimated_revenue_low: Option<i64>,
    #[serde(alias = "estimatedRevenueHigh")]
    estimated_revenue_high: Option<i64>,
    #[serde(alias = "estimatedRevenueAvg")]
    estimated_revenue_avg: Option<i64>,
    #[serde(alias = "estimatedEbitdaLow")]
    estimated_ebitda_low: Option<i64>,
    #[serde(alias = "estimatedEbitdaHigh")]
    estimated_ebitda_high: Option<i64>,
    #[serde(alias = "estimatedEbitdaAvg")]
    estimated_ebitda_avg: Option<i64>,
    #[serde(alias = "estimatedEbitLow")]
    estimated_ebit_low: Option<i64>,
    #[serde(alias = "estimatedEbitHigh")]
    estimated_ebit_high: Option<i64>,
    #[serde(alias = "estimatedEbitAvg")]
    estimated_ebit_avg: Option<i64>,
    #[serde(alias = "estimatedNetIncomeLow")]
    estimated_net_income_low: Option<i64>,
    #[serde(alias = "estimatedNetIncomeHigh")]
    estimated_net_income_high: Option<i64>,
    #[serde(alias = "estimatedNetIncomeAvg")]
    estimated_net_income_avg: Option<i64>,
    #[serde(alias = "estimatedSgaExpenseLow")]
    estimated_sga_expense_low: Option<i64>,
    #[serde(alias = "estimatedSgaExpenseHigh")]
    estimated_sga_expense_high: Option<i64>,
    #[serde(alias = "estimatedSgaExpenseAvg")]
    estimated_sga_expense_avg: Option<i64>,
    #[serde(alias = "estimatedEpsAvg")]
    estimated_eps_avg: Option<f64>,
    #[serde(alias = "estimatedEpsHigh")]
    estimated_eps_high: Option<f64>,
    #[serde(alias = "estimatedEpsLow")]
    estimated_eps_low: Option<f64>,
    #[serde(alias = "numberAnalystEstimatedRevenue")]
    number_analyst_estimated_revenue: Option<i64>,
    #[serde(alias = "numberAnalystsEstimatedEps")]
    number_analysts_estimated_eps: Option<i64>,
}

// TODO: https://site.financialmodelingprep.com/developer/docs/company-outlook-api
// a lot of structs :-(

#[derive(Serialize, Deserialize, Debug)]
pub struct FmpStockPeer {
    symbol: Option<String>,
    pp: Option<String>,
    #[serde(alias = "peersList")]
    peers_list: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FmpShareFloat {
    symbol: Option<String>,
    #[serde(alias = "freeFloat")]
    free_float: Option<f64>,
    #[serde(alias = "floatShares")]
    float_shares: Option<f64>,
    #[serde(alias = "outstandingShares")]
    outstanding_shares: Option<f64>,
    source: Option<String>,
    date: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FmpStockFullQuote {
    symbol: Option<String>,
    name: Option<String>,
    price: Option<f64>,
    #[serde(alias = "changesPercentage")]
    changes_percentage: Option<f64>,
    change: Option<f64>,
    #[serde(alias = "dayLow")]
    day_low: Option<f64>,
    #[serde(alias = "dayHigh")]
    day_high: Option<f64>,
    #[serde(alias = "yearHigh")]
    year_high: Option<f64>,
    #[serde(alias = "yearLow")]
    year_low: Option<f64>,
    #[serde(alias = "marketCap")]
    market_cap: Option<i64>,
    #[serde(alias = "priceAvg50")]
    price_avg50: Option<f64>,
    #[serde(alias = "priceAvg200")]
    price_avg200: Option<f64>,
    exchange: Option<String>,
    volume: Option<i64>,
    #[serde(alias = "avgVolume")]
    avg_volume: Option<i64>,
    open: Option<f64>,
    #[serde(alias = "previousClose")]
    previous_close: Option<f64>,
    eps: Option<f64>,
    pe: Option<f64>,
    #[serde(alias = "earningsAnnouncement")]
    earnings_announcement: Option<String>,
    #[serde(alias = "sharesOutstanding")]
    shares_outstanding: Option<i64>,
    timestamp: Option<i64>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FmpStockPriceChange {
    symbol: Option<String>,
    #[serde(alias = "1D")]
    days_1: Option<f64>,
    #[serde(alias = "5D")]
    days_5: Option<f64>,
    #[serde(alias = "1D")]
    months_1: Option<f64>,
    #[serde(alias = "3M")]
    months_3: Option<f64>,
    #[serde(alias = "6M")]
    months_6: Option<f64>,
    ytd: Option<f64>,
    #[serde(alias = "1Y")]
    years_1: Option<f64>,
    #[serde(alias = "3Y")]
    years_3: Option<f64>,
    #[serde(alias = "5Y")]
    years_5: Option<f64>,
    #[serde(alias = "10Y")]
    years_10: Option<f64>,
    max: Option<f64>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FmpHistorical {
    date: Option<String>,
    label: Option<String>,
    #[serde(alias = "adjDividend")]
    adj_dividend: Option<f64>,
    dividend: Option<f64>,
    #[serde(alias = "recordDate")]
    record_date: Option<String>,
    #[serde(alias = "paymentDate")]
    payment_date: Option<String>,
    #[serde(alias = "declarationDate")]
    declaration_date: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FmpHistoricalDividends {
    symbol: Option<String>,
    historical: Vec<FmpHistorical>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FmpHistoricalDividendsResult {
    symbol: Option<String>,
    #[serde(alias = "historicalStockList")]
    historical_stock_list: Vec<FmpHistoricalDividends>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FmpIndexQuote {
    symbol: Option<String>,
    price: Option<f64>,
    #[serde(alias = "extendedPrice")]
    extended_price: Option<f64>,
    change: Option<f64>,
    #[serde(alias = "dayHigh")]
    day_high: Option<f64>,
    #[serde(alias = "dayLow")]
    day_low: Option<f64>,
    #[serde(alias = "previousClose")]
    previous_close: Option<f64>,
    volume: Option<f64>,
    open: Option<f64>,
    close: Option<f64>,
    #[serde(alias = "lastTradeTime")]
    last_trade_time: Option<String>,
    #[serde(alias = "lastExtendedTradeTime")]
    last_extended_trade_time: Option<String>,
    #[serde(alias = "updatedAt")]
    updated_at: Option<String>,
    #[serde(alias = "createdAt")]
    created_at: Option<String>,
    #[serde(alias = "type")]
    typ: Option<String>,
    name: Option<String>,
    range: Option<f64>,
    #[serde(alias = "yearHigh")]
    year_high: Option<f64>,
    #[serde(alias = "yearLow")]
    year_low: Option<f64>,
    #[serde(alias = "priceAvg50")]
    price_avg50: Option<f64>,
    #[serde(alias = "priceAvg200")]
    price_avg200: Option<f64>,
    #[serde(alias = "changesPercentage")]
    changes_percentage: Option<f64>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FmpEconomicIndicator {
    date: Option<String>,
    value: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FmpTreasuryRates {
    date: Option<String>,
    month1: Option<String>,
    month2: Option<String>,
    month3: Option<String>,
    month6: Option<String>,
    year1: Option<String>,
    year2: Option<String>,
    year3: Option<String>,
    year5: Option<String>,
    year7: Option<String>,
    year10: Option<String>,
    year20: Option<String>,
    year30: Option<String>,
}
