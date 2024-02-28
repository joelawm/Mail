use tauri::{menu::{Menu, MenuItem, Submenu}, Wry, Manager, AppHandle};
use crate::window;

/// Function for handling menu events and maps to specific functions.
pub fn handle(event: WindowMenuEvent<Wry>) {
	match event.menu_item_id() {
	  "quit" => {
		std::process::exit(0);
	  }
	  "close" => {
		event.window().close().unwrap();
	  }
	  "about" => {
		let window = event.window().clone();
		window::create_about_window(window.app_handle())
	  }
	  "settings" => {
		let window = event.window().clone();
		window::create_settings_window(window.app_handle())
	  }
	  _ => {}
	}
  }

/// Create the base menu for the application.
pub fn menu<R>(handle: &AppHandle) -> Menu<R> {
	Menu::new(handle).unwrap()
		.append(&app_menu())
		.append(&file_menu())
		.append(&edit_menu())
}

/// Function for building app menu.
fn app_menu<R>() -> Submenu<R> {
	let about = MenuItem::new("about", "About", true);
	let settings = MenuItem::new("settings".to_string(), "Settings");
	Submenu::new("Mail", Menu::new().add_item(about).add_item(settings))
}

/// Function for building file menu.
fn file_menu<R>() -> Submenu<R> {
	let quit = CustomMenuItem::new("quit".to_string(), "Quit");
	let close = CustomMenuItem::new("close".to_string(), "Close");
	Submenu::new("File", Menu::new().add_item(quit).add_item(close))
}

/// Function for building edit menu.
fn edit_menu<R>() -> Submenu<R> {
	let quit = CustomMenuItem::new("quit".to_string(), "Quit");
	Submenu::new("Edit", Menu::new().add_item(quit))
}