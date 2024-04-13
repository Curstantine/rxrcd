<script>
	import EmptyListItem from "$lib/components/items/EmptyListItem.svelte";
	import ListItem from "$lib/components/items/ListItem.svelte";
	import ListItemSkeleton from "$lib/components/items/ListItemSkeleton.svelte";
	import ResultTitle from "$lib/components/search_combo/ResultTitle.svelte";

	/** @type {string} */
	export let label;

	/** @type {string} */
	export let href;

	/** @type {string} */
	export let child_href;

	/** @type {import("$lib/types/search").SearchEntryBaseResult | null} */
	export let data;
</script>

{#if data !== null}
	<ResultTitle {href} {label} disabled={data?.data?.length === 0} />

	<!-- TODO: Handle errors -->
	{#if !!data.data && data.data.length > 0}
		<div class="flex flex-col pb-4 transition-opacity use-transition-standard" class:replacing={data.replacing}>
			{#each data["data"] as item}
				<ListItem {...item} href="{child_href}/{item.id}" />
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
	<div class="flex flex-col pb-4">
		<ListItemSkeleton />
		<ListItemSkeleton />
		<ListItemSkeleton />
	</div>
{/if}

<style>
	.replacing {
		--at-apply: pointer-events-none opacity-50;
	}
</style>
