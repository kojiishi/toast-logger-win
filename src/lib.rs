//! [`ToastLogger`] is a [`log`] crate logger that sends logging output
//! to the [Windows Toast Notifications].
//! This is handy when you want to present errors or
//! a small amount of text to users
//! from UI-less applications on Windows.
//!
//! The following example shows a toast notification saying "Hello, world".
//! ```no_run
//! # use toast_logger_win::{Result, ToastLogger};
//! # fn test() -> Result<()> {
//! ToastLogger::builder()
//!     .max_level(log::LevelFilter::Error)
//!     .init()?;
//! log::error!("Hello, world");
//! # Ok(())
//! # }
//! ```
//! [Windows Toast Notifications]: https://learn.microsoft.com/windows/apps/design/shell/tiles-and-notifications/toast-notifications-overview
//!
//! # Features
//!
//! * The feature `winrt-toast` switches
//!   the underlying implementation
//!   from the [`windows` crate] to the [`winrt-toast` crate].
//!   Please see the [`Notification`] for more details.
//!
//! [`winrt-toast` crate]: https://docs.rs/winrt-toast/latest/winrt_toast/
//! [`windows` crate]: https://crates.io/crates/windows

#[cfg(not(feature = "winrt-toast"))]
pub(crate) mod win;

mod notification;
pub use notification::*;

mod toast_logger;
pub use toast_logger::*;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Format(#[from] std::fmt::Error),

    #[error("ToastLogger not initialized")]
    NotInitialized,

    #[error(transparent)]
    SetLogger(#[from] log::SetLoggerError),

    #[cfg(not(feature = "winrt-toast"))]
    #[error("Windows Error: {0}")]
    Windows(#[from] windows::core::Error),

    #[cfg(feature = "winrt-toast")]
    #[error("winrt_toast Error: {0}")]
    WinToast(#[from] winrt_toast::WinToastError),
}

pub type Result<T> = std::result::Result<T, Error>;
