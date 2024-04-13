import { invoke } from "@tauri-apps/api";

export async function initialize_theme() {
	/** @type {import("$lib/types/backend").ConfigurationAppearance}*/
	const config_appearance = await invoke("config_get_appearance");

	if (config_appearance.theme === "system") {
		const system_is_dark = matchMedia("prefers-color-scheme: dark");
		document.documentElement.classList.add(system_is_dark ? "dark" : "light");
	} else {
		document.documentElement.classList.add(config_appearance.theme);
	}
}
