<script>
	import SnackBar from "@/components/snack/SnackBar.svelte";
	import { pause_snack_timeouts, resume_snack_timeouts, snacks } from "@/components/snack/snack";

	let expanded = false;

	const onEnter = () => {
		if (expanded) return;

		expanded = true;
		pause_snack_timeouts();
	};

	const onLeave = () => {
		if (!expanded) return;

		expanded = false;
		resume_snack_timeouts();
	};

	/** @type {import("svelte/action").Action<HTMLUListElement>}*/
	function auto_sizer(node) {
		let cache_item_height = 0;

		/** @type {MutationCallback} */
		const callback = ([mutation]) => {
			const is_add_mut = mutation.addedNodes.length > 0;

			if (is_add_mut && cache_item_height === 0) {
				/** @type {HTMLLIElement}*/
				// @ts-ignore
				const child = mutation.addedNodes[0];
				const child_rect = child.getBoundingClientRect();

				cache_item_height = child_rect.height;
				node.style.setProperty("width", `${child_rect.width}px`);
			}

			const height = node.children.length * cache_item_height;
			node.style.setProperty("height", `${height}px`);
		};

		const observer = new MutationObserver(callback);
		observer.observe(node, { childList: true });

		return {
			destroy: () => observer.disconnect(),
		};
	}
</script>

<ul
	use:auto_sizer
	role="group"
	class:expanded
	class="absolute bottom-4 right-4"
	on:mouseenter={onEnter}
	on:mouseleave={onLeave}
>
	{#each $snacks as data, index}
		<SnackBar {data} {index} length={$snacks.length} expand={expanded} />
	{/each}
</ul>

<style>
	ul:not(.expanded) {
		--at-apply: max-h-0;
	}
</style>
