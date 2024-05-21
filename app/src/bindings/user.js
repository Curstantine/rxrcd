import { invoke } from "@tauri-apps/api";

/**
 * @returns {Promise<import("@/types/user").User>}
 */
export async function refresh_login() {
	return await invoke("refresh_login");
}

/**
 * @param {string} arl
 * @returns {Promise<import("@/types/user").User>}
 */
export async function login_with_arl(arl) {
	return await invoke("login_with_arl", { arl });
}

export async function logout() {
	return await invoke("logout");
}
