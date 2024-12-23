Tauri plugin for keeping your computer awake

| Platform | Supported |
| -------- | --------- |
| Linux    | ✓         |
| Windows  | ✓         |
| macOS    | ✓         |
| Android  | x         |
| iOS      | x         |

## Install

Install the Core plugin by adding the following to your `Cargo.toml` file:

`src-tauri/Cargo.toml`

```toml
# you can add the dependencies on the `[dependencies]` section if you do not target mobile
[dependencies]
tauri-plugin-keepawake = "0.1.0"
# alternatively with Git:
tauri-plugin-keepawake = { git = "https://github.com/thewh1teagle/tauri-plugin-keepawake", branch = "main" }
```

You can install the JavaScript Guest bindings using your preferred JavaScript package manager:

> Note: Since most JavaScript package managers are unable to install packages from git monorepos we provide read-only mirrors of each plugin. This makes installation option 2 more ergonomic to use.

```sh
npm add tauri-plugin-keepawake-api

# alternatively with Git:
bun add https://github.com/thewh1teagle/tauri-plugin-keepawake-api#main
```

## Usage

First you need to register the core plugin with Tauri:

`src-tauri/src/lib.rs`

```rust
fn main() {
    tauri::Builder::default()
        .setup(|app| {
            #[cfg(desktop)]
            app.handle().plugin(tauri_plugin_keepawake::init())?;
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```

Afterwards all the plugin's APIs are available through the JavaScript guest bindings:

```javascript
import { start, stop } from '@tauri-apps/tauri-plugin-keepawake-api'
const matches = await getMatches()
start() // Return promise. may fail
stop() // Return promise
```