<script>
	import GridItem from "@/components/items/GridItem.svelte";
	import GridItemSkeleton from "@/components/items/GridItemSkeleton.svelte";

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
	<a
		{href}
		class="h-8 flex items-center justify-between px-2 text-muted-foreground transition-colors use-transition-standard hover:text-primary"
	>
		<span class="text-sm">{label}</span>
		<div class="i-symbols-chevron-right h-5 w-5" />
	</a>

	{#if data.data !== null}
		<div class="grid-list" class:replacing={data.replacing}>
			{#each data["data"] as item}
				<GridItem {...item} href="{child_href}/{item.id}" />
			{/each}
		</div>
	{/if}
{:else}
	<div
		class="h-8 flex items-center justify-between px-2 text-muted-foreground transition-colors use-transition-emphasized"
	>
		<div class="h-2 w-12 animate-pulse rounded bg-secondary" />
		<div class="h-3 w-3 rounded-full bg-secondary" />
	</div>
	<div class="grid-list">
		<GridItemSkeleton />
		<GridItemSkeleton />
		<GridItemSkeleton />
	</div>
{/if}

<style>
	.grid-list {
		--at-apply: grid gap-y-2 pb-4;
		grid-template-columns: repeat(auto-fill, minmax(0, 9rem));
	}

	.replacing {
		--at-apply: pointer-events-none opacity-50;
	}
</style>
