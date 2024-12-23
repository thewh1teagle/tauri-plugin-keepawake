use std::sync::Mutex;

use tauri::{
  plugin::{Builder, TauriPlugin},
  Manager, Runtime,
};

pub use models::*;

#[cfg(desktop)]
mod desktop;

mod commands;
mod error;
mod models;

pub use error::{Error, Result};

#[cfg(desktop)]
use desktop::TauriPluginKeepawake;

/// Extensions to [`tauri::App`], [`tauri::AppHandle`] and [`tauri::Window`] to access the tauri-plugin-keepawake APIs.
pub trait TauriPluginKeepawakeExt<R: Runtime> {
  fn tauri_plugin_keepawake(&self) -> &TauriPluginKeepawake<R>;
}

impl<R: Runtime, T: Manager<R>> crate::TauriPluginKeepawakeExt<R> for T {
  fn tauri_plugin_keepawake(&self) -> &TauriPluginKeepawake<R> {
    self.state::<TauriPluginKeepawake<R>>().inner()
  }
}
struct KeepAwakeHandle(pub Mutex<Option<keepawake::KeepAwake>>);

/// Initializes the plugin.
pub fn init<R: Runtime>() -> TauriPlugin<R> {
  Builder::new("keepawake")
    .invoke_handler(tauri::generate_handler![commands::start, commands::stop])
    .setup(|app, api| {
      let tauri_plugin_keepawake = desktop::init(app, api)?;
      app.manage(tauri_plugin_keepawake);
      Ok(())
    })
    .build()
}
