use env_logger::Env;
use log::{debug, error, info, trace, warn};

pub fn init_logging() {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    trace!("Initialized Log::TRACE");
    debug!("Initialized Log::DEBUG");
    info!("Initialized Log::INFO");
    warn!("Initialized Log::WARN");
    error!("Initialized Log::ERROR");
}
