use std::{
	fs::{self, File},
	path::{Path, PathBuf},
};

use home::home_dir;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MatchaSettings {
	#[serde(default)]
	pub plugins: Vec<String>,
	#[serde(default)]
	pub themes: Vec<String>,
}

pub fn get_config() -> MatchaSettings {
	let config_dir = get_config_directory();
	let config_file_path = config_dir.join("config.json");
	let mut config_file = File::open(&config_file_path);

	if let Err(_) = config_file {
		fs::write(&config_file_path, "{}").unwrap_or_else(|_| panic!("Failed to create config file. Please create one manually at \"{:?}\"",
			config_file_path));
		config_file = File::open(&config_file_path);
	}
	let config_file = config_file.unwrap();

	let config_data = serde_json::from_reader(config_file);

	config_data.unwrap_or_else(|_| panic!("Failed to deserialize config file at \"{:?}\"",
		config_file_path))
}

pub fn get_config_directory() -> PathBuf {
	match home_dir() {
		Some(home) => {
			let dir = Path::new(&home).join(".config").join("matcha");
			fs::create_dir_all(&dir).unwrap_or_else(|_| panic!("Failed to create config directory at \"{:?}\"",
				dir));
			dir
		}
		None => {
			panic!("Failed to get home directory. Something has gone horribly wrong.");
		}
	}
}

pub fn get_plugins_directory() -> PathBuf {
	get_config_directory().join("plugins")
}

pub fn get_themes_directory() -> PathBuf {
	get_config_directory().join("themes")
}

pub fn resolve_plugin_file(path: PathBuf) -> PathBuf {
	if path.is_relative() {
		get_plugins_directory().join(path)
	} else {
		path
	}
}

pub fn resolve_theme_file(path: PathBuf) -> PathBuf {
	if path.is_relative() {
		get_themes_directory().join(path)
	} else {
		path
	}
}
