<script>
	import { onMount } from "svelte";

	/** @type {string} */
	export let href;

	/** @type {string} */
	export let title;

	/** @type {string | null} */
	export let subtitle;

	/** @type {string | null} */
	export let image;

	/** @type {HTMLDivElement} */
	let ref;

	let is_img_visible = false;

	onMount(() => {
		const observer = new IntersectionObserver((entries) => {
			entries.forEach((entry) => (is_img_visible = entry.isIntersecting));
		});

		observer.observe(ref);

		return () => {
			observer.unobserve(ref);
		};
	});
</script>

<div bind:this={ref} class="{$$props.class} flex flex-col px-2 py-1 w-36 use-transition-standard transition-colors">
	<a {href} class="cover">
		{#if image !== null && is_img_visible}
			<img src={image} alt="{title} Cover Preview" class="w-full rounded" />
		{/if}
	</a>

	<a {href} class="pt-2 text-sm text-foreground leading-tight">{title}</a>

	{#if subtitle !== null}
		<span class="mt-0.5 text-xs text-muted-foreground">{subtitle}</span>
	{/if}
</div>

<style>
	.cover {
		--at-apply: inline-flex h-32 w-32 items-center justify-center border-(1 border solid) rounded;
	}

	.cover:hover {
		--at-apply: bg-secondary;
	}
</style>
