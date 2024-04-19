<script>
	import { fade } from "svelte/transition";
	import { bounceIn, bounceOut } from "svelte/easing";

	import { external_hit } from "@/utils/actions";

	import ResultListItemSet from "@/components/search_combo/ResultListItemSet.svelte";
	import ResultGridItemSet from "@/components/search_combo/ResultGridItemSet.svelte";

	/** @type {string[]} ID to ignore when input goes out of focus, usually an input field that initiates the search combo */
	export let coupling_ids;

	/** @type {import("@/types/search").SearchEntries} */
	export let data;

	/** @type {() => void}*/
	export let close;
</script>

<div
	use:external_hit={{ close, coupling_ids }}
	in:fade={{ duration: 500, easing: bounceIn }}
	out:fade={{ duration: 500, easing: bounceOut }}
	class="absolute inset-x-0 top-12 grid grid-cols-1 max-h-md overflow-y-auto border-(1 border solid) rounded bg-background p-2 shadow"
>
	<ResultListItemSet label="Artists" href="/artists?q={data.query}" child_href="/artist" data={data.artists} />
	<ResultGridItemSet label="Albums" href="/albums?q={data.query}" child_href="/album" data={data.albums} />
</div>
