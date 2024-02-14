<script>
	import { fade } from "svelte/transition";
	import { bounceIn, bounceOut } from "svelte/easing";

	import ResultListItemSet from "@/components/search_combo/ResultListItemSet.svelte";
	import ResultGridItemSet from "@/components/search_combo/ResultGridItemSet.svelte";

	/** @type {import("@/types/search").SearchEntries} */
	export let data;
</script>

<div
	in:fade={{ duration: 300, easing: bounceIn }}
	out:fade={{ duration: 300, easing: bounceOut }}
	class="absolute inset-x-0 top-12 grid grid-cols-1 max-h-md overflow-y-auto border-(1 border solid) rounded bg-background p-2 shadow"
	on:blur
>
	<ResultListItemSet label="Artists" href="#/artists/" child_href="#/artist" data={data.artists} />
	<ResultGridItemSet label="Albums" href="#/albums/"   child_href="#/album" data={data.albums} />
</div>
