use std::{
    fmt, mem,
    sync::{Mutex, OnceLock},
};

use log::Log;

use crate::{BufferedRecord, Notification, Notifier};

type LogRecordFormatter =
    dyn Fn(&mut dyn fmt::Write, &log::Record) -> fmt::Result + Send + Sync + 'static;
type NotificationCreator =
    dyn Fn(&[BufferedRecord]) -> anyhow::Result<Notification> + Send + Sync + 'static;

struct ToastLoggerConfig {
    max_level: log::LevelFilter,
    is_auto_flush: bool,
    application_id: String,
    formatter: Box<LogRecordFormatter>,
    create_notification: Box<NotificationCreator>,
}

impl Default for ToastLoggerConfig {
    fn default() -> Self {
        Self {
            max_level: log::LevelFilter::Error,
            is_auto_flush: true,
            application_id: Self::DEFAULT_APP_ID.into(),
            formatter: Box::new(Self::default_formatter),
            create_notification: Box::new(Notification::new_with_records),
        }
    }
}

impl ToastLoggerConfig {
    // https://github.com/GitHub30/toast-notification-examples
    const DEFAULT_APP_ID: &str =
        r"{1AC14E77-02E7-4E5D-B744-2EB1AE5198B7}\WindowsPowerShell\v1.0\powershell.exe";

    fn default_formatter(buf: &mut dyn fmt::Write, record: &log::Record) -> fmt::Result {
        write!(buf, "{}: {}", record.level(), record.args())
    }

    fn create_notifier(&self) -> anyhow::Result<Notifier> {
        Notifier::new_with_application_id(&self.application_id)
    }
}

/// Builder for [`ToastLogger`].
///
/// # Examples
/// ```no_run
/// # use toast_logger_win::ToastLogger;
/// # fn test() -> anyhow::Result<()> {
/// ToastLogger::builder().init()?;
/// # Ok(())
/// # }
/// ```
#[derive(Default)]
pub struct ToastLoggerBuilder {
    config: ToastLoggerConfig,
}

impl ToastLoggerBuilder {
    fn new() -> Self {
        Self::default()
    }

    /// Initialize the [`log`] crate to use the [`ToastLogger`]
    /// with the configurations set to this builder.
    pub fn init(&mut self) -> anyhow::Result<()> {
        ToastLogger::init(self.build_config())
    }

    #[deprecated(since = "0.2.0", note = "Use `init()` instead")]
    pub fn init_logger(&mut self) -> anyhow::Result<()> {
        self.init()
    }

    /// Build a `ToastLogger`.
    ///
    /// The returned logger implements the [`Log`] trait
    /// and can be installed manually or nested within another logger.
    pub fn build(&mut self) -> anyhow::Result<ToastLogger> {
        ToastLogger::new(self.build_config())
    }

    fn build_config(&mut self) -> ToastLoggerConfig {
        mem::take(&mut self.config)
    }

    /// Set the maximum level of logs to be displayed.
    /// Logs above the specified level are discarded.
    pub fn max_level(&mut self, level: log::LevelFilter) -> &mut Self {
        self.config.max_level = level;
        self
    }

    /// Set whether to show a toast notification on each logging,
    /// or only when explicitly specified.
    /// When this is set to `false`,
    /// logs are appended to an internal buffer
    /// without being shown,
    /// until [`ToastLogger::flush()`] is called.
    ///
    /// The default value is `true`,
    /// which shows a toast notification on each logging.
    /// # Examples
    /// ```no_run
    /// # use toast_logger_win::ToastLogger;
    /// # fn test() -> anyhow::Result<()> {
    /// ToastLogger::builder()
    ///     .max_level(log::LevelFilter::Info)
    ///     .auto_flush(false)
    ///     .init()?;
    /// log::info!("Test info log");
    /// log::info!("Test info log 2");
    /// ToastLogger::flush()?;  // Shows only one notification with both logs.
    /// #  Ok(())
    /// # }
    /// ```
    pub fn auto_flush(&mut self, is_auto_flush: bool) -> &mut Self {
        self.config.is_auto_flush = is_auto_flush;
        self
    }

