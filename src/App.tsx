import { Route, Router, Routes, useLocation } from "@solidjs/router";
import { createEffect } from "solid-js";

import Home from "./routes/Home";
import Query from "./routes/Query";
import Settings from "./routes/Settings";

function App() {
	return (
		<Router>
			<AppRouter />
		</Router>
	);
}

function AppRouter() {
	const location = useLocation();

	createEffect(() => {
		console.debug(
			"[Router] [Location]",
			`${location.pathname}${location.hash}${location.search}`,
			location
		);
	});

	return (
		<Routes>
			<Route path="/query/:pluginID?" component={Query} />
			<Route path="/settings/:pluginID?" component={Settings} />
			<Route path="/about" element={<div>I hate coffee.</div>} />
			<Route path="/" component={Home} />
		</Routes>
	);
}

export default App;
