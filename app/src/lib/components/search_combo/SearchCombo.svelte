<script>
	import { fade } from "svelte/transition";
	import { bounceIn, bounceOut } from "svelte/easing";

	import ResultListItemSet from "$lib/components/search_combo/ResultListItemSet.svelte";
	import ResultGridItemSet from "$lib/components/search_combo/ResultGridItemSet.svelte";

	/** @type {string} ID to ignore when input goes out of focus, usually an input field that initiates the search combo */
	export let coupling_id;

	/** @type {import("$lib/types/search").SearchEntries} */
	export let data;

	/** @type {() => void}*/
	export let close;

	/** @type {import("svelte/action").Action} */
	function extern_hit(node) {
		/** @param e {PointerEvent & { target: HTMLDivElement }} */
		function listener(e) {
			if (e.target.id === coupling_id) return;
			if (!node.contains(e.target)) close.call(null);
		}

		document.addEventListener("click", listener);

		return {
			destroy: () => document.removeEventListener("click", listener),
		};
	}
</script>

<div
	use:extern_hit
	in:fade={{ duration: 500, easing: bounceIn }}
	out:fade={{ duration: 500, easing: bounceOut }}
	class="absolute inset-x-0 top-12 grid grid-cols-1 max-h-md overflow-y-auto border-(1 border solid) rounded bg-background p-2 shadow"
>
	<ResultListItemSet label="Artists" href="/artists?q={data.query}" child_href="/artist" data={data.artists} />
	<ResultGridItemSet label="Albums" href="/albums?q={data.query}" child_href="/album" data={data.albums} />
</div>