    /// Set the application ID for the Toast Notification.
    ///
    /// This is the application ID passed to the Windows [`CreateToastNotifier`] API.
    /// Please also see the [Application User Model ID][AUMID],
    /// and the "[Find the Application User Model ID of an installed app]".
    /// 
    /// [AUMID]: https://learn.microsoft.com/windows/win32/shell/appids
    /// [`CreateToastNotifier`]: https://learn.microsoft.com/uwp/api/windows.ui.notifications.toastnotificationmanager.createtoastnotifier#windows-ui-notifications-toastnotificationmanager-createtoastnotifier(system-string)
    /// [Find the Application User Model ID of an installed app]: https://learn.microsoft.com/windows/configuration/find-the-application-user-model-id-of-an-installed-app
    pub fn application_id(&mut self, application_id: &str) -> &mut Self {
        self.config.application_id = application_id.into();
        self
    }

    // https://docs.rs/env_logger/0.11.8/env_logger/#using-a-custom-format
    /// Set a custom formatter function
    /// that writes [`log::Record`] to [`fmt::Write`].
    ///
    /// The default formatter writes the logs with their levels as prefixes.
    /// # Examples
    /// ```no_run
    /// # use std::fmt;
    /// # use toast_logger_win::ToastLogger;
    /// # fn test() -> anyhow::Result<()> {
    /// ToastLogger::builder()
    ///     .format(|buf: &mut dyn fmt::Write, record: &log::Record| {
    ///         match record.level() {
    ///             log::Level::Info => buf.write_fmt(*record.args()),
    ///             _ => write!(buf, "{}: {}", record.level(), record.args()),
    ///         }
    ///     })
    ///     .init()?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn format<F>(&mut self, formatter: F) -> &mut Self
    where
        F: Fn(&mut dyn fmt::Write, &log::Record) -> fmt::Result + Send + Sync + 'static,
    {
        self.config.formatter = Box::new(formatter);
        self
    }

    /// Set a custom function to create the [`Notification`].
    /// # Examples
    /// ```
    /// # use toast_logger_win::{BufferedRecord, Notification, ToastLogger};
    /// let builder = ToastLogger::builder()
    ///     .create_notification(|records: &[BufferedRecord]| {
    ///         let notification = Notification::new_with_records(records);
    ///         // Change properties of `notification` as needed.
    ///         notification
    ///     });
    /// ```
    pub fn create_notification<F>(&mut self, create: F) -> &mut Self
    where
        F: Fn(&[BufferedRecord]) -> anyhow::Result<Notification> + Send + Sync + 'static,
    {
        self.config.create_notification = Box::new(create);
        self
    }
}

/// [`log`] crate logger that
/// implements [`log::Log`] trait.
/// It sends the logging output to the [Windows Toast Notifications].
///
/// # Examples
/// ```no_run
/// # use toast_logger_win::ToastLogger;
/// # pub fn test() -> anyhow::Result<()> {
///   ToastLogger::builder()
///       .max_level(log::LevelFilter::Info)
///       .init()?;
///   log::info!("Test info log");  // Shows a Windows Toast Notification.
/// #  Ok(())
/// # }
/// ```
/// [Windows Toast Notifications]: https://learn.microsoft.com/windows/apps/design/shell/tiles-and-notifications/toast-notifications-overview
pub struct ToastLogger {
    config: ToastLoggerConfig,
    notifier: Notifier,
    records: Mutex<Vec<BufferedRecord>>,
}

static INSTANCE: OnceLock<ToastLogger> = OnceLock::new();

impl ToastLogger {
    /// Returns a [`ToastLoggerBuilder`] instance
    /// that can build a [`ToastLogger`] with various configurations.
    pub fn builder() -> ToastLoggerBuilder {
        ToastLoggerBuilder::new()
    }

    fn init(config: ToastLoggerConfig) -> anyhow::Result<()> {
        log::set_max_level(config.max_level);
        if INSTANCE.set(Self::new(config)?).is_err() {
            panic!("ToastLogger already initialized.");
        }
        log::set_logger(INSTANCE.get().unwrap())?;
        Ok(())
    }

