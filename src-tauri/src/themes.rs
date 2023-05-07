use std::{collections::HashMap, fs, path::PathBuf};

use crate::config::resolve_theme_file;

pub struct Theme {
	pub id: String,
	pub name: String,

	pub path: PathBuf,
	pub source: String,
}

impl Theme {
	/// Creates a new theme instance.
	pub fn new(path: PathBuf) -> Theme {
		let path = resolve_theme_file(path);

		let name = path
			.file_name()
			.expect("Failed to read theme file name")
			.to_string_lossy()
			.to_string();

		let id = name.to_lowercase();

		let source = fs::read_to_string(&path)
			.expect("Failed to read theme file source");

		Theme {
			id,
			name,
			path,
			source,
		}
	}
}

pub struct ThemeManager {
	pub themes: HashMap<String, Theme>,
}

impl ThemeManager {
	pub fn new() -> ThemeManager {
		ThemeManager {
			themes: HashMap::new(),
		}
	}

	pub fn load(&mut self, path: PathBuf) -> &Theme {
		let theme = Theme::new(path.clone());
		let id = theme.id;

		if self.themes.contains_key(&id) {
			return self.themes.get(&id).unwrap();
		}

		let theme = Theme::new(path);
		self.themes.insert(id.clone(), theme);

		self.themes.get(&id).unwrap()
	}

	pub fn unload(&mut self, id: String) {
		self.themes.remove(&id);
	}

	pub fn unload_all(&mut self) {
		self.themes.clear();
	}
}
