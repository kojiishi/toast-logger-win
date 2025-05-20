[![CI-badge]][CI]
[![crate-badge]][crate]
[![docs-badge]][docs]

[CI-badge]: https://github.com/kojiishi/toast-logger-win/actions/workflows/rust-ci.yml/badge.svg
[CI]: https://github.com/kojiishi/toast-logger-win/actions/workflows/rust-ci.yml
[crate-badge]: https://img.shields.io/crates/v/toast-logger-win.svg
[crate]: https://crates.io/crates/toast-logger-win
[docs-badge]: https://docs.rs/toast-logger-win/badge.svg
[docs]: https://docs.rs/toast-logger-win/

# toast-logger-win

Rust's [`log`] crate logger that sends logging output
to the Windows Toast notifications.

This is handy when you want a small amount of output
from UI-less applications on Windows.

The following example shows a toast notification saying "Hello, world".
```rust
ToastLogger::builder()
    .max_level(log::LevelFilter::Info)
    .init_logger()?;
log::info!("Hello, world");
```

For more details, please see [docs.rs][docs].

[`log`]: https://crates.io/crates/log
