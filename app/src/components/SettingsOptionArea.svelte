<script>
	/** @type {string} */
	export let label;

	/** @type {string | string[]} */
	export let subtitle;
</script>

<div class="{$$props.class} option_area">
	<span class="grid-area-[header]">{label}</span>
	<span class="grid-area-[description] text-sm text-muted-foreground">
		{#if typeof subtitle === "string"}
			{subtitle}
		{:else}
			{#each subtitle as line}
				<p>{line}</p>
			{/each}
		{/if}
	</span>

	<div class="grid-area-[option]">
		<slot />
	</div>
</div>

<style>
	.option_area {
		--at-apply: grid grid-cols-[1fr_1fr_auto] py-2 items-center;
		grid-template-areas:
			"header header option"
			"description description option";
	}
</style>
