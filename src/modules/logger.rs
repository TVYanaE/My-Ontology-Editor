use std::io;
use tracing_appender::non_blocking::WorkerGuard;
use tracing_subscriber::{
    fmt, prelude::*, Registry,
    EnvFilter,
};

pub fn init_logger() -> WorkerGuard {

    // Layer for log files 
    let file_appender = tracing_appender::rolling::daily("logs", "my_ontology_editor.log");
    let (non_blocking, guard) = tracing_appender::non_blocking(file_appender);

    let filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_|{
            EnvFilter::new("error,my_ontology_editor=debug")
        });

    // Layer for terminal 
    let console_layer = fmt::layer()
        .with_writer(io::stderr)
        .pretty();

    let file_layer = fmt::layer()
        .with_writer(non_blocking)
        .with_ansi(false);

    Registry::default()
        .with(filter)
        .with(console_layer)
        .with(file_layer)
        .init();

    return guard;
}
