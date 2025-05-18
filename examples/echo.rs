use std::env;

use toast_logger_win::ToastLogger;

pub fn main() -> anyhow::Result<()> {
    ToastLogger::builder()
        .max_level(log::LevelFilter::Info)
        .init_logger()?;

    let args: Vec<String> = env::args().skip(1).collect();
    log::info!("{}", args.join("\n"));
    Ok(())
}
