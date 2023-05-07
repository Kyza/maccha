# ![](/src-tauri/icons/Square30x30Logo.png) Maccha

I hate coffee.

Maccha is an extremely extensible and themable power menu for Windows, macOS, and Linux.

## Plugins

Plugins are written in Rust (other languages may work but are untested) and TypeScript/JavaScript. They're compiled to dynamic libraries (`.dll` and `.so` depending on the system) for distribution.

The JS side of plugins is used for creating the UI. They can call native (Rust) functions they provide, or any Tauri API to perform native or heavy operations.

### UI

Each plugin gets its own tab in the UI. Once the user has typed something into the query bar, the relevant tabs will appear.

### Priority

As the user types out their query, plugins will be asked for an estimate on the priority they think they should appear with.

The higher the priority, the closer to the start of the list. Meanwhile, a priority of `0` means the plugin's tab won't display at all.

### Settings

Each plugin also has its own settings page. Currently, exactly how this will be handled and the storage location of the settings is to be determined.

## Themes

Themes are single CSS files that get loaded into the webview.

## Todo

A list of things I want to do.

- [ ] Plugins.
	- [x] Plugin manager.
	- [x] UI loader.
	- [x] Priority API.
	- [x] Query state reactivity APIs.
	- [ ] Fix plugin panel switching.
	- [ ] Quick plugin installer.
	- [x] Plugin settings API.
	- [ ] Plugin settings pages.
	- [ ] Rust plugin utility crate.
	- [ ] Better plugin error handling.
- [x] Themes.
	- [x] Theme manager.
	- [x] Theme loader.
- [ ] Improved keyboard navigation.
- [x] Refresh plugins/themes button.
- [ ] Maccha settings page.
- [x] Automatically create config file.
- [ ] Window vibrancy.
	- [ ] Settings.
	- [ ] Blur (Windows 7+).
	- [ ] Acrylic (Windows 10+).
	- [ ] Mica (Windows 11+).
	- [ ] Vibrancy (macOS 10.10+).
