use std::env;

use toast_logger_win::ToastLogger;

pub fn main() -> anyhow::Result<()> {
    ToastLogger::builder()
        .max_level(log::LevelFilter::Info)
        .init_logger()?;

    for arg in env::args().skip(1) {
        log::info!("{}", arg);
    }
    Ok(())
}
