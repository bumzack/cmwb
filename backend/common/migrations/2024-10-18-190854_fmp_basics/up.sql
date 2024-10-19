-- Your SQL goes here


CREATE TABLE usr (
  id SERIAL PRIMARY KEY,
  username VARCHAR (500)   UNIQUE  NOT NULL ,
  "password" VARCHAR (5000) NOT NULL,
  created timestamp with time zone default (now() at time zone 'utc')  NOT NULL,
  updated timestamp with time zone default (now() at time zone 'utc') NOT NULL
);

CREATE TABLE fmp_stock (
  id SERIAL PRIMARY KEY,
  symbol VARCHAR (50)   UNIQUE  NOT NULL ,
  "name" VARCHAR (50) NOT NULL,
  price DOUBLE PRECISION NOT NULL,
  exchange VARCHAR (500),
  exchange_short_name VARCHAR (50),
  typ VARCHAR (50),
  created timestamp with time zone default (now() at time zone 'utc')  NOT NULL,
  updated timestamp with time zone default (now() at time zone 'utc') NOT NULL
);

CREATE TABLE fmp_euro_next_stock (
  id SERIAL PRIMARY KEY,
  symbol VARCHAR (50) UNIQUE NOT NULL ,
  name VARCHAR (50),
  currency VARCHAR (50) NOT NULL,
  price DOUBLE PRECISION NOT NULL,
  stock_exchange VARCHAR (500),
  exchange_short_name VARCHAR (50),
  created timestamp with time zone default (now() at time zone 'utc') NOT NULL,
  updated timestamp with time zone default (now() at time zone 'utc') NOT NULL
);

CREATE TABLE fmp_company (
  id SERIAL PRIMARY KEY,
  symbol VARCHAR (50)   UNIQUE  NOT NULL ,
  price DOUBLE PRECISION,
  beta DOUBLE PRECISION,
  vol_avg bigint,
  mkt_cap DOUBLE PRECISION,
  last_div DOUBLE PRECISION,
  range VARCHAR (500),
  changes DOUBLE PRECISION,
  company_name VARCHAR (500),
  currency VARCHAR(20),
  cik VARCHAR(20),
  isin VARCHAR(20),
  cusip VARCHAR(20),
  exchange VARCHAR(500),
  exchange_short_name VARCHAR (50),
  industry VARCHAR(500),
  website VARCHAR(500),
  description TEXT,
  ceo VARCHAR(500),
  sector VARCHAR(500),
  country VARCHAR(500),
  full_time_employees VARCHAR(500),
  phone VARCHAR(500),
  address VARCHAR(500),
  city VARCHAR(500),
  state VARCHAR(500),
  zip VARCHAR(500),
  dcf_diff DOUBLE PRECISION,
  dcf DOUBLE PRECISION,
  image VARCHAR(500),
  ipo_date VARCHAR(500),
  default_image boolean,
  is_etf boolean,
  is_actively_trading boolean,
  is_adr boolean,
  is_fund boolean,
  created timestamp with time zone default (now() at time zone 'utc') NOT NULL,
  updated timestamp with time zone default (now() at time zone 'utc') NOT NULL
);

CREATE TABLE fmp_stock_peer (
  id SERIAL PRIMARY KEY,
  symbol VARCHAR (500)   UNIQUE  NOT NULL ,
  peers_list TEXT,
  created timestamp with time zone default (now() at time zone 'utc') NOT NULL,
  updated timestamp with time zone default (now() at time zone 'utc') NOT NULL
);

CREATE TABLE fmp_stock_full_quote (
  id SERIAL PRIMARY KEY,
  symbol VARCHAR (500) NOT NULL,
  "name" VARCHAR (500),
  price DOUBLE PRECISION,
  changes_percentage DOUBLE PRECISION,
  change DOUBLE PRECISION,
  day_low DOUBLE PRECISION,
  day_high DOUBLE PRECISION,
  year_high DOUBLE PRECISION,
  year_low DOUBLE PRECISION,
  market_cap DOUBLE PRECISION,
  price_avg50 DOUBLE PRECISION,
  price_avg200 DOUBLE PRECISION,
  exchange VARCHAR (500),
  volume BIGINT,
  avg_volume BIGINT,
  open DOUBLE PRECISION,
  previous_close DOUBLE PRECISION,
  eps DOUBLE PRECISION,
  pe DOUBLE PRECISION,
  earnings_announcement VARCHAR (500),
  shares_outstanding BIGINT,
  "timestamp" timestamp NOT NULL,
  created timestamp with time zone default (now() at time zone 'utc') NOT NULL,
  updated timestamp with time zone default (now() at time zone 'utc') NOT NULL,
  UNIQUE(symbol, "timestamp")
);

CREATE TABLE fmp_historical (
  id SERIAL PRIMARY KEY,
  symbol VARCHAR(500) NOT NULL,
  date VARCHAR (500),
  label VARCHAR (500),
  adj_dividend DOUBLE PRECISION,
  dividend DOUBLE PRECISION,
  record_date VARCHAR (500),
  payment_date VARCHAR (500),
  declaration_date VARCHAR (500),
  created timestamp with time zone default (now() at time zone 'utc') NOT NULL,
  updated timestamp with time zone default (now() at time zone 'utc') NOT NULL,
  UNIQUE(symbol, date)
);

CREATE TABLE fmp_historical_dividends (
  id SERIAL PRIMARY KEY,
  historical_id BIGINT,
  created timestamp with time zone default (now() at time zone 'utc') NOT NULL,
  updated timestamp with time zone default (now() at time zone 'utc') NOT NULL
);

CREATE TABLE fmp_economic_indicator (
  id SERIAL PRIMARY KEY,
  date VARCHAR(500) NOT NULL,
  value VARCHAR(500) NOT NULL,
  created timestamp with time zone default (now() at time zone 'utc') NOT NULL,
  updated timestamp with time zone default (now() at time zone 'utc') NOT NULL
);
