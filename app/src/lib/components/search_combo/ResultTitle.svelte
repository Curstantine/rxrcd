<script>
	import { link } from "svelte-spa-router";

	/** @type {string} */
	export let label;

	/** @type {string} */
	export let href;

	/** @type {boolean} */
	export let disabled;
</script>

<a
	{href}
	use:link
	aria-disabled={disabled}
	class:disabled
	class="h-8 flex items-center justify-between px-2 text-muted-foreground transition-colors use-transition-standard hover:text-primary"
>
	<span class="text-sm">{label}</span>
	<div class="i-symbols-chevron-right h-5 w-5" />
</a>

<style>
	.disabled {
		--at-apply: pointer-events-none opacity-50;
	}
</style>
