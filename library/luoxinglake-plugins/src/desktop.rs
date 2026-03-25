use serde::de::DeserializeOwned;
use tauri::{plugin::PluginApi, AppHandle, Runtime};

use crate::models::*;

pub fn init<R: Runtime, C: DeserializeOwned>(
  app: &AppHandle<R>,
  _api: PluginApi<R, C>,
) -> crate::Result<LuoxinglakePlugins<R>> {
  Ok(LuoxinglakePlugins(app.clone()))
}

/// Access to the luoxinglake-plugins APIs.
pub struct LuoxinglakePlugins<R: Runtime>(AppHandle<R>);

impl<R: Runtime> LuoxinglakePlugins<R> {
  pub fn ping(&self, payload: PingRequest) -> crate::Result<PingResponse> {
    Ok(PingResponse { value: payload.value })
  }
}
