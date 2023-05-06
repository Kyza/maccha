use std::{
	collections::HashMap,
	ffi::{c_char, c_double, c_uint, CStr},
	path::PathBuf,
};

use libloading::{Library, Symbol};

use crate::utils::{ptr_to_string, str_to_ptr};

pub struct Plugin {
	pub library: Library,

	pub id: String,
	pub name: String,
}

impl Plugin {
	/// Creates a new plugin instance wrapper around a dynamic library.
	pub unsafe fn new(library_path: PathBuf) -> Plugin {
		let library = Library::new(library_path).unwrap();

		let id: Symbol<*const &str> = library.get(b"id").unwrap();
		let id = (**id).to_string();

		let name: Symbol<*const &str> = library.get(b"name").unwrap();
		let name = (**name).to_string();

		Plugin { library, id, name }
	}

	/// A function that loads a library, grabs the ID from it, and unloads it.
	/// For a bit of extra speed instead of using `Plugin::new` and grabbing more data than needed.
	pub unsafe fn get_id(library_path: PathBuf) -> String {
		let library = Library::new(library_path).unwrap();

		let id: Symbol<*const &str> = library.get(b"id").unwrap();
		(**id).to_string()
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
}
