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
}

pub fn get_config() -> MatchaSettings {
	let config_dir = get_config_directory();
	let config_file_path = config_dir.join("config.json");
	let config_file = File::open(&config_file_path);

	if let Err(err) = config_file {
		panic!(
				"Failed to read config file. Please create one at \"{:?}\" if it doesn't exist.\n{}",
				config_file_path, err
		);
	}
	let config_file = config_file.unwrap();

	let config_data = serde_json::from_reader(config_file);

	if let Err(err) = config_data {
		panic!("Failed to deserialize config file.\n{}", err);
	}

	config_data.unwrap()
}

pub fn get_config_directory() -> PathBuf {
	match home_dir() {
		Some(home) => {
			let dir = Path::new(&home).join(".matcha");
			fs::create_dir_all(&dir).unwrap_or(());
			return dir;
		}
		None => {
			panic!("Failed to get home directory. Something has gone horribly wrong.");
		}
	}
}

pub fn get_plugins_directory() -> PathBuf {
	get_config_directory().join("plugins")
}
