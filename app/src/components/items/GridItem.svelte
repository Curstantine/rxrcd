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

	export let small = false;

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

<div bind:this={ref} class:small class="{$$props.class} item">
	<a {href} class="aspect-square">
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
	.item {
		--at-apply: flex flex-col use-transition-standard transition-colors;
	}

	.item:not(.small) {
		--at-apply: w-42;
	}

	.item.small {
		--at-apply: w-36;
	}

	.cover {
		--at-apply: inline-flex items-center justify-center h-42 w-42 border-(1 border solid) rounded;
	}

	.cover:hover {
		--at-apply: bg-secondary;
	}
</style>
