use tauri::{AppHandle, command, Runtime};

use crate::models::*;
use crate::Result;
use crate::TauriPluginKeepawakeExt;

#[command]
pub(crate) async fn start<R: Runtime>(
    app: AppHandle<R>,
    config: Option<KeepAwakeConfig>,
) -> Result<()> {
    app.tauri_plugin_keepawake().start(&app, config)
}

#[command]
pub(crate) async fn stop<R: Runtime>(
    app: AppHandle<R>,
) -> Result<()> {
    app.tauri_plugin_keepawake().stop(&app)
}
