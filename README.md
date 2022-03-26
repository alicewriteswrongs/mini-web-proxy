# mini web proxy

This is small Rust webserver which basically serves as a sanitizing web proxy.
It's quite unsophisticated! So I wouldn't rely on it for much.

## configuration

You can set `base_url` to set the url which you wish to proxy access to, either
via an environment variable or via a `Config.toml`.

Then `cargo run` should get you going in local development.
