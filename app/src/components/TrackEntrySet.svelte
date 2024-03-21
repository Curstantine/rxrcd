<script>
	import TrackItem from "@/components/items/TrackItem.svelte";
	import TrackItemSkeleton from "@/components/items/TrackItemSkeleton.svelte";

	/** @type {import("@/types/deezer").Track[] | null}*/
	export let tracks;
</script>

<div class="flex flex-col px-8 py-4">
	<div class="h-10 inline-flex items-center px-4 text-sm text-muted-foreground font-medium">
		<span class="w-10">#</span>
		<span class="max-w-lg flex-1">Title</span>
		<span class="flex-1">Featuring Artists</span>
		<span class="mr-8">Length</span>
	</div>

	{#if tracks !== null}
		{#each tracks as { id, title, artist: feat_artist }, i}
			<TrackItem {id} {title} {feat_artist} index={i + 1} length="3:36" />
		{/each}
	{:else}
		{#each new Array(12) as _}
			<TrackItemSkeleton />
		{/each}
	{/if}
</div>
