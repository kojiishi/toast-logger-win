use std::{env, time::Duration};

use toast_logger_win::{Notification, ToastLogger};

pub fn main() -> anyhow::Result<()> {
    let args: Vec<String> = env::args().skip(1).collect();
    let duration = if args.is_empty() {
        Duration::ZERO
    } else {
        Duration::from_secs(args[0].parse()?)
    };

    let mut builder = ToastLogger::builder();
    let message = if duration.is_zero() {
        "This message shouldn't expire".into()
    } else {
        let message = format!("This message should expire in {duration:?}.");
        builder.create_notification(move |records| {
            let mut notification = Notification::new_with_records(records)?;
            notification.expires_in(duration)?;
            Ok(notification)
        });
        message
    };
    builder.max_level(log::LevelFilter::Info).init()?;

    log::info!("{message}");
    Ok(())
}
