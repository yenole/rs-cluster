use dotenv_config::EnvConfig;

#[derive(Debug, Clone, EnvConfig)]
pub struct Config {
    #[env_config(name = "LISTEN", default = "8080")]
    pub listen: String,
    #[env_config(name = "ASR_SECRET_ID", default = "")]
    pub asr_secret_id: String,
    #[env_config(name = "ASR_SECRET_KEY", default = "")]
    pub asr_secret_key: String,
}
