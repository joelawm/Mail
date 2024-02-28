
/// Create a about window for the application. This routes to the /about view in the application.
#[tauri::command]
pub fn create_about_window(handle: tauri::AppHandle) {
	let _about = tauri::WindowBuilder::new(
		&handle,
		"about", 
		tauri::WindowUrl::External("http://localhost:5173/about".parse().unwrap())
	  )
	  .always_on_top(true)
	  .max_inner_size(200.0, 350.0)
	  .inner_size(200.0, 350.0)
	  .title("About")
	  .resizable(false)
	  .center()
	  .build().unwrap(); // fix me later opening 2 windows breaks me
}

/// Create a settings window for the application. This routes to the /settings view in the application.
#[tauri::command]
pub fn create_settings_window(handle: tauri::AppHandle) {
	let _settings = tauri::WindowBuilder::new(
		&handle,
		"settings", 
		tauri::WindowUrl::External("http://localhost:5173/settings".parse().unwrap())
	  )
	  .always_on_top(true)
	  .max_inner_size(600.0, 800.0)
	  .inner_size(600.0, 800.0)
	  .title("Settings")
	  .resizable(false)
	  .center()
	  .build().unwrap(); // fix me later opening 2 windows breaks me
}