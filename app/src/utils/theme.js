import { invoke } from "@tauri-apps/api";
import { derived, get, writable } from "svelte/store";

const media_sys_dark = matchMedia("(prefers-color-scheme: dark)");

/** @type {{ id: import("@/types/utils").Theme, label: string }[]} */
export const themes = [
	{ id: "light", label: "Light" },
	{ id: "dark", label: "Dark" },
	{ id: "system", label: "System" },
];

/** @type {import("@/types/utils").WritableTheme} */
const selected_theme_ = writable(null);
export const selected_theme = derived(selected_theme_, (val) => themes.find(({ id }) => id === val));

/** @param {boolean} is_system_dark */
function get_sys_pref_theme(is_system_dark = media_sys_dark.matches) {
	return is_system_dark ? "dark" : "light";
}

/** @param {MediaQueryListEvent} arg0 */
function on_sys_pref_change({ matches: is_system_dark }) {
	document.documentElement.classList.replace(
		get_sys_pref_theme(!is_system_dark),
		get_sys_pref_theme(is_system_dark),
	);
}

/** @param {import("@/types/utils").Theme} theme */
function set_theme_(theme) {
	const old_theme = get(selected_theme_);
	const apply = theme === "system" ? get_sys_pref_theme() : theme;
	const old_applied = old_theme === "system" ? get_sys_pref_theme() : old_theme;

	if (theme === "system" && old_theme !== "system") {
		media_sys_dark.addEventListener("change", on_sys_pref_change);
	}

	if (theme !== "system" && old_theme === "system") {
		media_sys_dark.removeEventListener("change", on_sys_pref_change);
	}

	if (old_theme === null) {
		document.documentElement.classList.add(apply);
	} else {
		document.documentElement.classList.replace(old_applied, apply);
	}

	selected_theme_.set(theme);
}

export async function initialize_theme() {
	/** @type {import("@/types/backend").ConfigurationAppearance}*/
	const config_appearance = await invoke("config_get_appearance");
	set_theme_(config_appearance.theme);
}

/** @param {import("@/types/utils").Theme} theme */
export async function set_theme(theme) {
	await invoke("config_set_theme", { theme });
	set_theme_(theme);
}
