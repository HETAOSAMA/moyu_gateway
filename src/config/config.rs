#[derive(Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ApplicationConfig {
    pub server_name: String,
    pub server_url: String,
    pub redis_url: String,
    pub redis_db: Option<u16>,
    pub database_url: String,
    pub log_dir: String,
    pub log_type: String,
    pub log_temp_size: String,
    pub log_chan_len: Option<usize>,
    pub log_pack_compress: String,
    pub log_rolling_type: String,
    pub log_level: String,
}

impl Default for ApplicationConfig {
    fn default() -> Self {
        let js_data = include_str!("../../application.json5");
        let mut result: ApplicationConfig = json5::from_str(js_data).expect("json5 parse error");
        log::info!("config: {:?}", result);
        result
    }
}