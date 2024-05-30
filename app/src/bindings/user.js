import { invoke } from "@tauri-apps/api";

/**
 * @returns {Promise<import("@/types/user").UserAuthState>}
 */
export async function get_auth_state() {
	return await invoke("get_auth_state");
}

/**
 * @returns {Promise<import("@/types/user").User>}
 */
export async function refresh_login() {
	return await invoke("refresh_login");
}

/**
 * @param {import("@/types/user").UserAuthType} data
 * @returns {Promise<import("@/types/user").User>}
 */
export async function login(data) {
	return await invoke("login", { data });
}

export async function logout() {
	return await invoke("logout");
}
