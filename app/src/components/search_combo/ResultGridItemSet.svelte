<script>
	import EmptyListItem from "@/components/items/EmptyListItem.svelte";
	import GridItem from "@/components/items/GridItem.svelte";
	import GridItemSkeleton from "@/components/items/GridItemSkeleton.svelte";
	import ResultTitle from "@/components/search_combo/ResultTitle.svelte";

	/** @type {string} */
	export let label;

	/** @type {string} */
	export let href;

	/** @type {string} */
	export let child_href;

	/** @type {import("@/types/search").SearchEntryIEResult | null} */
	export let data;
</script>

{#if data !== null}
	<ResultTitle {href} {label} disabled={data?.data?.length === 0} />

	{#if data.data?.length > 0}
		<div class="grid-list" class:replacing={data.replacing}>
			{#each data["data"] as item}
				<GridItem {...item} small href="{child_href}/{item.id}" />
			{/each}
		</div>
	{:else}
		<EmptyListItem />
	{/if}
{:else}
	<div
		class="h-8 flex items-center justify-between px-2 text-muted-foreground transition-colors use-transition-emphasized"
	>
		<div class="h-2 w-12 animate-pulse rounded bg-secondary" />
		<div class="h-3 w-3 rounded-full bg-secondary" />
	</div>
	<div class="grid-list">
		<GridItemSkeleton small />
		<GridItemSkeleton small />
		<GridItemSkeleton small />
	</div>
{/if}

<style>
	.grid-list {
		--at-apply: grid gap-4 pb-4 pt-2 px-2 transition-opacity use-transition-standard;
		grid-template-columns: repeat(auto-fill, minmax(0, 9rem));
	}

	.replacing {
		--at-apply: pointer-events-none opacity-50;
	}
</style>
