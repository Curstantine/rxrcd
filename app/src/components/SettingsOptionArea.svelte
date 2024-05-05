<script>
	/** @type {string} */
	export let form_id;

	/** @type {string} */
	export let label;

	/** @type {string | string[]} */
	export let subtitle;

	/** @type {"col" | "row"}*/
	export let layout = "row";
</script>

<div class="{$$props.class} {layout} option_area">
	<label for={form_id} class="grid-area-[header]">{label}</label>
	<label for={form_id} class="grid-area-[description] pr-2 text-sm text-muted-foreground">
		{#if typeof subtitle === "string"}
			{subtitle}
		{:else}
			{#each subtitle as line}
				<p>{line}</p>
			{/each}
		{/if}
	</label>

	<div class="option">
		<slot />
	</div>
</div>

<style>
	.option_area {
		--at-apply: grid py-2 items-center;
	}

	.option {
		--at-apply: grid-area-[option];
	}

	.option_area.row {
		--at-apply: grid-cols-[1fr_1fr_auto];
		grid-template-areas:
			"header header option"
			"description description option";
	}

	.option_area.col {
		--at-apply: grid-cols-[1fr_1fr];
		grid-template-areas:
			"header header header"
			"description description description"
			"option option option";
	}

	.option_area.col .option {
		--at-apply: mt-2;
	}
</style>
