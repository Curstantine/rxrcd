import { onDestroy, tick } from "svelte";
import { location, pop } from "svelte-spa-router";
import { derived, get, writable } from "svelte/store";

/**
 * @typedef {import("svelte/store").Readable<boolean>} ReadableBool
 * @typedef {() => void} VoidFunction
 *
 * @returns {[[ReadableBool, ReadableBool], { back: VoidFunction, forward: VoidFunction }]} [[back_disabled, forward_disabled], { back, forward }]
 */
export function extort_nav_state() {
	let internal = false;

	/** @type {import("svelte/store").Writable<string[]>} */
	const history_stack = writable([]);
	const current_index = writable(0);

	const back_disabled = derived(current_index, ($current_index) => $current_index === 0);
	const forward_disabled = derived(
		[history_stack, current_index],
		([$history_stack, $current_index]) => $current_index === $history_stack.length - 1,
	);

	/**
	 * We need to check if the url change is from a back/forward action.
	 *
	 * If it is, we ignore since we are just traversing the stack.
	 * If not, we need to check if the user has pressed "back" before and remove the future tree from that index accordingly.
	 */
	const un_sub = location.subscribe((url) => {
		const stack = get(history_stack);
		const index = get(current_index);

		// We don't want to mutate history_stack if the action comes from a internal back/forth change.
		if (internal) {
			internal = false;
			return;
		}

		// We don't want to mutate current_index in the initial render, so the "/" path won't start back/forth.
		if (stack.length === 0) {
			stack.push(url);
			return;
		}

		// We can check if this is "backed" before or not by comparing the current index and last index of the history stack.
		// The index that gets replaced should be "next forward" (So the forward tree resets)
		if (index !== stack.length - 1) {
			const current_pos = index + 1;

			stack.splice(current_pos, stack.length - current_pos, url);
			current_index.set(stack.length - 1);
		} else {
			stack.push(url);
			current_index.update((index) => index + 1);
		}

		return history_stack.set(stack);
	});

	const back = async () => {
		internal = true;
		current_index.update((index) => index - 1);
		await pop();
	};

	const forward = async () => {
		internal = true;
		current_index.update((index) => index + 1);

		await tick();
		window.history.forward();
	};

	onDestroy(() => un_sub());

	return [[back_disabled, forward_disabled], { back, forward }];
}
