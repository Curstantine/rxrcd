import { invoke } from "@tauri-apps/api";

/**
 * @returns {Promise<void>}
 */
export async function setup() {
	return await invoke("setup");
}
