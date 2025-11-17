use tracing_subscriber::{EnvFilter, fmt, layer::SubscriberExt, util::SubscriberInitExt};

use crate::config;

pub fn init() {
    let logger_config = config::get_config().logger();
    let log_level = logger_config.level();
    let format_config = logger_config.format();

    let layer = fmt::layer()
        .with_level(format_config.level())
        .with_file(format_config.file())
        .with_line_number(format_config.line_number())
        .with_target(format_config.target())
        .with_thread_ids(format_config.thread_ids())
        .with_thread_names(format_config.thread_names())
        .with_ansi(format_config.ansi());

    tracing_subscriber::registry()
        .with(EnvFilter::new(log_level))
        .with(layer)
        .init();
}
