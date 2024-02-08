import debounce from "@/utils/debounce";
import { onDestroy, tick } from "svelte";
import { location, pop } from "svelte-spa-router";
import { derived, get, readonly, writable } from "svelte/store";

/**
 * @typedef {import("svelte/store").Readable<boolean>} ReadableBool
 * @typedef {() => void} VoidFunction
 *
 * @returns {[[ReadableBool, ReadableBool], { back: VoidFunction, forward: VoidFunction }]} [[back_disabled, forward_disabled], { back, forward }]
 */
export function extort_nav_state() {
	/** @type {string[]} */
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
	const un_sub = location.subscribe((url) => {
		const index = get(current_index);

		// We don't want to mutate history_stack if the action comes from a internal back/forth change.
		if (internal) {
			internal = false;
			return;
		}

		// We don't want to mutate current_index in the initial render, so the "/" path won't start back/forth.
		if (stack.length === 0) {
			stack.push(url);
			return;
		}

		// We can check if this is "backed" before or not by comparing the current index and last index of the history stack.
		// The index that gets replaced should be "next forward" (So the forward tree resets)
		if (index !== stack.length - 1) {
			const current_pos = index + 1;

			stack.splice(current_pos, stack.length - current_pos, url);
			current_index.set(stack.length - 1);
		} else {
			stack.push(url);
			current_index.update((index) => index + 1);
		}
	});

	const back = async () => {
		internal = true;
		current_index.update((index) => index - 1);
		await pop();
	};

	const forward = async () => {
		internal = true;
		current_index.update((index) => index + 1);

		await tick();
		window.history.forward();
	};

	onDestroy(un_sub);

	return [[back_disabled, forward_disabled], { back, forward }];
}

/**
 * @typedef {import("svelte/store").Writable<string>} WritableString
 * @typedef {import("svelte/store").Writable<import("@/types/search").SearchEntries>} WritableSearchEntries
 * @typedef {import("svelte/store").Readable<import("@/types/search").SearchEntries>} ReadableSearchEntries
 *
 * @returns {[[ReadableBool, ReadableBool, WritableString], ReadableSearchEntries]}
 */
export function extort_search_state() {
	const search = writable("");
	const loading = writable(true);
	const show = writable(false);

	/** @type {WritableSearchEntries} */
	const entries = writable([]);

	/** @type {{ run: (arg0: string) => void, clear: () => void }} */
	const { run: fetcher, clear: clearFetcher } = debounce(async (query) => {
		entries.set([
			[{ title: "Artists", href: `#/search/artists?q=${query}`, list_type: "list" }, [
				{ title: "Daft Punk", href: "/aoe" },
				{ title: "Blume Popo", href: "/aoe" },
				{ title: "High Velocity", href: "/aoe" },
			]],
			[{ title: "Albums", href: `#/search/albums?q=${query}`, list_type: "grid" }, [
				{ title: "SILENT PLANET: RELOADED", subtitle: "TeddyLoid", href: "/aoe" },
				{ title: "SILENT PLANET", subtitle: "TeddyLoid", href: "/aoe" },
				{ title: "SILENT PLANET 2", subtitle: "TeddyLoid feat. IA", href: "/aoe" },
			]],
		]);

		loading.set(false);
	});

	const un_sub = search.subscribe((str) => {
		if (get(show) && str.length === 0) show.set(false);
		if (!get(show) && str.length >= 3) {
			// We only want the animation if this is the first time searching since the modal opened
			loading.set(true);
			show.set(true);
		}

		if (str.length >= 3) {
			fetcher.call(undefined, str);
		}

		return () => {
			clearFetcher();
		};
	});

	onDestroy(un_sub);

	return [[readonly(show), readonly(loading), search], readonly(entries)];
}
