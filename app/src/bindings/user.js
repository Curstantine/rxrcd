import { invoke } from "@tauri-apps/api";

/**
 * @returns {Promise<import("@/types/user").User>}
 */
export async function refresh_login() {
	return await invoke("refresh_login");
}

/**
 * @template {"arl" | "credentials"} T
 * @param {import("@/types/user").UserAuthType<T>} auth_type
 * @returns {Promise<import("@/types/user").User>}
 */
export async function login(auth_type) {
	return await invoke("login", auth_type);
}

export async function logout() {
	return await invoke("logout");
}
