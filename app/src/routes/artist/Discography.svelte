<script>
	import GridItem from "@/components/items/GridItem.svelte";
</script>

<div class="flex flex-col px-4 pt-4">
	<h2 class="pl-2 font-medium">Albums</h2>
	<div class="grid-list">
		{#each new Array(6) as _}
			<GridItem title="Test subject 2.0" subtitle="By testing" href="/" image={null} />
		{/each}
	</div>
</div>

<style>
	.grid-list {
		--at-apply: grid gap-y-2 pt-1;
		grid-template-columns: repeat(auto-fill, minmax(9rem, 1fr));
	}
</style>
