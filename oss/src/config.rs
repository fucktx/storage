use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    pub region: Option<String>,
    pub endpoint: Option<String>,
    pub max_retry: u32,
    pub access_key: Option<String>,
    pub secret_key: Option<String>,
    pub bucket: String,
    pub default_key_prefix: Option<String>,
}
