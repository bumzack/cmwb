use crate::auth::Keys;

pub struct Config {
    pub database_url: String,
    pub jwt_secret: String,
    pub keys: Keys,
    pub fmp_api_key: String,
    pub fed_api_key: String,
}
