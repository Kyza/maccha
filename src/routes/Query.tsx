import { Link as A, useParams } from "@solidjs/router";
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
	const params = useParams<{ pluginID?: string }>();

	createEffect(async () => {
		await updatePriorities(query());
		window.dispatchEvent(new CustomEvent("maccha-query", { detail: query() }));
	});

	const currentPlugin = () =>
		pluginsStore.value.find((plugin) => plugin.id === params?.pluginID);

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
				<Show when={pluginsStore.value[0]?.priority === 0}>
					<div style={{ "text-align": "center" }}>
						<p>Install some plugins to get functionality!</p>
						<p>If you have, start typing to use them!</p>
					</div>
				</Show>
				<Show when={availablePlugins().length > 0}>
					<div>
						<For each={availablePlugins()}>
							{(plugin) => <A href={`/query/${plugin.id}`}>{plugin.name}</A>}
						</For>
					</div>
					<Show when={params?.pluginID}>
						<PluginPanel query={query()} plugin={currentPlugin()!} />
					</Show>
					<Show when={!params?.pluginID && pluginsStore.value[0]?.priority > 0}>
						<PluginPanel query={query()} plugin={pluginsStore.value[0]} />
					</Show>
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
