// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

extern crate imap;
extern crate native_tls;

mod authentication;
mod client;
mod mail;
mod version;

#[tokio::main]
async fn main() {
  // Allows us to take control of the tokio runtime for external threads.
  tauri::async_runtime::set(tokio::runtime::Handle::current());

  // Create Tauri Application Build specific information
  let build = if cfg!(target_os = "macos") {
    tauri::Builder::default()
      .plugin(tauri_plugin_shell::init())
      .invoke_handler(tauri::generate_handler![
        version::package_version
      ])
  } else {
    tauri::Builder::default()
      .manage(client::ClientState::default())
      .plugin(tauri_plugin_shell::init())
      .invoke_handler(tauri::generate_handler![
        version::package_version,
        client::commands::client_connect,
        client::commands::client_disconnect,
        client::commands::get_all_inboxes,
        client::commands::get_mailbox
      ])
  };

  // Build Tauri Application
  build.run(tauri::generate_context!()).expect("error while running tauri application");
}