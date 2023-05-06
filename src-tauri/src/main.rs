// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![feature(let_chains)]
#![feature(box_into_inner)]

use std::{path::Path, sync::Mutex};

use base64::{engine::general_purpose, Engine as _};

use once_cell::sync::Lazy;

use config::get_config;
use phf::phf_map;
use plugins::PluginManager;
use themes::ThemeManager;
use utils::{ptr_to_string, str_to_ptr};

pub mod config;
pub mod plugins;
pub mod themes;
pub mod utils;

#[tauri::command]
fn is_production() -> bool {
	if cfg!(dev) {
		false
	} else {
		true
	}
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
fn get_plugin_priority(id: &str, data: String) -> f64 {
	let plugin_manager = PLUGIN_MANAGER.lock().unwrap();

	unsafe { plugin_manager.plugins.get(id).unwrap().get_priority(&data) }
}

fn main() {
	tauri::Builder::default()
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
			get_plugin_priority
		])
		.run(tauri::generate_context!())
		.expect("Error while running Maccha");
}
