use dotenv::dotenv;
use log::info;

use crate::{config::Config, routes::setup_routing};

#[derive(Debug)]
pub struct App {}

impl Default for App {
    fn default() -> Self {
        dotenv().ok();
        env_logger::Builder::from_default_env()
            .filter_level(log::LevelFilter::Info)
            .target(env_logger::Target::Stdout)
            .init();
        Self {}
    }
}

impl App {
    pub async fn run(&self) {
        let cfg = match Config::init() {
            Ok(cfg) => cfg,
            Err(e) => panic!("failed to init config: {}", e),
        };

        let addr = format!("0.0.0.0:{}", &cfg.listen);
        info!("listen on :{}", &cfg.listen);
        let app = setup_routing(cfg.into());
        let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
        axum::serve(listener, app).await.unwrap();
    }
}
