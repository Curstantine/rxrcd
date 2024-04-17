<script>
	import { link } from "svelte-spa-router";
	import active from "svelte-spa-router/active";

	export let href;

	/** @type {string} */
	export let label;

	/** @type {string} */
	export let icon;
</script>

<a use:link use:active {href}>
	<div class="{icon} h-5 w-5" />
	<span class="text-sm">{label}</span>
</a>

<style>
	a {
		--at-apply: h-10 flex items-center gap-x-2 rounded-md px-2 text-foreground use-transition-standard;
	}

	a:hover {
		--at-apply: bg-secondary;
	}

	:global(a.active) {
		--at-apply: bg-secondary;
	}
</style>
