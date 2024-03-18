import { writable } from "svelte/store";

/** @param {import("svelte/store").Readable<string>} id */
export function extort_data_state(id) {
	/** @type {import("@/types/deezer").Album} */
	const album = writable(null);
}
