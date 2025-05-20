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

`ToastLogger` is a [`log`] crate logger that sends logging output
to the [Windows Toast Notifications].
This is handy when you want to present a small amount of text to users
from UI-less applications on Windows.

The following example shows a toast notification saying "Hello, world".
```rust
ToastLogger::builder()
    .max_level(log::LevelFilter::Info)
    .init()?;
log::info!("Hello, world");
```

Please see the [API documentation at docs.rs][docs] for more details.

[`log`]: https://crates.io/crates/log
[Windows Toast Notifications]: https://learn.microsoft.com/windows/apps/design/shell/tiles-and-notifications/toast-notifications-overview
