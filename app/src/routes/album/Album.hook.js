import { invoke } from "@tauri-apps/api";
import { get, readonly, writable } from "svelte/store";

/** @param {import("svelte/store").Readable<string>} id */
export function extort_data_state(id) {
	/** @type {import("svelte/store").Writable<import("@/types/deezer").Album>} */
	const album = writable(null);

	/** @param {number} id */
	async function get_track_data(id) {
		/** @type {import("@/types/deezer").Album} */
		const result = await invoke("get_album", { id });

		album.set(result);
	}

	id.subscribe((val) => {
		const id = Number.parseInt(val);
		if (get(album) !== null) album.set(null);

		get_track_data(id);
	});

	return { album: readonly(album) };
}
