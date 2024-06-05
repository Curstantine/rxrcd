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

/**
 * Changes the data language used in the deezer specific requests.
 *
 * Note: Use this along {@link config_set_account},
 * as this method does not mutate the configuration file.
 * @param {import("@/types/config").DataLanguage} language
 */
export async function change_data_language(language) {
	return await invoke("change_data_language", { language });
}
