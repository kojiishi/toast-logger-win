use std::env;

use toast_logger_win::{Result, ToastLogger};

pub fn main() -> Result<()> {
    ToastLogger::builder()
        .max_level(log::LevelFilter::Info)
        .init()?;

    let args: Vec<String> = env::args().skip(1).collect();
    log::info!("{}", args.join("\n"));
    Ok(())
}
