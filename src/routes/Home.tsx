/*
	This component exists to redirect to the runner.
	The root path isn't used because plugin tabs are accessed using nested routes.
	/query/files
	/query/calculator
*/

import { useNavigate } from "@solidjs/router";

function Home() {
	useNavigate()("/query");

	return <>Maccha is initializing...</>;
}

export default Home;
