import { get, readonly, writable } from "svelte/store";

import Discography from "@/routes/artist/Discography.svelte";
import { invoke } from "@tauri-apps/api";

/**
 * @typedef {{ id: string, label: string, component: import("svelte").ComponentType | null }} TabItem
 * @type {TabItem[]}
 */
export const tabs = [
	{ id: "tab_discography", label: "Discography", component: Discography },
	{ id: "tab_albums", label: "Albums", component: null },
	{ id: "tab_eps", label: "EPs", component: null },
	{ id: "tab_singles", label: "Singles", component: null },
	{ id: "tab_compilations", label: "Compilations", component: null },
];

export function extort_tab_state() {
	const active_tab = writable(tabs[0]);

	/** @param e {MouseEvent & { currentTarget: HTMLButtonElement }} */
	function on_tab_click(e) {
		const new_tab = tabs.find(({ id }) => e.currentTarget.id === id);
		active_tab.set(new_tab);
	}

	return { active_tab: readonly(active_tab), on_tab_click };
}

/**
 *  @param {HTMLDivElement} node
 *  @param {import("svelte/store").Readable<TabItem>} active_tab
 */
export function style_tab(node, active_tab) {
	/** @param {HTMLElement} target */
	const styler = (target) => {
		node.style.width = `${target.offsetWidth}px`;
		node.style.transform = `translateX(${target.offsetLeft}px)`;
	};

	const update_styles = () => {
		const id = get(active_tab).id;
		styler(document.getElementById(id));
	};

	const un_sub = active_tab.subscribe(update_styles);

	return {
		destroy: () => un_sub(),
	};
}

/** @param {import("svelte/store").Readable<string>} id */
export function extort_data_state(id) {
	/** @type {import("svelte/store").Writable<import("@/types/deezer").Artist | null>} */
	const artist = writable(null);

	/** @type {import("svelte/store").Writable<import("@/types/albums").DerivedAlbumList | null>} */
	const albums = writable(null);

	/** @param {number} id */
	async function get_artist_data(id) {
		const data = await invoke("get_artist", { id });
		artist.set(data);
	}

	/** @param {number} id */
	async function get_artist_albums(id) {
		/** @type {import("@/types/deezer").ArtistAlbumList} */
		const { next, total, data } = await invoke("get_artist_albums", { artistId: id });

		/** @type {import("@/types/albums").DerivedAlbumList["data"]} */
		const def = { album: [], ep: [], single: [], compilation: [] };
		for (let i = 0; i < data.length; i++) {
			const album = data[i];
			def[album.record_type].push(album);
		}

		albums.set({ next, total, data: def });
	}

	id.subscribe((val) => {
		const id = Number.parseInt(val);
		if (get(artist) !== null) artist.set(null);

		Promise.all([
			get_artist_data(id),
			get_artist_albums(id),
		]);
	});

	return {
		artist: readonly(artist),
		albums: readonly(albums),
	};
}
