import { Link as A } from "@solidjs/router";
import {
	Component,
	For,
	Show,
	createEffect,
	createSignal,
	onCleanup,
	onMount,
	untrack,
} from "solid-js";

import "./Query.css";

import pluginsStore, { Plugin, updatePriorities } from "../stores/plugins";

const [query, setQuery] = createSignal("");
window.addEventListener("maccha-set-query", (event: CustomEventInit) => {
	setQuery(event.detail.toString());
});

function Query() {
	const [selectedPluginID, setSelectedPluginID] = createSignal<string | void>();

	createEffect(async () => {
		await updatePriorities(query());
		// Set the selected plugin automatically if there were just no available ones.
		if (!selectedPluginID() && availablePlugins().length > 0) {
			setSelectedPluginID(availablePlugins()[0].id);
		}
		// If there are no available plugins then nothing should be selected.
		else if (availablePlugins().length === 0) {
			setSelectedPluginID(undefined);
		}
		window.dispatchEvent(new CustomEvent("maccha-query", { detail: query() }));
	});

	const currentPluginInstance = () =>
		pluginsStore.value.find((plugin) => plugin.id === selectedPluginID());

	const availablePlugins = () =>
		pluginsStore.value.filter((plugin) => plugin.priority > 0);

	return (
		<div id="query-page">
			<input
				type="text"
				id="query-input"
				placeholder="..."
				autofocus
				autocomplete="off"
				value={query()}
				ref={(el) => {
					// Better autofocus.
					function globalFocus() {
						// el.focus();
					}
					onMount(() => {
						el.focus();
						document.body.addEventListener("keydown", globalFocus);
					});
					onCleanup(() => {
						document.body.removeEventListener("keydown", globalFocus);
					});
				}}
				onInput={(event) => {
					setQuery(event.target.value);
				}}
				onKeyUp={(event) => {
					if (event.key) {
						if (event.key.toLowerCase() === "enter") {
							window.dispatchEvent(new CustomEvent("maccha-submit", { detail: query() }));
						} else if (event.key.toLowerCase() === "escape") {
							setQuery("");
						}
					}
				}}
			/>
			<div id="content">
				<Show when={pluginsStore.value.length === 0}>
					<p id="awaiting-query-message">Install some plugins to get functionality!</p>
				</Show>
				<Show when={availablePlugins().length === 0 || !selectedPluginID()}>
					<p id="awaiting-query-message">Start typing to query your plugins!</p>
				</Show>
				<Show when={availablePlugins().length > 0 && selectedPluginID()}>
					<div id="plugin-tabs">
						<For each={availablePlugins()}>
							{(plugin) => (
								<a
									class="plugin-tab"
									classList={{
										active: selectedPluginID() === plugin.id,
									}}
									onClick={() => {
										setSelectedPluginID(plugin.id);
									}}
								>
									{plugin.name}
								</a>
							)}
						</For>
					</div>
					<PluginPanel query={query()} plugin={currentPluginInstance()!} />
				</Show>
			</div>
			<div id="bottom-bar">
				<A href="/settings">Settings</A>
			</div>
		</div>
	);
}

// const ContentPanel: Component<{
// }> = (props) => { }

const PluginPanel: Component<{
	query: string;
	plugin: Plugin;
}> = (props) => {
	let cleanup: () => void;

	onCleanup(() => {
		cleanup?.();
	});

	return (
		<>
			<Show when={props.plugin}>
				<div
					id="plugin-panel"
					ref={async (el) => {
						const cp = (
							await import(
								`data:text/javascript;base64,${untrack(() => props.plugin.panel)}`
							)
						).default;
						cleanup = cp({ root: el });
						window.dispatchEvent(
							new CustomEvent("maccha-query", { detail: untrack(() => props.query) })
						);
					}}
				/>
			</Show>
		</>
	);
};

export default Query;
