# mini web proxy

This is small Rust webserver which basically serves as a sanitizing web proxy.
It's quite unsophisticated! So I wouldn't rely on it for much.

## configuration

You can set a `$BASE_URL` environment variable to the url which you wish to
proxy access to, like so (for development):

```sh
BASE_URL="https://example.com" cargo run
```
