// @generated automatically by Diesel CLI.

diesel::table! {
    fmp_company (id) {
        id -> Int4,
        #[max_length = 50]
        symbol -> Varchar,
        price -> Nullable<Float8>,
        beta -> Nullable<Float8>,
        vol_avg -> Nullable<Int8>,
        mkt_cap -> Nullable<Float8>,
        last_div -> Nullable<Float8>,
        #[max_length = 500]
        range -> Nullable<Varchar>,
        changes -> Nullable<Float8>,
        #[max_length = 500]
        company_name -> Nullable<Varchar>,
        #[max_length = 20]
        currency -> Nullable<Varchar>,
        #[max_length = 20]
        cik -> Nullable<Varchar>,
        #[max_length = 20]
        isin -> Nullable<Varchar>,
        #[max_length = 20]
        cusip -> Nullable<Varchar>,
        #[max_length = 500]
        exchange -> Nullable<Varchar>,
        #[max_length = 50]
        exchange_short_name -> Nullable<Varchar>,
        #[max_length = 500]
        industry -> Nullable<Varchar>,
        #[max_length = 500]
        website -> Nullable<Varchar>,
        description -> Nullable<Text>,
        #[max_length = 500]
        ceo -> Nullable<Varchar>,
        #[max_length = 500]
        sector -> Nullable<Varchar>,
        #[max_length = 500]
        country -> Nullable<Varchar>,
        #[max_length = 500]
        full_time_employees -> Nullable<Varchar>,
        #[max_length = 500]
        phone -> Nullable<Varchar>,
        #[max_length = 500]
        address -> Nullable<Varchar>,
        #[max_length = 500]
        city -> Nullable<Varchar>,
        #[max_length = 500]
        state -> Nullable<Varchar>,
        #[max_length = 500]
        zip -> Nullable<Varchar>,
        dcf_diff -> Nullable<Float8>,
        dcf -> Nullable<Float8>,
        #[max_length = 500]
        image -> Nullable<Varchar>,
        #[max_length = 500]
        ipo_date -> Nullable<Varchar>,
        default_image -> Nullable<Bool>,
        is_etf -> Nullable<Bool>,
        is_actively_trading -> Nullable<Bool>,
        is_adr -> Nullable<Bool>,
        is_fund -> Nullable<Bool>,
        created -> Timestamptz,
        updated -> Timestamptz,
    }
}

diesel::table! {
    fmp_economic_indicator (id) {
        id -> Int4,
        #[max_length = 500]
        date -> Varchar,
        #[max_length = 500]
        value -> Varchar,
        created -> Timestamptz,
        updated -> Timestamptz,
    }
}

diesel::table! {
    fmp_euro_next_stock (id) {
        id -> Int4,
        #[max_length = 50]
        symbol -> Varchar,
        #[max_length = 50]
        name -> Nullable<Varchar>,
        #[max_length = 50]
        currency -> Varchar,
        price -> Float8,
        #[max_length = 500]
        stock_exchange -> Nullable<Varchar>,
        #[max_length = 50]
        exchange_short_name -> Nullable<Varchar>,
        created -> Timestamptz,
        updated -> Timestamptz,
    }
}

diesel::table! {
    fmp_historical (id) {
        id -> Int4,
        #[max_length = 500]
        symbol -> Varchar,
        #[max_length = 500]
        date -> Nullable<Varchar>,
        #[max_length = 500]
        label -> Nullable<Varchar>,
        adj_dividend -> Nullable<Float8>,
        dividend -> Nullable<Float8>,
        #[max_length = 500]
        record_date -> Nullable<Varchar>,
        #[max_length = 500]
        payment_date -> Nullable<Varchar>,
        #[max_length = 500]
        declaration_date -> Nullable<Varchar>,
        created -> Timestamptz,
        updated -> Timestamptz,
    }
}

diesel::table! {
    fmp_historical_dividends (id) {
        id -> Int4,
        historical_id -> Nullable<Int8>,
        created -> Timestamptz,
        updated -> Timestamptz,
    }
}

diesel::table! {
    fmp_stock (id) {
        id -> Int4,
        #[max_length = 50]
        symbol -> Varchar,
        #[max_length = 50]
        name -> Varchar,
        price -> Float8,
        #[max_length = 500]
        exchange -> Nullable<Varchar>,
        #[max_length = 50]
        exchange_short_name -> Nullable<Varchar>,
        #[max_length = 50]
        typ -> Nullable<Varchar>,
        created -> Timestamptz,
        updated -> Timestamptz,
    }
}

diesel::table! {
    fmp_stock_full_quote (id) {
        id -> Int4,
        #[max_length = 500]
        symbol -> Varchar,
        #[max_length = 500]
        name -> Nullable<Varchar>,
        price -> Nullable<Float8>,
        changes_percentage -> Nullable<Float8>,
        change -> Nullable<Float8>,
        day_low -> Nullable<Float8>,
        day_high -> Nullable<Float8>,
        year_high -> Nullable<Float8>,
        year_low -> Nullable<Float8>,
        market_cap -> Nullable<Float8>,
        price_avg50 -> Nullable<Float8>,
        price_avg200 -> Nullable<Float8>,
        #[max_length = 500]
        exchange -> Nullable<Varchar>,
        volume -> Nullable<Int8>,
        avg_volume -> Nullable<Int8>,
        open -> Nullable<Float8>,
        previous_close -> Nullable<Float8>,
        eps -> Nullable<Float8>,
        pe -> Nullable<Float8>,
        #[max_length = 500]
        earnings_announcement -> Nullable<Varchar>,
        shares_outstanding -> Nullable<Int8>,
        timestamp -> Timestamp,
        created -> Timestamptz,
        updated -> Timestamptz,
    }
}

diesel::table! {
    fmp_stock_peer (id) {
        id -> Int4,
        #[max_length = 500]
        symbol -> Varchar,
        peers_list -> Nullable<Text>,
        created -> Timestamptz,
        updated -> Timestamptz,
    }
}

diesel::table! {
    usr (id) {
        id -> Int4,
        #[max_length = 500]
        username -> Varchar,
        #[max_length = 5000]
        password -> Varchar,
        created -> Timestamptz,
        updated -> Timestamptz,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    fmp_company,
    fmp_economic_indicator,
    fmp_euro_next_stock,
    fmp_historical,
    fmp_historical_dividends,
    fmp_stock,
    fmp_stock_full_quote,
    fmp_stock_peer,
    usr,
);
