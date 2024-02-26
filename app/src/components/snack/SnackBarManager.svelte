<script>
	import SnackBar from "@/components/snack/SnackBar.svelte";
	import { pauseSnackTimeouts, resumeSnackTimeouts, snacks } from "@/components/snack/snack";

	let expanded = false;

	const onEnter = () => {
		if (expanded) return;

		expanded = true;
		pauseSnackTimeouts();
	};

	const onLeave = () => {
		if (!expanded) return;

		expanded = false;
		resumeSnackTimeouts();
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
	class="absolute bottom-4 right-4"
	class:expanded
	role="group"
	on:mouseenter={onEnter}
	on:mouseleave={onLeave}
>
	{#each $snacks as data, index}
		<SnackBar {data} {index} length={$snacks.length} expand={expanded} />
	{/each}
</ul>

<style>
	ul:not(.expand) {
		--at-apply: max-h-0;
	}
</style>
