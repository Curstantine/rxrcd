import { invoke } from "@tauri-apps/api";

/**
 * @returns {Promise<import("@/types/config").ConfigurationAppearance>}
 */
export async function config_get_appearance() {
	return await invoke("config_get_appearance");
}

/**
 * @param {import("@/types/config").ConfigurationAppearance} appearance
 * @returns {Promise<void>}
 */
export async function config_set_appearance(appearance) {
	return await invoke("config_set_appearance", { appearance });
}

/**
 * @returns {Promise<import("@/types/config").ConfigurationDownload>}
 */
export async function config_get_download() {
	return await invoke("config_get_download");
}

/**
 * @param {import("@/types/config").ConfigurationDownload} download
 * @returns {Promise<void>}
 */
export async function config_set_download(download) {
	return await invoke("config_set_download", { download });
}

/**
 * @returns {Promise<void>}
 */
export async function config_reload() {
	return await invoke("config_reload");
}
