<script>
	import ListItem from "@/components/search_combo/ListItem.svelte";
	import ListItemSkeleton from "@/components/search_combo/ListItemSkeleton.svelte";

	/** @type {string} */
	export let label;

	/** @type {string} */
	export let href;

	/** @type {import("@/types/search").SearchEntryBaseResult | null} */
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
		<div class="flex flex-col pb-4">
			{#each data["data"] as item}
				<ListItem data={item} />
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
	<div class="flex flex-col pb-4">
		<ListItemSkeleton />
		<ListItemSkeleton />
		<ListItemSkeleton />
	</div>
{/if}
