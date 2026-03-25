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
use desktop::LuoxinglakeService;
#[cfg(mobile)]
use mobile::LuoxinglakeService;

/// Extensions to [`tauri::App`], [`tauri::AppHandle`] and [`tauri::Window`] to access the luoxinglake-service APIs.
pub trait LuoxinglakeServiceExt<R: Runtime> {
  fn luoxinglake_service(&self) -> &LuoxinglakeService<R>;
}

impl<R: Runtime, T: Manager<R>> crate::LuoxinglakeServiceExt<R> for T {
  fn luoxinglake_service(&self) -> &LuoxinglakeService<R> {
    self.state::<LuoxinglakeService<R>>().inner()
  }
}

/// Initializes the plugin.
pub fn init<R: Runtime>() -> TauriPlugin<R> {
  Builder::new("luoxinglake-service")
    .invoke_handler(tauri::generate_handler![commands::ping])
    .setup(|app, api| {
      #[cfg(mobile)]
      let luoxinglake_service = mobile::init(app, api)?;
      #[cfg(desktop)]
      let luoxinglake_service = desktop::init(app, api)?;
      app.manage(luoxinglake_service);
      Ok(())
    })
    .build()
}
