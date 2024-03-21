<script>
	import { link } from "svelte-spa-router";

	/** @type {number} */
	export let id;

	/** @type {number} */
	export let index;

	/** @type {string} */
	export let title;

	/** @type {import("@/types/deezer").TrackRelArtist | null} */
	export let feat_artist;

	/** @type {string} */
	export let length;

	$: artist_url = feat_artist !== null ? `/artist/${feat_artist.id}` : "javascript:void(0)";

	const onMoreClick = () => id;
</script>

<div class="entry">
	<span class="w-10">{index}.</span>
	<span class="max-w-lg flex-1 text-foreground">{title}</span>
	<a use:link href={artist_url} class="flex-1 hover:underline">
		{feat_artist?.name ?? "-"}
	</a>
	<span>{length}</span>
	<button class="ml-6 w-8 icon-button-layout" on:click={onMoreClick}>
		<div class="i-symbols-more-horiz h-4 w-4" />
	</button>
</div>

<style>
	.entry {
		--at-apply: h-10 inline-flex items-center rounded-md pl-4 pr-2 text-sm text-muted-foreground transition-colors
			use-transition-standard;
	}

	.entry:hover {
		--at-apply: bg-muted/50;
	}
</style>
