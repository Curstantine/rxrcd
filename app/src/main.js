import "./app.css";
import App from "./App.svelte";

import "virtual:uno.css";
import "@unocss/reset/tailwind.css";

export default new App({
	target: document.getElementById("app"),
});
