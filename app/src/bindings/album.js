import { invoke } from "@tauri-apps/api";

/**
 * @param {number} id
 * @returns {Promise<import("@/types/deezer").Album>}
 */
export async function get_album(id) {
	return await invoke("get_album", { id });
}

/**
 * @param {string} query
 * @param {number} [limit]
 * @param {number} [index]
 * @returns {Promise<import("@/types/deezer").AlbumSearch>}
 */
export async function search_albums(query, limit, index) {
	return await invoke("search_albums", { query, limit, index });
}

/**
 * @param {number} artist_id
 * @param {number} [limit]
 * @param {number} [index]
 * @returns {Promise<import("@/types/deezer").ArtistAlbumList>}
 */
export async function get_artist_albums(artist_id, limit, index) {
	return await invoke("get_artist_albums", { artist_id, limit, index });
}
