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
