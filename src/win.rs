//!
//! Thin wrappers for the Windows APIs.
//!

use std::time::Duration;

use windows::{
    Foundation::{DateTime, IReference, PropertyValue},
    Globalization::Calendar,
    UI::Notifications::{
        ToastNotification, ToastNotificationManager, ToastNotifier, ToastTemplateType,
    },
    core::{IInspectable, Interface},
};

use crate::Result;

/// Represents a Toast Notification.
///
/// A thin wrapper for the [`windows::UI::Notifications::ToastNotification`].
///
/// Please also see the [`Windows.UI.Notifications.ToastNotification` class].
///
/// [`windows::UI::Notifications::ToastNotification`]: https://microsoft.github.io/windows-docs-rs/doc/windows/UI/Notifications/struct.ToastNotification.html
/// [`Windows.UI.Notifications.ToastNotification` class]: https://learn.microsoft.com/uwp/api/windows.ui.notifications.toastnotification
#[derive(Debug)]
pub struct NotificationImpl {
    notification: ToastNotification,
}

impl NotificationImpl {
    /// Create a `win::ToastNotification` with the given text.
    // # Examples
    // ```
    // # use toast_logger_win::win::{ToastNotification, ToastNotifier};
    // fn show_text(notifier: &ToastNotifier, text: &str) -> Result<()> {
    //     let notification = ToastNotification::new_with_text(text)?;
    //     notifier.show(&notification)
    // }
    // ```
    pub fn new_with_text(text: &str) -> Result<Self> {
        let template = ToastTemplateType::ToastText01;
        let toast_xml = ToastNotificationManager::GetTemplateContent(template)?;
        let text_node = toast_xml.SelectSingleNode(&"//text[@id=\"1\"]".into())?;
        text_node.SetInnerText(&text.into())?;
        let notification = ToastNotification::CreateToastNotification(&toast_xml)?;
        Ok(Self { notification })
    }

    /// Set the expiration time to the `duration` from the current time.
    pub fn expires_in(&mut self, duration: Duration) -> Result<()> {
        let win_cal = Calendar::new()?;
        win_cal.AddSeconds(duration.as_secs() as i32)?;
        let dt = win_cal.GetDateTime()?;
        let dt_obj: IInspectable = PropertyValue::CreateDateTime(dt)?;
        let dt_ref: IReference<DateTime> = dt_obj.cast()?;
        self.notification.SetExpirationTime(&dt_ref)?;
        Ok(())
    }
}

/// A thin wrapper for the [`windows::UI::Notifications::ToastNotifier`].
///
/// Please also see the [`Windows.UI.Notifications.ToastNotifier` class].
///
/// [`windows::UI::Notifications::ToastNotifier`]: https://microsoft.github.io/windows-docs-rs/doc/windows/UI/Notifications/struct.ToastNotifier.html
/// [`Windows.UI.Notifications.ToastNotifier` class]: https://learn.microsoft.com/uwp/api/windows.ui.notifications.toastnotifier
#[derive(Debug)]
pub struct NotifierImpl {
    notifier: ToastNotifier,
}

impl NotifierImpl {
    pub fn new_with_application_id(application_id: &str) -> Result<Self> {
        let manager = ToastNotificationManager::GetDefault()?;
        let notifier = manager.CreateToastNotifierWithId(&application_id.into())?;
        Ok(Self { notifier })
    }

    pub fn show(&self, notification: &NotificationImpl) -> Result<()> {
        self.notifier.Show(&notification.notification)?;
        Ok(())
    }
}
