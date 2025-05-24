/// A struct to own copies of parts of `log::Record` for buffering.
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
}

/// Abstracted notification.
pub struct Notification {
    #[cfg(not(feature = "winrt-toast"))]
    inner: crate::win::ToastNotification,
    #[cfg(feature = "winrt-toast")]
    inner: winrt_toast::Toast,
}

impl Notification {
    /// Construct from a string.
    #[cfg(not(feature = "winrt-toast"))]
    pub fn new_with_text(text: &str) -> anyhow::Result<Self> {
        Ok(Self {
            inner: crate::win::ToastNotification::new_with_text(text)?,
        })
    }

    #[cfg(feature = "winrt-toast")]
    pub fn new_with_text(text: &str) -> anyhow::Result<Self> {
        let mut toast = winrt_toast::Toast::new();
        toast.text1(text);
        Ok(Self { inner: toast })
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
}

/// Abstracted notifier for the `Notification`.
pub struct Notifier {
    #[cfg(not(feature = "winrt-toast"))]
    inner: crate::win::ToastNotifier,
    #[cfg(feature = "winrt-toast")]
    inner: winrt_toast::ToastManager,
}

impl Notifier {
    #[cfg(not(feature = "winrt-toast"))]
    pub fn new_with_application_id(application_id: &str) -> anyhow::Result<Self> {
        Ok(Self {
            inner: crate::win::ToastNotifier::new_with_application_id(application_id)?,
        })
    }

    #[cfg(feature = "winrt-toast")]
    pub fn new_with_application_id(application_id: &str) -> anyhow::Result<Self> {
        Ok(Self {
            inner: winrt_toast::ToastManager::new(application_id),
        })
    }

    pub fn show(&self, notification: &Notification) -> anyhow::Result<()> {
        self.inner.show(&notification.inner)?;
        Ok(())
    }
}
