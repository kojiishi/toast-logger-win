//! [`ToastLogger`] is a [`log`] crate logger that sends logging output
//! to the [Windows Toast Notifications].
//! This is handy when you want to present a small amount of text to users
//! from UI-less applications on Windows.
//!
//! The following example shows a toast notification saying "Hello, world".
//! ```no_run
//! # use toast_logger_win::ToastLogger;
//! # fn test() -> anyhow::Result<()> {
//! ToastLogger::builder()
//!     .max_level(log::LevelFilter::Info)
//!     .init()?;
//! log::info!("Hello, world");
//! # Ok(())
//! # }
//! ```
//! [Windows Toast Notifications]: https://learn.microsoft.com/windows/apps/design/shell/tiles-and-notifications/toast-notifications-overview

pub mod win;

mod toast_logger;
pub use toast_logger::*;
