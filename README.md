# wpush.rs

[<img alt="github" src="https://img.shields.io/badge/github-saezjuan/wpush.rs-8da0cb?style=for-the-badge&labelColor=555555&logo=github" height="20">](https://github.com/saez-juan/wpush.rs)
[<img alt="crates.io" src="https://img.shields.io/crates/v/wpush?style=for-the-badge&color=fc8d62&logo=rust" height="20">](https://crates.io/crates/wpush.rs)
[<img alt="docs.rs" src="https://img.shields.io/badge/docs.rs-wpush-66c2a5?style=for-the-badge&labelColor=555555&logo=docs.rs" height="20">](https://docs.rs/wpush)

![Toast Notification on Windows](https://learn.microsoft.com/en-us/windows/apps/design/images/shell-1x.png)

This crate provides an easy way to send [toast notification](https://learn.microsoft.com/en-us/windows/apps/design/shell/tiles-and-notifications/toast-notifications-overview) on Windows. It is heavily inspired on the Go package [go-toast/toast](https://github.com/go-toast/toast).

Also, this library includes WSL support, that means, if you compile and run from WSL, the toast should appear. However, this approach may be limited because, internally, the toast feature on Windows needs to access to `regedit`, which causes some issues to get the `APP_ID`.

Go to full [crate documentation](https://docs.rs/wpush).

## Usage

Add this to your `cargo.toml`:

```toml
[dependencies]
wpush = "0.1.0"
```

On your `main.rs` you could do something like this:

```rust
use wpush::Notification;

fn main() {
    let mut n = Notification::new();

    n.set_title(Some("WPush Library"));
    n.set_message(Some("This toast was sent from Rust!"));

    n.push();
}
```

### Notification Setup

The `Notification` has some setters that can be used before pushing.

```rust
let mut n = Notification::new();

n.set_app_id(&str);           // The APP_ID is required.
                              // Default: "Windows App"
n.set_title(Option<&str>);
n.set_message(Option<&str>);
n.set_icon(Option<&str>);    // Must be a path to the image file
n.set_audio(Option<Audio>);  // Audio is a wpush enum.

n.push();
```

Some setters are missing. They are going to be added on next versions.

## Licence

This library is under **Mozilla Public Licence Version 2.0**

---

_Made by [Juan Saez](https://saez.work)_
