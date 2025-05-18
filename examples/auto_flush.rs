use std::env;

use toast_logger_win::ToastLogger;

pub fn main() -> anyhow::Result<()> {
    ToastLogger::builder()
        .max_level(log::LevelFilter::Info)
        .auto_flush(false)
        .init_logger()?;

    for arg in env::args().skip(1) {
        log::info!("{}", arg);
    }
    ToastLogger::flush()?;
    Ok(())
}
