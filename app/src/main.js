import App from "./App.svelte";

import "./app.css";
import "virtual:uno.css";
import "@unocss/reset/tailwind-compat.css";

export default new App({
	// @ts-expect-error We already know app won't be null
	target: document.getElementById("app"),
});
