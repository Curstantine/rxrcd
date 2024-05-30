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
 * @param {import("@/types/user").UserAuthType} auth_type
 * @returns {Promise<import("@/types/user").User>}
 */
export async function login(auth_type) {
	return await invoke("login", auth_type);
}

export async function logout() {
	return await invoke("logout");
}