    fn new(config: ToastLoggerConfig) -> anyhow::Result<Self> {
        let notifier = config.create_notifier()?;
        Ok(Self {
            config,
            notifier,
            records: Mutex::new(Vec::new()),
        })
    }

    /// Flush the internal log buffer.
    /// If the buffer is not empty,
    /// this function shows one toast notification
    /// by concatenating all logs in the buffer.
    ///
    /// Please see [`ToastLoggerBuilder::auto_flush()`] for more details.
    pub fn flush() -> anyhow::Result<()> {
        let logger = INSTANCE
            .get()
            .ok_or_else(|| anyhow::anyhow!("ToastLogger not initialized."))?;
        logger.flush_result()
    }

    fn take_records(&self) -> Option<Vec<BufferedRecord>> {
        let mut records = self.records.lock().unwrap();
        if records.is_empty() {
            return None;
        }
        Some(mem::take(&mut *records))
    }

    fn log_result(&self, record: &log::Record) -> anyhow::Result<()> {
        if !self.enabled(record.metadata()) {
            return Ok(());
        }

        let mut text = String::new();
        (self.config.formatter)(&mut text, record)?;
        if text.is_empty() {
            return Ok(());
        }
        let buffered_record = BufferedRecord::new_with_formatted_args(record, &text);

        if self.config.is_auto_flush {
            self.show_notification(&[buffered_record])?;
            return Ok(());
        }

        // If not auto-flushing, append to the internal buffer.
        let mut records = self.records.lock().unwrap();
        records.push(buffered_record);
        Ok(())
    }

    fn flush_result(&self) -> anyhow::Result<()> {
        if let Some(records) = self.take_records() {
            return self.show_notification(&records);
        }
        Ok(())
    }

    fn show_notification(&self, records: &[BufferedRecord]) -> anyhow::Result<()> {
        let notification = (self.config.create_notification)(records)?;
        self.notifier.show(&notification)?;
        Ok(())
    }
}

impl log::Log for ToastLogger {
    fn enabled(&self, metadata: &log::Metadata) -> bool {
        metadata.level() <= self.config.max_level
    }

    fn log(&self, record: &log::Record) {
        if let Err(error) = self.log_result(record) {
            eprintln!("Error while logging: {error}");
        }
    }

    fn flush(&self) {
        if let Err(error) = self.flush_result() {
            eprintln!("Error flushing: {error}");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builder_default() {
        let builder = ToastLogger::builder();
        assert_eq!(builder.config.max_level, log::LevelFilter::Error);
        assert!(builder.config.is_auto_flush);
        assert_eq!(
            builder.config.application_id,
            ToastLoggerConfig::DEFAULT_APP_ID
        );
    }

    #[test]
    fn max_level() -> anyhow::Result<()> {
        let logger = ToastLogger::builder()
            .max_level(log::LevelFilter::Info)
            .auto_flush(false)
            .build()?;
        let info = log::Record::builder()
            .level(log::Level::Info)
            .args(format_args!("test"))
            .build();
        let debug = log::Record::builder()
            .level(log::Level::Debug)
            .args(format_args!("test"))
            .build();
        logger.log(&debug);
        assert_eq!(logger.take_records(), None);
        logger.log(&info);
        assert_eq!(
            logger.take_records().unwrap_or_default(),
            [BufferedRecord {
                level: log::Level::Info,
                args: "INFO: test".into()
            }]
        );
        Ok(())
    }

    #[test]
    fn format() -> anyhow::Result<()> {
        let logger = ToastLogger::builder()
            .max_level(log::LevelFilter::Info)
            .auto_flush(false)
            .format(|buf: &mut dyn fmt::Write, record: &log::Record| buf.write_fmt(*record.args()))
            .build()?;
        let info = log::Record::builder()
            .level(log::Level::Info)
            .args(format_args!("test"))
            .build();
        logger.log(&info);
        assert_eq!(
            logger.take_records().unwrap_or_default(),
            [BufferedRecord {
                level: log::Level::Info,
                args: "test".into()
            }]
        );
        Ok(())
    }
}
