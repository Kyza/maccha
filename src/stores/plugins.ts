import { invoke } from "@tauri-apps/api";
import { batch } from "solid-js";
import { createStore, produce } from "solid-js/store";

export type Plugin = {
	id: string;
	name: string;
	panel: string;
	priority: number;
};

export type Plugins = Plugin[];

const [_plugins, setPlugins] = createStore<Plugins>([]);

export function register(plugin: Plugin) {
	setPlugins(
		produce((plugins) => {
			plugin.priority = 0;
			plugins.push(plugin);
		})
	);
}

export function unregister(id: string) {
	setPlugins(
		produce((plugins) => {
			const plugin = plugins.find((plugin) => plugin.id === id);
			if (plugin) {
				plugins = plugins.splice(plugins.indexOf(plugin), 1);
			}
		})
	);
}

export async function updatePriorities(query: string) {
	await batch(async () => {
		let i = 0;
		for (const plugin of _plugins) {
			const priority: number = await invoke("get_plugin_priority", {
				id: plugin.id,
				data: query,
			});

			setPlugins(
				produce((plugins) => {
					plugins[i].priority = priority;
				})
			);
			i++;
		}
		setPlugins(
			produce((plugins) => plugins.sort((a, b) => a.priority - b.priority))
		);
	});
}

declare global {
	var plugins: {
		value: typeof _plugins;
		set: typeof setPlugins;
		register: typeof register;
		unregister: typeof unregister;
		updatePriorities: typeof updatePriorities;
	};
}

globalThis.plugins = {
	value: _plugins,
	set: setPlugins,
	register,
	unregister,
	updatePriorities,
};
export default globalThis.plugins;
