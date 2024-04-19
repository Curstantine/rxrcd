<script>
	import { link } from "svelte-spa-router";

	/** @type {string} */
	export let href;

	/** @type {boolean} */
	export let active;

	/** @type {string} */
	export let label;

	/** @type {string} */
	export let icon;
</script>

<a use:link {href} class:active>
	<div class="{icon} h-5 w-5" />
	<span class="text-sm">{label}</span>
</a>

<style>
	a {
		--at-apply: h-10 flex items-center gap-x-2 rounded-md px-2 text-foreground use-transition-standard;
	}

	a:hover {
		--at-apply: underline;
	}

	a.active {
		--at-apply: bg-secondary decoration-none pointer-events-none;
	}
</style>
