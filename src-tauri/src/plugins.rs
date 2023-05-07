use std::{
	collections::HashMap,
	ffi::{c_char, c_double},
	path::PathBuf,
};

use libloading::{Library, Symbol};

use crate::{
	config::resolve_plugin_file,
	utils::{ptr_to_string, str_to_ptr},
};

pub struct Plugin {
	pub library: Library,

	pub id: String,
	pub name: String,
}

impl Plugin {
	/// Creates a new plugin instance wrapper around a dynamic library.
	pub unsafe fn new(library_path: PathBuf) -> Plugin {
		let library = Plugin::load(library_path);

		let id: Symbol<unsafe extern "C" fn() -> *const c_char> =
			library.get(b"id").unwrap();
		let id = ptr_to_string(id());

		let name: Symbol<unsafe extern "C" fn() -> *const c_char> =
			library.get(b"name").unwrap();
		let name = ptr_to_string(name());

		Plugin { library, id, name }
	}

	pub unsafe fn load(library_path: PathBuf) -> Library {
		Library::new(resolve_plugin_file(library_path)).unwrap()
	}

	/// A function that loads a library, grabs the ID from it, and unloads it.
	/// For a bit of extra speed instead of using `Plugin::new` and grabbing more data than needed.
	pub unsafe fn get_id(library_path: PathBuf) -> String {
		let library = Plugin::load(library_path);

		let id: Symbol<unsafe extern "C" fn() -> *const c_char> =
			library.get(b"id").unwrap();
		ptr_to_string(id())
	}

	/// Grabs an arbitrary function from the library that accepts a string and returns a string.
	/// For use with the Tauri command `run_plugin_function` so the plugin can call native functions from JS.
	pub unsafe fn get_function(
		&self,
		name: &[u8],
	) -> unsafe extern "C" fn(*const c_char) -> *const c_char {
		*self.library.get(name).unwrap()
	}

	pub unsafe fn panel(&self) -> String {
		let panel: Symbol<unsafe extern "C" fn() -> *const c_char> =
			self.library.get(b"panel").unwrap();
		ptr_to_string(panel())
	}

	pub unsafe fn get_priority(&self, data: &str) -> f64 {
		let get_priority: Symbol<
			unsafe extern "C" fn(*const c_char) -> c_double,
		> = self.library.get(b"get_priority").unwrap();
		get_priority(str_to_ptr(data))
	}
}

pub struct PluginManager {
	pub plugins: HashMap<String, Box<Plugin>>,
}

impl PluginManager {
	pub fn new() -> PluginManager {
		PluginManager {
			plugins: HashMap::new(),
		}
	}

	pub unsafe fn load(&mut self, library_path: PathBuf) -> &Box<Plugin> {
		let id = Plugin::get_id(library_path.clone());
		if self.plugins.contains_key(&id) {
			return self.plugins.get(&id).unwrap();
		}

		let plugin = Plugin::new(library_path);
		self.plugins.insert(id.clone(), Box::new(plugin));

		self.plugins.get(&id).unwrap()
	}

	pub fn unload(&mut self, id: String) {
		self.plugins.remove(&id);
	}

	pub fn unload_all(&mut self) {
		self.plugins.clear();
	}
}
