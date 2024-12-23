use std::sync::Mutex;

use serde::de::DeserializeOwned;
use tauri::{plugin::PluginApi, AppHandle, Manager, Runtime};
use crate::{models::*, KeepAwakeHandle};

pub fn init<R: Runtime, C: DeserializeOwned>(
  app: &AppHandle<R>,
  _api: PluginApi<R, C>,
) -> crate::Result<TauriPluginKeepawake<R>> {
  app.manage(KeepAwakeHandle(Mutex::new(None::<keepawake::KeepAwake>)));
  Ok(TauriPluginKeepawake(app.clone()))
}

/// Access to the tauri-plugin-keepawake APIs.
pub struct TauriPluginKeepawake<R: Runtime>(AppHandle<R>);

impl<R: Runtime> TauriPluginKeepawake<R> {
  pub fn start(&self, app: &AppHandle<R>, config: Option<KeepAwakeConfig>) -> crate::Result<()> {
    let waker_state = app.state::<KeepAwakeHandle>();
    let mut waker_handle = waker_state.0.lock().unwrap();
    if waker_handle.is_some() {
      return Err(crate::Error::KeepawakePlugin(crate::error::KeepAwakeError::AlreadyStarted));
    }
    let mut builder = keepawake::Builder::default();
    let builder = builder.app_name(app.package_info().name.clone());
    if let Some(config) = config {
      builder.display(config.display).idle(config.idle).sleep(config.sleep);
    }
    let new_waker = builder.create()?;
    *waker_handle = Some(new_waker);
    Ok(())
  }

  pub fn stop(&self, app: &AppHandle<R>) -> crate::Result<()> {
    let waker_state = app.state::<KeepAwakeHandle>();
    let mut waker_handle = waker_state.0.lock().unwrap();
    // Just drop the waker to stop it.
    let _ = waker_handle.take();
    Ok(())
  }
}
