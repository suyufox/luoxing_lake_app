use tauri::{
  plugin::{Builder, TauriPlugin},
  Manager, Runtime,
};

pub use models::*;

#[cfg(desktop)]
mod desktop;
#[cfg(mobile)]
mod mobile;

mod commands;
mod error;
mod models;

pub use error::{Error, Result};

#[cfg(desktop)]
use desktop::LuoxinglakePlugins;
#[cfg(mobile)]
use mobile::LuoxinglakePlugins;

/// Extensions to [`tauri::App`], [`tauri::AppHandle`] and [`tauri::Window`] to access the luoxinglake-plugins APIs.
pub trait LuoxinglakePluginsExt<R: Runtime> {
  fn luoxinglake_plugins(&self) -> &LuoxinglakePlugins<R>;
}

impl<R: Runtime, T: Manager<R>> crate::LuoxinglakePluginsExt<R> for T {
  fn luoxinglake_plugins(&self) -> &LuoxinglakePlugins<R> {
    self.state::<LuoxinglakePlugins<R>>().inner()
  }
}

/// Initializes the plugin.
pub fn init<R: Runtime>() -> TauriPlugin<R> {
  Builder::new("luoxinglake-plugins")
    .invoke_handler(tauri::generate_handler![commands::ping])
    .setup(|app, api| {
      #[cfg(mobile)]
      let luoxinglake_plugins = mobile::init(app, api)?;
      #[cfg(desktop)]
      let luoxinglake_plugins = desktop::init(app, api)?;
      app.manage(luoxinglake_plugins);
      Ok(())
    })
    .build()
}
