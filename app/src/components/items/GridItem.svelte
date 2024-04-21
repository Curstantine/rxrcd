<script>
	import { onMount } from "svelte";
	import { fade } from "svelte/transition";
	import { link } from "svelte-spa-router";

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
		const observer = new IntersectionObserver((entries, ob) => {
			entries.forEach((entry) => (is_img_visible = entry.isIntersecting));
			if (is_img_visible) ob.unobserve(ref);
		});

		observer.observe(ref);

		return () => {
			observer.unobserve(ref);
		};
	});
</script>

<div bind:this={ref} class:small class="{$$props.class} item">
	<a use:link {href} class="aspect-square">
		{#if image !== null && is_img_visible}
			<img
				in:fade={{ duration: 300 }}
				height="256"
				width="256"
				src={image}
				alt="{title} Cover Preview"
				class="rounded"
			/>
		{/if}
	</a>

	<a use:link {href} class="pt-2 text-sm text-foreground leading-tight">{title}</a>

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
