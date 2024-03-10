<script>
	import { onDestroy, onMount } from "svelte";
	import { fade } from "svelte/transition";
	import { bounceIn, bounceOut } from "svelte/easing";

	import ResultListItemSet from "@/components/search_combo/ResultListItemSet.svelte";
	import ResultGridItemSet from "@/components/search_combo/ResultGridItemSet.svelte";

	/** @type {HTMLDivElement}*/
	let div;

	/** @type {import("@/types/search").SearchEntries} */
	export let data;

	/** @type {() => void}*/
	export let close;

	/**
	 *
	 * @param e {PointerEvent & { target: HTMLDivElement }}
	 */
	function external_hit_listener(e) {
		if (e.target.id === "search_input") return;
		if (!div.contains(e.target)) close.call(null);
	}

	onMount(() => document.addEventListener("click", external_hit_listener));
	onDestroy(() => document.removeEventListener("click", external_hit_listener));
</script>

<div
	bind:this={div}
	in:fade={{ duration: 500, easing: bounceIn }}
	out:fade={{ duration: 500, easing: bounceOut }}
	class="absolute inset-x-0 top-12 grid grid-cols-1 max-h-md overflow-y-auto border-(1 border solid) rounded bg-background p-2 shadow"
>
	<ResultListItemSet label="Artists" href="/artists?q={data.query}" child_href="/artist" data={data.artists} />
	<ResultGridItemSet label="Albums" href="/albums?q={data.query}" child_href="/album" data={data.albums} />
</div>
