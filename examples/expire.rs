#[cfg(feature = "winrt-toast")]
use toast_logger_win::Notification;
use toast_logger_win::ToastLogger;

pub fn main() -> anyhow::Result<()> {
    let mut builder = ToastLogger::builder();
    #[allow(unused_assignments)]
    #[allow(unused_mut)]
    let mut message: String = "This message doesn't expire".into();
    #[cfg(feature = "winrt-toast")]
    {
        builder.create_notification(|records| {
            let mut notification = Notification::new_with_records(records)?;
            notification.expires_in(std::time::Duration::from_secs(3));
            Ok(notification)
        });
        message = "This message should expire in 3 seconds".into();
    }
    builder.max_level(log::LevelFilter::Info).init()?;

    log::info!("{}", message);
    Ok(())
}
