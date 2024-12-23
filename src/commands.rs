use tauri::{AppHandle, command, Runtime};

use crate::models::*;
use crate::Result;
use crate::TauriPluginKeepawakeExt;

#[command]
pub(crate) async fn start<R: Runtime>(
    app: AppHandle<R>,
    payload: KeepAwakeRequest,
) -> Result<()> {
    app.tauri_plugin_keepawake().start(payload, &app)
}

#[command]
pub(crate) async fn stop<R: Runtime>(
    app: AppHandle<R>,
) -> Result<()> {
    app.tauri_plugin_keepawake().stop(&app)
}
