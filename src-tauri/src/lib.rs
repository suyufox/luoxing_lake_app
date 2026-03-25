use tauri::Manager;
use tauri_plugin_deep_link::DeepLinkExt;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
  let mut builder = tauri::Builder::default();

  #[cfg(desktop)]
  {
    builder = builder.plugin(tauri_plugin_single_instance::init(|app, args, _cwd| {
      println!("a new app instance was opened with {args:?} and the deep link event was already triggered");

      let _ = app.get_webview_window("main").expect("no main window").set_focus();
    }));
  }

  builder = builder.plugin({
    tauri_plugin_log::Builder::new()
      .clear_targets()
      .target(tauri_plugin_log::Target::new(tauri_plugin_log::TargetKind::Stdout))
      .target(tauri_plugin_log::Target::new(tauri_plugin_log::TargetKind::Webview))
      .max_file_size(10_000_000 /* bytes */)
      .rotation_strategy(tauri_plugin_log::RotationStrategy::KeepAll)
      .format(|out, message, record| out.finish(format_args!("[{} {}] {}", record.level(), record.target(), message)))
      .timezone_strategy(tauri_plugin_log::TimezoneStrategy::UseLocal)
      .target(tauri_plugin_log::Target::new(tauri_plugin_log::TargetKind::LogDir {
        file_name: Some("laster".to_string()),
      }))
      .level(tauri_plugin_log::log::LevelFilter::Info)
      .build()
  });

  builder = builder
    .plugin(tauri_plugin_deep_link::init())
    .plugin(tauri_plugin_notification::init())
    .plugin(tauri_plugin_store::Builder::new().build())
    .plugin(tauri_plugin_os::init());

  builder = builder.setup(|app| {
    #[cfg(desktop)]
    app
      .handle()
      .plugin(tauri_plugin_window_state::Builder::default().build());

    #[cfg(any(windows, target_os = "linux"))]
    {
      app.deep_link().register_all()?;
    }

    if let Ok(Some(urls)) = app.deep_link().get_current() {
      println!("Started with: {:?}", urls);
      // app.emit("deep-link", urls).ok();
    }

    app.deep_link().on_open_url(|_urls| {
      //println!("Received: {:?}", urls);
      // app.emit_all("deep-link", urls).ok();
    });

    Ok(())
  });

  builder
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
