use tauri::Manager;
use tauri_plugin_shell::ShellExt;
use tauri_plugin_shell::process::CommandEvent;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
  tauri::Builder::default()
    .plugin(tauri_plugin_log::Builder::default()
      .level(log::LevelFilter::Debug)
      .build())
    .plugin(tauri_plugin_shell::init())
    .setup(|app| {
      log::info!("🚀 GhostWire Tauri app starting...");
      
      // Start GhostWire Sidecar with comprehensive error handling
      match app.shell().sidecar("ghostwire") {
        Ok(mut sidecar_command) => {
          log::info!("✅ Sidecar command created successfully");

          // Ensure the sidecar has a writable home directory on Android
          if let Ok(app_data) = app.path().app_data_dir() {
            log::info!("📂 Setting GHOSTWIRE_HOME to: {:?}", app_data);
            sidecar_command = sidecar_command.env("GHOSTWIRE_HOME", app_data.to_string_lossy().to_string());
          }
          
          match sidecar_command.spawn() {
            Ok((mut rx, _child)) => {
              log::info!("✅ Sidecar process spawned successfully");
              
              tauri::async_runtime::spawn(async move {
                log::info!("📡 Starting sidecar output monitor...");
                while let Some(event) = rx.recv().await {
                    match event {
                        CommandEvent::Stdout(line_bytes) => {
                             let line = String::from_utf8_lossy(&line_bytes);
                             log::info!("GhostWire STDOUT: {}", line);
                        }
                        CommandEvent::Stderr(line_bytes) => {
                             let line = String::from_utf8_lossy(&line_bytes);
                             log::warn!("GhostWire STDERR: {}", line);
                        }
                        CommandEvent::Error(err) => {
                             log::error!("GhostWire ERROR: {}", err);
                        }
                        CommandEvent::Terminated(payload) => {
                             log::error!("GhostWire TERMINATED: code={:?}, signal={:?}", 
                                        payload.code, payload.signal);
                        }
                        _ => {}
                    }
                }
                log::warn!("📡 Sidecar output monitor ended");
              });
            }
            Err(e) => {
              log::error!("❌ Failed to spawn sidecar process: {}", e);
              log::error!("⚠️  The app will continue without the backend. WebUI will load but won't connect to mesh network.");
            }
          }
        }
        Err(e) => {
          log::error!("❌ Failed to create sidecar command: {}", e);
          log::error!("⚠️  The app will continue without the backend. WebUI will load but won't connect to mesh network.");
        }
      }

      log::info!("✅ Tauri setup complete");
      Ok(())
    })
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
