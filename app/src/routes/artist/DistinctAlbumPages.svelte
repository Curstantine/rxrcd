<script>
	import { derived } from "svelte/store";

	import GridItem from "@/components/items/GridItem.svelte";
	import GridItemSkeleton from "@/components/items/GridItemSkeleton.svelte";

	/** @type {import("@/types/deezer").AlbumRecordType}*/
	export let type;

	/** @type {import("svelte/store").Readable<import("@/types/albums").DerivedAlbumList | null>} */
	export let data;

	const entries = derived(data, (val) => (!val?.data[type] ? null : val.data[type]));
</script>

<div class="flex flex-col px-4 pt-4">
	{#if $entries !== null && $entries.length > 0}
		<div class="grid-list">
			{#each $entries as { id, title, release_date, cover_medium }}
				<GridItem href="/album/{id}" {title} subtitle={release_date} image={cover_medium} />
			{/each}
		</div>
	{:else if $entries === null}
		<div class="grid-list">
			{#each new Array(Math.floor(Math.random() * 6) + 1) as _}
				<GridItemSkeleton />
			{/each}
		</div>
	{:else}
		<span class="mx-auto pt-4 text-6xl leading-loose">U_U</span>
		<span class="mx-auto text-muted-foreground">No matching content available...</span>
	{/if}
</div>

<style>
	.grid-list {
		--at-apply: grid gap-4 pt-3 pb-6 px-2;
		grid-template-columns: repeat(auto-fill, minmax(10.5rem, 1fr));
	}
</style>
