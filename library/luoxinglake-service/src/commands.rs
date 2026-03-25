use tauri::{command, AppHandle, Runtime};

use crate::models::*;
use crate::LuoxinglakeServiceExt;
use crate::Result;

#[command]
pub(crate) async fn ping<R: Runtime>(app: AppHandle<R>, payload: PingRequest) -> Result<PingResponse> {
  app.luoxinglake_service().ping(payload)
}
