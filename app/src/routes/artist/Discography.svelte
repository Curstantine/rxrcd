<script>
	import { derived } from "svelte/store";

	import GridItem from "@/components/items/GridItem.svelte";
	import GridItemSkeleton from "@/components/items/GridItemSkeleton.svelte";

	/** @type {import("svelte/store").Readable<import("@/types/albums").DerivedAlbumList | null>} */
	export let data;

	const entries = derived(data, (val) => (val === null ? null : val["data"]));

	/** @type {{ id: string, key: import("@/types/deezer").AlbumRecordType, title: string }[]} */
	const subs = [
		{ id: "albums", key: "album", title: "Albums" },
		{ id: "eps", key: "ep", title: "EPs" },
		{ id: "singles", key: "single", title: "Singles" },
		{ id: "compilations", key: "compilation", title: "Compilations" },
	];
</script>

<div class="flex flex-col px-4 pt-4">
	{#each subs as sub}
		{#if $entries === null || $entries[sub.key].length > 0}
			<h2 id={sub.id} class="pl-2 font-medium">{sub.title}</h2>
		{/if}

		{#if $entries !== null && $entries[sub.key].length > 0}
			<div class="grid-list">
				{#each $entries[sub.key] as { id, title, release_date, cover_medium }}
					<GridItem href="/album/{id}" {title} subtitle={release_date} image={cover_medium} />
				{/each}
			</div>
		{:else if $entries === null}
			<div class="grid-list">
				{#each new Array(Math.floor(Math.random() * 6) + 1) as _}
					<GridItemSkeleton />
				{/each}
			</div>
		{/if}
	{/each}
</div>

<style>
	.grid-list {
		--at-apply: grid gap-4 pt-3 pb-6 px-2;
		grid-template-columns: repeat(auto-fill, minmax(10.5rem, 1fr));
	}
</style>
