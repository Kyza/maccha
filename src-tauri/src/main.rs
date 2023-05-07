// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{path::Path, process, sync::Mutex};

use base64::{engine::general_purpose, Engine as _};

use once_cell::sync::Lazy;

use config::get_config;
use plugins::PluginManager;
use tauri::{
	CustomMenuItem, Manager, SystemTray, SystemTrayEvent, SystemTrayMenu,
};
use themes::ThemeManager;
use utils::{ptr_to_string, str_to_ptr};

pub mod config;
pub mod plugins;
pub mod themes;
pub mod utils;

#[tauri::command]
fn is_production() -> bool {
	!cfg!(dev)
}

static PLUGIN_MANAGER: Lazy<Mutex<PluginManager>> =
	Lazy::new(|| Mutex::new(PluginManager::new()));
static THEME_MANAGER: Lazy<Mutex<ThemeManager>> =
	Lazy::new(|| Mutex::new(ThemeManager::new()));

#[tauri::command]
fn run_plugin_function(id: &str, name: &str, data: String) -> String {
	let plugin_manager = PLUGIN_MANAGER.lock().unwrap();

	unsafe {
		ptr_to_string(plugin_manager
			.plugins
			.get(id)
			.unwrap()
			.get_function(name.as_bytes())(str_to_ptr(&data)))
	}
}

#[tauri::command]
fn get_storage_directory(storage_name: String) -> String {
	config::get_storage_directory()
		.join(storage_name)
		.to_string_lossy()
		.to_string()
}

#[tauri::command]
fn get_plugin_priority(id: &str, data: String) -> f64 {
	let plugin_manager = PLUGIN_MANAGER.lock().unwrap();

	unsafe { plugin_manager.plugins.get(id).unwrap().get_priority(&data) }
}

#[derive(Clone, serde::Serialize)]
struct Payload {
	args: Vec<String>,
	cwd: String,
}

fn main() {
	let quit = CustomMenuItem::new("quit".to_string(), "Quit");
	let tray_menu = SystemTrayMenu::new().add_item(quit);
	tauri::Builder::default()
		.plugin(tauri_plugin_store::Builder::default().build())
		.plugin(tauri_plugin_window_state::Builder::default().build())
		.plugin(tauri_plugin_single_instance::init(|app, argv, cwd| {
			let window = app.get_window("main").expect("Failed to find the main window");
			window.show().expect("Failed to show the main window");
			window.set_focus().expect("Failed to focus the main window");

			app.emit_all("single-instance", Payload { args: argv, cwd }).unwrap();
		}))
		.system_tray(SystemTray::new().with_menu(tray_menu))
		.on_system_tray_event(|app, event| match event {
			SystemTrayEvent::LeftClick { .. } => {
				let window = app.get_window("main").expect("Failed to find the main window");
				window.show().expect("Failed to show the main window");
			window.set_focus().expect("Failed to focus the main window");
			}
			SystemTrayEvent::MenuItemClick { id, .. } => {
				if id.as_str() == "quit" {
					process::exit(0);
				}
			}
			_ => {}
		})
		.on_page_load(|window, _page_load_payload| {
			PLUGIN_MANAGER.lock().unwrap().unload_all();
			THEME_MANAGER.lock().unwrap().unload_all();

			_ = window.eval(
				"globalThis.waitFor = (name, cb) => {
				let interval = setInterval(() => {
					if (globalThis[name] != null) {
						clearInterval(interval);
						cb();
					}
				}, 0);
			};",
			);

			let config = get_config();

			for plugin in config.plugins {
				let library_path = Path::new(&plugin).to_path_buf();

				unsafe {
					let mut plugin_manager = PLUGIN_MANAGER.lock().unwrap();
					let plugin = plugin_manager.load(library_path);
					let id = &plugin.id;
					let name = &plugin.name;
					let panel = general_purpose::STANDARD_NO_PAD.encode(plugin.panel());

					window
						.eval(&format!("console.log('Registering plugin {name}.');
						globalThis.waitFor('plugins', () => {{
						console.log('Registered plugin {name}.');
						globalThis.plugins.register({{ id: '{id}', name: '{name}', panel: `{panel}` }});
					}});"))
						.unwrap();
				}
			}

			for theme in config.themes {
				let theme_path = Path::new(&theme).to_path_buf();
				let mut theme_manager = THEME_MANAGER.lock().unwrap();
				let theme = theme_manager.load(theme_path);

				let name = &theme.name;
				let source = &theme.source;

				window
					.eval(&format!("console.log('Registering theme {name}.');let theme = document.createElement('style');theme.textContent=`{source}`;document.head.appendChild(theme);"))
					.unwrap();

				// window
				// 	.eval(&format!("console.log('Registering theme {name}.');
				// 	globalThis.waitFor('plugins', () => {{
				// 	console.log('Registered theme {name}.');
				// 	globalThis.plugins.register({{ id: '{id}', name: '{name}', panel: `{panel}` }});
				// }});"))
				// 	.unwrap();
			}
		})
		// .setup(|app| {
		// 	let window =
		// 		app.get_window("main").expect("Failed to get main window");

		// 	Ok(())
		// })
		.invoke_handler(tauri::generate_handler![
			is_production,
			run_plugin_function,
			get_plugin_priority,
			get_storage_directory
		])
		.run(tauri::generate_context!())
		.expect("Error while running Maccha");
}
