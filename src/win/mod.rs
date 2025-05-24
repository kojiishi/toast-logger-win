//!
//! Thin wrappers for the Windows APIs.
//!

mod winapi {
    pub use windows::UI::Notifications::{
        ToastNotification, ToastNotificationManager, ToastNotifier, ToastTemplateType,
    };
}

/// Represents a Toast Notification.
///
/// A thin wrapper for the [`windows::UI::Notifications::ToastNotification`].
///
/// Please also see the [`Windows.UI.Notifications.ToastNotification` class].
///
/// [`windows::UI::Notifications::ToastNotification`]: https://microsoft.github.io/windows-docs-rs/doc/windows/UI/Notifications/struct.ToastNotification.html
/// [`Windows.UI.Notifications.ToastNotification` class]: https://learn.microsoft.com/uwp/api/windows.ui.notifications.toastnotification
#[derive(Debug)]
pub struct ToastNotification {
    notification: winapi::ToastNotification,
}

impl ToastNotification {
    /// Create a `win::ToastNotification` with the given text.
    // # Examples
    // ```
    // # use toast_logger_win::win::{ToastNotification, ToastNotifier};
    // fn show_text(notifier: &ToastNotifier, text: &str) -> anyhow::Result<()> {
    //     let notification = ToastNotification::new_with_text(text)?;
    //     notifier.show(&notification)
    // }
    // ```
    pub fn new_with_text(text: &str) -> anyhow::Result<Self> {
        let template = winapi::ToastTemplateType::ToastText01;
        let toast_xml = winapi::ToastNotificationManager::GetTemplateContent(template)?;
        let text_node = toast_xml.SelectSingleNode(&"//text[@id=\"1\"]".into())?;
        text_node.SetInnerText(&text.into())?;
        let notification = winapi::ToastNotification::CreateToastNotification(&toast_xml)?;
        Ok(Self { notification })
    }
}

/// A thin wrapper for the [`windows::UI::Notifications::ToastNotifier`].
///
/// Please also see the [`Windows.UI.Notifications.ToastNotifier` class].
///
/// [`windows::UI::Notifications::ToastNotifier`]: https://microsoft.github.io/windows-docs-rs/doc/windows/UI/Notifications/struct.ToastNotifier.html
/// [`Windows.UI.Notifications.ToastNotifier` class]: https://learn.microsoft.com/uwp/api/windows.ui.notifications.toastnotifier
#[derive(Debug)]
pub struct ToastNotifier {
    notifier: winapi::ToastNotifier,
}

impl ToastNotifier {
    pub fn new_with_application_id(application_id: &str) -> anyhow::Result<Self> {
        let manager = winapi::ToastNotificationManager::GetDefault()?;
        let notifier = manager.CreateToastNotifierWithId(&application_id.into())?;
        Ok(Self { notifier })
    }

    pub fn show(&self, notification: &ToastNotification) -> anyhow::Result<()> {
        self.notifier.Show(&notification.notification)?;
        Ok(())
    }
}
