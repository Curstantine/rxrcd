import { invoke } from "@tauri-apps/api";

/**
 * @returns {Promise<import("@/types/commands").SetupReturnFlags>}
 */
export async function setup() {
	return await invoke("setup");
}
