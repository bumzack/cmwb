use chrono::NaiveDateTime;
use diesel::prelude::*;

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::fmp_stock)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct DbFmpStock {
    pub symbol: String,
    pub name: String,
    pub price: f64,
    pub exchange: Option<String>,
    pub exchange_short_name: Option<String>,
    pub typ: Option<String>,
    pub created: NaiveDateTime,
    pub updated: NaiveDateTime,
}

#[derive(serde::Serialize, serde::Deserialize, Queryable, Selectable)]
#[diesel(table_name = crate::schema::usr)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct DbUser {
    pub id: i32,
    pub username: String,
    pub password: String,
    pub created: NaiveDateTime,
    pub updated: NaiveDateTime,
}

#[derive(serde::Deserialize, serde::Serialize, Insertable)]
#[diesel(table_name = crate::schema::usr)]
pub struct DbNewUser {
    pub username: String,
    pub password: String,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::fmp_stock)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct DbNewFmpStock {
    pub symbol: String,
    pub name: String,
    pub price: f64,
    pub exchange: Option<String>,
    pub exchange_short_name: Option<String>,
    pub typ: Option<String>,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::fmp_euro_next_stock)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct DbFmpEuroNextStock {
    pub symbol: String,
    pub name: Option<String>,
    pub currency: String,
    pub stock_exchange: Option<String>,
    pub exchange_short_name: Option<String>,
    pub created: NaiveDateTime,
    pub updated: NaiveDateTime,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::fmp_company)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct DbFmpCompany {
    pub symbol: String,
    pub price: Option<f64>,
    pub beta: Option<f64>,
    pub vol_avg: Option<i64>,
    pub mkt_cap: Option<f64>,
    pub last_div: Option<f64>,
    pub range: Option<String>,
    pub changes: Option<f64>,
    pub company_name: Option<String>,
    pub currency: Option<String>,
    pub cik: Option<String>,
    pub isin: Option<String>,
    pub cusip: Option<String>,
    pub exchange: Option<String>,
    pub exchange_short_name: Option<String>,
    pub industry: Option<String>,
    pub website: Option<String>,
    pub description: Option<String>,
    pub ceo: Option<String>,
    pub sector: Option<String>,
    pub country: Option<String>,
    pub full_time_employees: Option<String>,
    pub phone: Option<String>,
    pub address: Option<String>,
    pub city: Option<String>,
    pub state: Option<String>,
    pub zip: Option<String>,
    pub dcf_diff: Option<f64>,
    pub dcf: Option<f64>,
    pub image: Option<String>,
    pub ipo_date: Option<String>,
    pub default_image: Option<bool>,
    pub is_etf: Option<bool>,
    pub is_actively_trading: Option<bool>,
    pub is_adr: Option<bool>,
    pub is_fund: Option<bool>,
    pub created: NaiveDateTime,
    pub updated: NaiveDateTime,
}

// #[derive(Queryable, Selectable)]
// #[diesel(table_name = crate::schema::fmp_company_note)]
// #[diesel(check_for_backend(diesel::pg::Pg))]
// pub struct FmpCompanyNote {
//     cik: Option<String>,
//     symbol: Option<String>,
//     title: Option<String>,
//     exchange: Option<String>,
// }

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::fmp_stock_peer)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct DbFmpStockPeer {
    pub symbol: String,
    pub peers_list: Option<String>,
    pub created: NaiveDateTime,
    pub updated: NaiveDateTime,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::fmp_stock_full_quote)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct DbFmpStockFullQuote {
    pub symbol: String,
    pub name: Option<String>,
    pub price: Option<f64>,
    pub changes_percentage: Option<f64>,
    pub change: Option<f64>,
    pub day_low: Option<f64>,
    pub day_high: Option<f64>,
    pub year_high: Option<f64>,
    pub year_low: Option<f64>,
    pub market_cap: Option<f64>,
    pub price_avg50: Option<f64>,
    pub price_avg200: Option<f64>,
    pub exchange: Option<String>,
    pub volume: Option<i64>,
    pub avg_volume: Option<i64>,
    pub open: Option<f64>,
    pub previous_close: Option<f64>,
    pub eps: Option<f64>,
    pub pe: Option<f64>,
    pub earnings_announcement: Option<String>,
    pub shares_outstanding: Option<i64>,
    pub timestamp: NaiveDateTime,
    pub created: NaiveDateTime,
    pub updated: NaiveDateTime,
}

// #[derive(Queryable, Selectable)]
// #[diesel(table_name = crate::schema::fmp_stock_pr)]
// #[diesel(check_for_backend(diesel::pg::Pg))]
// pub struct FmpStockPriceChange {
//     symbol: Option<String>,
//     days_1: Option<f64>,
//     days_5: Option<f64>,
//     months_1: Option<f64>,
//     months_3: Option<f64>,
//     months_6: Option<f64>,
//     ytd: Option<f64>,
//     years_1: Option<f64>,
//     years_3: Option<f64>,
//     years_5: Option<f64>,
//     years_10: Option<f64>,
//     max: Option<f64>,
//     created: NaiveDateTime,
//     updated: NaiveDateTime,
// }

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::fmp_historical)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct DbFmpHistorical {
    pub date: Option<String>,
    pub symbol: String,
    pub label: Option<String>,
    pub adj_dividend: Option<f64>,
    pub dividend: Option<f64>,
    pub record_date: Option<String>,
    pub payment_date: Option<String>,
    pub declaration_date: Option<String>,
    pub created: NaiveDateTime,
    pub updated: NaiveDateTime,
}
