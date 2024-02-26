import { invoke } from "@tauri-apps/api";
import { get, readonly, writable } from "svelte/store";

import { pushToSnackStack } from "@/components/snack/snack";
import Discography from "@/routes/artist/Discography.svelte";
import { wait } from "@/utils/delayed";

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
		const limit = 50;

		/**
		 * @param index {number}
		 * @param limit {number}
		 *
		 * @returns {Promise<import("@/types/albums").DerivedAlbumList>}
		 */
		const fetch = async (index, limit) => {
			/** @type {import("@/types/deezer").ArtistAlbumList} */
			const { next, total, data } = await invoke("get_artist_albums", { artistId: id, index, limit });

			/** @type {import("@/types/albums").DerivedAlbumList["data"]} */
			const derive = { album: [], ep: [], single: [], compilation: [] };
			for (let i = 0; i < data.length; i++) {
				const album = data[i];
				derive[album.record_type].push(album);
			}

			return { next, total, data: derive };
		};

		const init_fetch = await fetch(0, limit);
		albums.set(init_fetch);

		const fetch_est = Math.floor(init_fetch.total / limit);
		const snack_session = pushToSnackStack({
			persistent: true,
			label: "Retrieving data",
			description: "Fetching missing artist data...",
		});

		let index = 0;
		let human_idx = 1;
		let next_url = new URL(init_fetch.next);

		while (next_url !== null) {
			index = Number.parseInt(next_url.searchParams.get("index"));

			await wait(2000);
			snack_session.update({ label: `Retrieving albums (${human_idx}/${fetch_est})` });

			const { next, total, data } = await fetch(index, limit);
			next_url = next !== null ? new URL(next) : null;

			albums.update(
				({ data: { album: old_albums, compilation: old_compilations, ep: old_eps, single: old_single } }) => ({
					next,
					total,
					data: {
						album: old_albums.concat(data.album),
						ep: old_eps.concat(data.ep),
						single: old_single.concat(data.single),
						compilation: old_compilations.concat(data.compilation),
					},
				}),
			);

			human_idx++;
		}

		snack_session.close();
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
