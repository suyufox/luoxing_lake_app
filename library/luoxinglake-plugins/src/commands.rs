use tauri::{command, AppHandle, Runtime};

use crate::models::*;
use crate::LuoxinglakePluginsExt;
use crate::Result;

#[command]
pub(crate) async fn ping<R: Runtime>(app: AppHandle<R>, payload: PingRequest) -> Result<PingResponse> {
  app.luoxinglake_plugins().ping(payload)
}
