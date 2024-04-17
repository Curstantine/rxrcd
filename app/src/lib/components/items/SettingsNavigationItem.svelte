<script>
	import { page } from "$app/stores";

	/** @type {string} */
	export let href;

	/** @type {string} */
	export let label;
</script>

<a {href} class:active={$page.url.pathname === href}>
	<div class="{$$props.class} h-5! w-5!" />
	<span class="text-sm">{label}</span>
</a>

<style>
	a {
		--at-apply: h-10 flex select-none items-center gap-x-2 rounded-md px-2 text-foreground use-transition-standard;
	}

	a:hover {
		--at-apply: underline;
	}

	a.active,
	a.active:hover {
		--at-apply: bg-secondary;
	}
</style>
