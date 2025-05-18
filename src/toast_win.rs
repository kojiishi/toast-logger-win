mod win {
    pub use windows::UI::Notifications::{
        ToastNotification, ToastNotificationManager, ToastNotifier, ToastTemplateType,
    };
}

#[derive(Debug)]
pub struct ToastNotification {
    notification: win::ToastNotification,
}

impl ToastNotification {
    pub fn new_with_text(text: &str) -> anyhow::Result<Self> {
        let template = win::ToastTemplateType::ToastText01;
        let toast_xml = win::ToastNotificationManager::GetTemplateContent(template)?;
        let text_node = toast_xml.SelectSingleNode(&"//text[@id=\"1\"]".into())?;
        text_node.SetInnerText(&text.into())?;
        let notification = win::ToastNotification::CreateToastNotification(&toast_xml)?;
        Ok(Self { notification })
    }
}

#[derive(Debug)]
pub struct ToastNotifier {
    notifier: win::ToastNotifier,
}

impl ToastNotifier {
    // https://github.com/GitHub30/toast-notification-examples
    pub(crate) const DEFAULT_APP_ID: &str =
        r"{1AC14E77-02E7-4E5D-B744-2EB1AE5198B7}\WindowsPowerShell\v1.0\powershell.exe";

    pub fn new_with_application_id(application_id: &str) -> anyhow::Result<Self> {
        let manager = win::ToastNotificationManager::GetDefault()?;
        let notifier = manager.CreateToastNotifierWithId(&application_id.into())?;
        Ok(Self { notifier })
    }

    pub fn show(&self, notification: &ToastNotification) -> anyhow::Result<()> {
        self.notifier.Show(&notification.notification)?;
        Ok(())
    }
}
