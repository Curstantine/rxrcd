import { invoke } from "@tauri-apps/api";
import { onDestroy, tick } from "svelte";
import { derived, get, readonly, writable } from "svelte/store";

import { onNavigate } from "$app/navigation";

import { debounce } from "$lib/utils/delayed";

/**
 * @typedef {import("svelte/store").Readable<boolean>} ReadableBool
 * @typedef {() => void} VoidFunction
 *
 * @returns {[[ReadableBool, ReadableBool], { back: VoidFunction, forward: VoidFunction }]} [[back_disabled, forward_disabled], { back, forward }]
 */
export function extort_nav_state() {
	/** @type {URL[]} */
	const stack = [];
	let internal = false;

	const current_index = writable(0);

	const back_disabled = derived(current_index, (idx) => idx === 0);
	const forward_disabled = derived(current_index, (idx) => idx === stack.length - 1);

	/**
	 * We need to check if the url change is from a back/forward action.
	 *
	 * If it is, we ignore since we are just traversing the stack.
	 * If not, we need to check if the user has pressed "back" before and remove the future tree from that index accordingly.
	 */
	onNavigate(({ to }) => {
		const index = get(current_index);

		if (to === null) throw Error("onNavigation 'to' was null.");

		// We don't want to mutate history_stack if the action comes from a internal back/forth change.
		if (internal) {
			internal = false;
			return;
		}

		// We don't want to mutate current_index in the initial render, so the "/" path won't start back/forth.
		if (stack.length === 0) {
			stack.push(to.url);
			return;
		}

		// We can check if this is "backed" before or not by comparing the current index and last index of the history stack.
		// The index that gets replaced should be "next forward" (So the forward tree resets)
		if (index !== stack.length - 1) {
			const current_pos = index + 1;

			stack.splice(current_pos, stack.length - current_pos, to.url);
			current_index.set(stack.length - 1);
		} else {
			stack.push(to.url);
			current_index.update((index) => index + 1);
		}
	});

	const back = async () => {
		internal = true;
		current_index.update((index) => index - 1);
		window.history.back();
	};

	const forward = async () => {
		internal = true;
		current_index.update((index) => index + 1);

		await tick();
		window.history.forward();
	};

	return [[back_disabled, forward_disabled], { back, forward }];
}

/**
 * @typedef {import("svelte/store").Writable<string>} WritableString
 * @typedef {import("svelte/store").Writable<import("$lib/types/search").SearchEntries>} WritableSearchEntries
 * @typedef {import("svelte/store").Readable<import("$lib/types/search").SearchEntries>} ReadableSearchEntries
 *
 * @returns {[[ReadableBool, WritableString], ReadableSearchEntries, { close: () => void }]}
 */
export function extort_search_state() {
	const search = writable("");
	const show = writable(false);

	/** @type {WritableSearchEntries} */
	const entries = writable({ albums: null, query: "", artists: null });

	const startReplacing = () => {
		entries.update(({ query, artists, albums }) => ({
			query,
			artists: artists !== null ? { ...artists, replacing: true } : null,
			albums: albums !== null ? { ...albums, replacing: true } : null,
		}));
	};

	/** @type {{ run: (arg0: string) => void, clear: () => void }} */
	const { run: fetcher, clear: clearFetcher } = debounce(async (query) => {
		try {
			/** @type {import("$lib/types/deezer").AlbumSearch} */
			const albums = await invoke("search_albums", { query });

			/** @type {import("$lib/types/search").SearchEntryIE[]}*/
			const data = albums.data.map(({ id, title, artist: { name }, cover_big }) => ({
				id,
				title,
				subtitle: name,
				image: cover_big,
			}));

			entries.update(({ artists }) => {
				return { artists, query, albums: { data, replacing: false, error: null } };
			});
		} catch (e) {
			entries.update(({ artists }) => {
				return {
					artists,
					query,
					albums: {
						// @ts-ignore
						error: e.toString(),
						replacing: false,
						data: null,
					},
				};
			});
		}

		try {
			/** @type {import("$lib/types/deezer").ArtistSearch} */
			const artists = await invoke("search_artists", { query });

			/** @type {import("$lib/types/search").SearchEntryBase[]}*/
			const data = artists.data.map(({ id, name }) => ({ id, title: name, subtitle: null }));

			entries.update(({ albums }) => {
				return { albums, query, artists: { data, replacing: false, error: null } };
			});
		} catch (e) {
			entries.update(({ albums }) => {
				// @ts-ignore
				return { albums, query, artists: { error: e.toString(), replacing: false, data: null } };
			});
		}
	});

	const search_un_sub = search.subscribe((str) => {
		if (get(show) && str.length === 0) show.set(false);
		if (!get(show) && str.length >= 3) {
			show.set(true);
			entries.set({ albums: null, query: str, artists: null });
			clearFetcher.call(undefined);
		}

		if (str.length >= 3) {
			startReplacing();
			fetcher.call(undefined, str);
		}

		return () => {
			clearFetcher.call(undefined);
		};
	});

	const close = () => {
		show.set(false);
	};

	onDestroy(() => {
		search_un_sub();
	});

	return [[readonly(show), search], readonly(entries), { close }];
}
