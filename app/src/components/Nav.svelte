<script>
	import { tick, onDestroy } from "svelte";
	import { writable, get } from "svelte/store";
	import { location, pop } from "svelte-spa-router";

	import Input from "@/components/Input.svelte";

	/** @type {string} */
	let search;

	/** @type {import("svelte/store").Writable<string[]>}*/
	const history_stack = writable([]);
	let inner_action = false;
	let current_index = 0;

	/**
	 * We need to check if the url change is from a back/forward action.
	 *
	 * If it is, we ignore since we are just traversing the stack.
	 * If not, we need to check if the user has pressed "back" before and remove the future tree from that index accordingly.
	 */
	const un_sub = location.subscribe((url) => {
		const stack = get(history_stack);

		// We don't want to mutate history_stack if the action comes from a internal back/forth change.
		if (inner_action) {
			inner_action = false;
			return;
		}

		// We don't want to mutate current_index in the initial render, so the "/" path won't start back/forth.
		if (stack.length === 0) {
			stack.push(url);
			return;
		}

		const stack_end_index = stack.length - 1;
		const current_pos = current_index + 1;

		// We can check if this is "backed" before or not by comparing the current index and last index of the history stack.
		// The index that gets replaced should be "next forward" (So the forward tree resets)
		if (current_index !== stack_end_index) {
			stack.splice(current_pos, stack.length - current_pos, url);
			current_index = stack.length - 1;

			return history_stack.set(stack);
		}

		stack.push(url);
		current_index++;
		return history_stack.set(stack);
	});

	$: back_disabled = $history_stack && current_index === 0;
	$: forward_disabled = current_index === $history_stack.length - 1;

	const back = async () => {
		inner_action = true;
		current_index -= 1;
		await pop();
	};

	const forward = async () => {
		inner_action = true;
		current_index += 1;

		await tick();
		window.history.forward();
	};

	onDestroy(() => un_sub());
</script>

<div class="sticky inset-x-0 top-0 flex bg-background/95 px-4 py-2 backdrop-blur-xl space-x-4">
	<div class="inline-flex space-x-2">
		<a href="#/" class="h-8 w-8 icon-button-layout ghost-button">
			<div class="i-symbols-home-rounded h-5 w-5" class:text-foreground={$location === "/"} />
		</a>

		<button on:click={back} class="h-8 w-8 icon-button-layout ghost-button" disabled={back_disabled}>
			<div class="i-symbols-chevron-left h-5 w-5" />
		</button>
		<button on:click={forward} class="h-8 w-8 icon-button-layout ghost-button" disabled={forward_disabled}>
			<div class="i-symbols-chevron-right h-5 w-5" />
		</button>
	</div>

	<Input bind:input={search} placeholder="Search" class="flex-1" />

	<a href="#/settings" class="h-8 w-8 icon-button-layout ghost-button">
		<div class="i-symbols-settings-rounded h-5 w-5" class:text-foreground={$location === "/settings"} />
	</a>
</div>
