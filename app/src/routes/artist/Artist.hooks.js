import { get, readonly, writable } from "svelte/store";

import { get_artist_albums } from "@/bindings/album";
import { get_artist } from "@/bindings/artist";
import { wait } from "@/utils/delayed";
import { fetch_error } from "@/utils/errors";
import { transform_to_map } from "@/utils/transformer";

import { pushToSnackStack } from "@/components/snack/snack";

/**
 * @typedef {{ id: string, label: string, rel_type: import("@/types/deezer").AlbumRecordType | "discography" }} TabItem
 * @type {TabItem[]}
 */
export const tabs = [
	{ id: "tab_discography", rel_type: "discography", label: "Discography" },
	{ id: "tab_albums", rel_type: "album", label: "Albums" },
	{ id: "tab_eps", rel_type: "ep", label: "EPs" },
	{ id: "tab_singles", rel_type: "single", label: "Singles" },
	{ id: "tab_compilations", rel_type: "compilation", label: "Compilations" },
];

export function extort_tab_state() {
	const active_tab = writable(tabs[0]);

	/** @param e {MouseEvent & { currentTarget: HTMLButtonElement }} */
	function on_tab_click(e) {
		const new_tab = tabs.find(({ id }) => e.currentTarget.id === id);
		if (new_tab === undefined) {
			throw new Error(`Targeted tab '${e.currentTarget.id}' does not exist in the tabs declaration`);
		}

		active_tab.set(new_tab);
	}

	return { active_tab: readonly(active_tab), on_tab_click };
}

/**
 *  @param {HTMLDivElement} node
 *  @param {import("svelte/store").Readable<TabItem>} active_tab
 */
export function style_tab_decoration(node, active_tab) {
	/** @param {HTMLElement} target */
	const styler = (target) => {
		node.style.width = `${target.offsetWidth}px`;
		node.style.transform = `translateX(${target.offsetLeft}px)`;
	};

	const update_styles = () => {
		const id = get(active_tab).id;
		const element = document.getElementById(id);
		if (element === null) {
			throw new Error(`Failed to find an element with id '${id}' to update the tab decoration`);
		}

		styler(element);
	};

	const un_sub = active_tab.subscribe(update_styles);

	return {
		destroy: () => un_sub(),
	};
}

/**
 *  @param {HTMLDivElement} node
 */
export function style_tab_bar(node) {
	const app = document.getElementById("app");
	if (app === null) throw new Error("#app element is not mounted");

	let marked = false;
	const styler = () => {
		if (!marked && app.scrollTop >= node.offsetTop) {
			node.style.setProperty("--un-bg-color-opacity", "0.95");
			return marked = true;
		}

		if (marked && node.offsetTop > app.scrollTop) {
			node.style.setProperty("--un-bg-color-opacity", "1");
			return marked = false;
		}
	};

	app.addEventListener("scroll", styler);

	return {
		destroy: () => app.removeEventListener("scroll", styler),
	};
}

/** @param {import("svelte/store").Readable<string>} id */
export function extort_data_state(id) {
	/** @type {import("svelte/store").Writable<import("@/types/deezer").Artist | null>} */
	const artist = writable(null);

	/** @type {import("svelte/store").Writable<import("@/types/albums").DerivedAlbumList | null>} */
	const albums = writable(null);

	/** @param {number} id */
	async function update_artist_data(id) {
		const data = await get_artist(id);
		artist.set(data);
	}

	/** @param {number} id */
	async function update_artist_albums(id) {
		const limit = 50;

		/**
		 * @param index {number}
		 * @param limit {number}
		 *
		 * @returns {Promise<import("@/types/albums").DerivedAlbumList>}
		 */
		const fetch = async (index, limit) => {
			/** @type {import("@/types/deezer").ArtistAlbumList} */
			const { next, total, data } = await get_artist_albums(id, limit, index);
			data.forEach((x) => transform_to_map(x, [["release_date", "LOCALE_DATE_STRING"]]));

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
		if (init_fetch.next === null) return;

		const fetch_est = Math.floor(init_fetch.total / limit);
		const snack_session = pushToSnackStack({
			persistent: true,
			label: "Retrieving data",
			description: "Fetching missing artist data...",
		});

		let index = 0;
		let human_idx = 1;

		/** @type {URL | null} */
		let next_url = new URL(init_fetch.next);

		while (next_url !== null) {
			const index_str = next_url.searchParams.get("index");
			if (index_str === null) {
				fetch_error("Fetch iteration started without a valid index in the search parameters", next_url);
			}

			await wait(2000);
			index = Number.parseInt(index_str);
			snack_session.update({ label: `Retrieving albums (${human_idx}/${fetch_est})` });

			const { next, total, data } = await fetch(index, limit);
			next_url = next !== null ? new URL(next) : null;

			albums.update(
				(old_data) => {
					if (old_data === null) fetch_error("old_data was null in an fetch update cycle", next_url);

					const {
						data: { album: old_albums, compilation: old_compilations, ep: old_eps, single: old_single },
					} = old_data;

					return {
						next,
						total,
						data: {
							album: old_albums.concat(data.album),
							ep: old_eps.concat(data.ep),
							single: old_single.concat(data.single),
							compilation: old_compilations.concat(data.compilation),
						},
					};
				},
			);

			human_idx++;
		}

		snack_session.close();
	}

	id.subscribe((val) => {
		const id = Number.parseInt(val);
		if (get(artist) !== null) artist.set(null);

		Promise.all([
			update_artist_data(id),
			update_artist_albums(id),
		]);
	});

	return {
		artist: readonly(artist),
		albums: readonly(albums),
	};
}
