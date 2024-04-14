<script>
	/** @type {string} */
	export let href;

	/** @type {string} */
	export let title;

	/** @type {string | null} */
	export let subtitle;
</script>

<a {href} class="{$$props.class} item">
	<span class="text-sm text-foreground">{title}</span>
	{#if subtitle !== null}
		<span class="text-xs text-muted-foreground">{subtitle}</span>
	{/if}
</a>

<style>
	.item {
		--at-apply: flex flex-col p-2 use-transition-standard transition-colors rounded;
	}

	.item:hover {
		--at-apply: bg-secondary;
	}
</style>
