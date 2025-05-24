use crate::win;

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
    inner: win::ToastNotification,
}

impl Notification {
    pub fn new_with_text(text: &str) -> anyhow::Result<Self> {
        Ok(Self {
            inner: win::ToastNotification::new_with_text(text)?,
        })
    }

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
    inner: win::ToastNotifier,
}

impl Notifier {
    pub fn new_with_application_id(application_id: &str) -> anyhow::Result<Self> {
        Ok(Self {
            inner: win::ToastNotifier::new_with_application_id(application_id)?,
        })
    }

    pub fn show(&self, notification: &Notification) -> anyhow::Result<()> {
        self.inner.show(&notification.inner)
    }
}
