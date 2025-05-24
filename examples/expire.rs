#[cfg(feature = "winrt-toast")]
use toast_logger_win::Notification;
use toast_logger_win::ToastLogger;

pub fn main() -> anyhow::Result<()> {
    let mut builder = ToastLogger::builder();
    #[cfg(feature = "winrt-toast")]
    let message = {
        builder.create_notification(|records| {
            let mut notification = Notification::new_with_records(records)?;
            notification.expires_in(std::time::Duration::from_secs(3));
            Ok(notification)
        });
        "This message should expire in 3 seconds"
    };
    #[cfg(not(feature = "winrt-toast"))]
    let message = "This message shouldn't expire";
    builder.max_level(log::LevelFilter::Info).init()?;

    log::info!("{}", message);
    Ok(())
}
