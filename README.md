# Naughty Tauri Wry Plugin

This plugin is meant to be used as a reproducible example of a RefCell panic that can occur in Tauri related to
https://github.com/tauri-apps/tauri/issues/14801.

It spawns a new thread that runs in a loop, waiting for a `WindowEvent::Focused(true)`. It then mutably borrows the
`WindowMap` exposed through the `tauri-runtime-wry` public API, and sends a `WindowMessage::Close` to every window in
the map, forgets the mutable reference, and parks the thread.

This will cause a RefCell panic immediately. Enable in `tauri::App::wry_plugin(&mut self, NaughtyPluginBuilder);`.

## Example

Run the example with a `cargo run`, you should see a window flash and crash immediately with a panic similar to:
```
thread 'main' (1548796) panicked at ~/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/tauri-runtime-wry-2.10.0/src/lib.rs:4268:31:
RefCell already mutably borrowed
```