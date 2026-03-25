use serde::de::DeserializeOwned;
use tauri::{plugin::PluginApi, AppHandle, Runtime};

use crate::models::*;

pub fn init<R: Runtime, C: DeserializeOwned>(
  app: &AppHandle<R>,
  _api: PluginApi<R, C>,
) -> crate::Result<LuoxinglakeService<R>> {
  Ok(LuoxinglakeService(app.clone()))
}

/// Access to the luoxinglake-service APIs.
pub struct LuoxinglakeService<R: Runtime>(AppHandle<R>);

impl<R: Runtime> LuoxinglakeService<R> {
  pub fn ping(&self, payload: PingRequest) -> crate::Result<PingResponse> {
    Ok(PingResponse { value: payload.value })
  }
}
