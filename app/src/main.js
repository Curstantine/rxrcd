import App from "./App.svelte";

import "./app.css";
import "virtual:uno.css";
import "@unocss/reset/tailwind-compat.css";

export default new App({
	target: document.getElementById("app"),
});
