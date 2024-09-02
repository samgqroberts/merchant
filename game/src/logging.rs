use tracing::info;
use tracing_subscriber::{
    fmt::{self},
    prelude::*,
    EnvFilter,
};

pub fn initialize_logging() -> () {
    if !std::env::var("MERCHANT_LOG").is_ok() {
        // there is no env var for MERCHANT_LOG
        // logging is not enabled
        // do not initialize tracing (do not create log file)
        return;
    }
    // a "rolling" file appender that *never* actually rolls the file
    let file_appender = tracing_appender::rolling::never("./", "merchant.log");
    // a tracing-subscriber Layer that formats the captured spans / events and writes to the file
    let file_layer = fmt::layer().with_writer(file_appender);
    // a tracing Subscriber that has only one Layer, the file-writing layer
    let subscriber = tracing_subscriber::registry()
        .with(file_layer)
        // use the value of MERCHANT_LOG to determine the log levels
        .with(EnvFilter::from_env("MERCHANT_LOG"));
    // set the subscriber as the global default subscriber for the program
    tracing::subscriber::set_global_default(subscriber).expect("Unable to set global subscriber");
    // log a simple message
    info!("game start: initialized logging");
}
