/* @refresh reload */
import { render } from "solid-js/web";

import App from "./App";
import "./styles.css";

import { invoke, window as twindow } from "@tauri-apps/api";
import {
	isRegistered,
	register,
	unregisterAll,
} from "@tauri-apps/api/globalShortcut";

(async () => {
	if (await isRegistered("Alt+Space")) {
		unregisterAll();
	}
	await register("Alt+Space", async () => {
		const win = twindow.getCurrent();
		if (await win.isVisible()) {
			win.hide();
		} else {
			await win.show();
			win.setFocus();
		}
	});

	const isProduction = await invoke("is_production");
	if (isProduction) {
		const win = twindow.getCurrent();
		await win.onFocusChanged(({ payload: focused }) => {
			if (!focused) win.hide();
		});
	}

	render(() => <App />, document.getElementById("root") as HTMLElement);
})();
