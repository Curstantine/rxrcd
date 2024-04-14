/**
 * @typedef {import("svelte/store").Writable<import("$lib/types/deezer").Artist | null>} WritableArtist
 * @typedef {import("svelte/store").Writable<import("$lib/types/albums").DerivedAlbumList | null>} WritableDerivedAlbum
 */

import { invoke } from "@tauri-apps/api";
import { readonly, writable } from "svelte/store";

import { pushToSnackStack } from "$lib/components/snack/snack";
import { wait } from "$lib/utils/delayed";
import { transform_to_map } from "$lib/utils/transformer";

/** @type {import('./$types').LayoutLoad} */
export async function load({ params }) {
	const id = Number.parseInt(params.id);

	/** @type {WritableDerivedAlbum} */
	const albums = writable(null);

	get_artist_albums(id, albums);

	return {
		artist: get_artist_data(id),
		albums: readonly(albums),
	};
}

/**
 * @param {number} id
 * @returns {Promise<import("$lib/types/deezer").Artist>}
 */
async function get_artist_data(id) {
	const data = await invoke("get_artist", { id });
	return data;
}

/**
 * @param {number} id
 * @param {WritableDerivedAlbum} albums
 */
async function get_artist_albums(id, albums) {
	const limit = 50;

	/**
	 * @param index {number}
	 * @param limit {number}
	 *
	 * @returns {Promise<import("$lib/types/albums").DerivedAlbumList>}
	 */
	const fetch = async (index, limit) => {
		/** @type {import("$lib/types/deezer").ArtistAlbumList} */
		const { next, total, data } = await invoke("get_artist_albums", { artistId: id, index, limit });
		data.forEach((x) => transform_to_map(x, [["release_date", "LOCALE_DATE_STRING"]]));

		/** @type {import("$lib/types/albums").DerivedAlbumList["data"]} */
		const derive = { album: [], ep: [], single: [], compilation: [] };
		for (let i = 0; i < data.length; i++) {
			const album = data[i];
			derive[album.record_type].push(album);
		}

		return { next, total, data: derive };
	};

	const init_fetch = await fetch(0, limit);
	albums.set(init_fetch);
	if (init_fetch.next === null) return;

	const fetch_est = Math.floor(init_fetch.total / limit);
	const snack_session = pushToSnackStack({
		persistent: true,
		label: "Retrieving data",
		description: "Fetching missing artist data...",
	});

	let human_idx = 1;
	/** @type {URL | null} */
	let next_url = new URL(init_fetch.next);

	while (next_url !== null) {
		const index_param = next_url.searchParams.get("index");
		if (index_param === null) {
			throw new Error(`'index' search parameter is missing from url: ${next_url.toString()}`);
		}

		snack_session.update({ label: `Retrieving albums (${human_idx}/${fetch_est})` });
		await wait(2000);

		const { next, total, data } = await fetch(Number.parseInt(index_param), limit);
		next_url = next !== null ? new URL(next) : null;

		albums.update((result) => {
			const album = result?.data.album.concat(data.album) ?? data.album;
			const ep = result?.data.ep.concat(data.ep) ?? data.ep;
			const single = result?.data.single.concat(data.single) ?? data.single;
			const compilation = result?.data.compilation.concat(data.compilation) ?? data.compilation;

			return { next, total, data: { album, ep, single, compilation } };
		});

		human_idx++;
	}

	snack_session.close();
}
