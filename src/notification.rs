#[cfg(doc)]
use crate::ToastLoggerBuilder;

/// A struct to own copies of parts of `log::Record` for buffering.
///
/// The `log::Record` has lifetime for the lower overhead,
/// and it's not suitable to buffer them.
/// This struct captures the data needed for longer lifetime.
#[derive(Debug, PartialEq, Eq)]
pub struct BufferedRecord {
    pub level: log::Level,
    pub args: String,
}

impl BufferedRecord {
    pub fn new_with_formatted_args(record: &log::Record, args: &str) -> Self {
        Self {
            level: record.level(),
            args: args.to_string(),
        }
    }

    pub fn level(&self) -> log::Level {
        self.level
    }

    pub fn args(&self) -> &str {
        &self.args
    }
}

/// Abstracted notification.
///
/// [`ToastLoggerBuilder::create_notification`] provides a hooking point
/// before the [`Notification`] is shown.
pub struct Notification {
    #[cfg(not(feature = "winrt-toast"))]
    inner: crate::win::ToastNotification,
    #[cfg(feature = "winrt-toast")]
    inner: winrt_toast::Toast,
}

impl Notification {
    /// Construct from a string.
    pub fn new_with_text(text: &str) -> anyhow::Result<Self> {
        Ok(Self {
            #[cfg(not(feature = "winrt-toast"))]
            inner: crate::win::ToastNotification::new_with_text(text)?,
            #[cfg(feature = "winrt-toast")]
            inner: {
                let mut toast = winrt_toast::Toast::new();
                toast.text1(text);
                toast
            },
        })
    }

    /// Construct from a list of [`BufferedRecord`].
    pub fn new_with_records(records: &[BufferedRecord]) -> anyhow::Result<Self> {
        let text = records
            .iter()
            .map(|r| r.args.as_str())
            .collect::<Vec<_>>()
            .join("\n");
        Self::new_with_text(&text)
    }

    /// Set the expirations of this notification.
    /// Please see [`ToastNotification.ExpirationTime`].
    ///
    /// Available only when the "`winrt-toast`" feature is enabled.
    ///
    /// [`ToastNotification.ExpirationTime`]: https://learn.microsoft.com/uwp/api/windows.ui.notifications.toastnotification.expirationtime
    #[cfg(any(feature = "winrt-toast", doc))]
    pub fn expires_in(&mut self, duration: std::time::Duration) {
        self.inner.expires_in(duration);
    }

    /// The inner [`winrt_toast::Toast`].
    ///
    /// Available only when the "`winrt-toast`" feature is enabled.
    #[cfg(feature = "winrt-toast")]
    pub fn inner(&self) -> &winrt_toast::Toast {
        &self.inner
    }
}

/// Abstracted notifier for the `Notification`.
pub(crate) struct Notifier {
    #[cfg(not(feature = "winrt-toast"))]
    inner: crate::win::ToastNotifier,
    #[cfg(feature = "winrt-toast")]
    inner: winrt_toast::ToastManager,
}

impl Notifier {
    pub fn new_with_application_id(application_id: &str) -> anyhow::Result<Self> {
        Ok(Self {
            #[cfg(not(feature = "winrt-toast"))]
            inner: crate::win::ToastNotifier::new_with_application_id(application_id)?,
            #[cfg(feature = "winrt-toast")]
            inner: winrt_toast::ToastManager::new(application_id),
        })
    }

    pub fn show(&self, notification: &Notification) -> anyhow::Result<()> {
        self.inner.show(&notification.inner)?;
        Ok(())
    }
}
