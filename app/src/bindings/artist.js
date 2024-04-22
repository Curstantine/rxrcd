import { invoke } from "@tauri-apps/api";

/**
 * @param {number} id
 * @returns {Promise<import("@/types/deezer").Artist>}
 */
export async function get_artist(id) {
	return await invoke("get_artist", { id });
}

/**
 * @param {string} query
 * @param {number} [limit]
 * @param {number} [index]
 * @returns {Promise<import("@/types/deezer").ArtistSearch>}
 */
export async function search_artists(query, limit, index) {
	return await invoke("search_artists", { query, limit, index });
}
