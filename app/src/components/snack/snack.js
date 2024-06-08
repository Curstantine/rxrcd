import { derived, get, readonly, writable } from "svelte/store";

export const DEFAULT_SNACK_TIMEOUT = 5000;

/**
 * @typedef {import("svelte/store").Writable<import("@/types/snack").SnackInstance>} WritableInstance
 * @typedef {import("svelte/store").Readable<import("@/types/snack").SnackInstance>} ReadableInstance
 * @typedef {{ [key: symbol]: import("@/types/snack").InstanceMapEntry }} InstanceMap
 *
 * @type {import("svelte/store").Writable<InstanceMap>}
 */
const instances = writable({});

/**
 * @type {import("svelte/store").Readable<ReadableInstance[]>}
 */
export const snacks = derived(instances, (source) => {
	const keys = Object.getOwnPropertySymbols(source);

	/** @type {ReadableInstance[]} */
	const instances = new Array(keys.length);

	for (let i = 0; i < keys.length; i++) {
		const key = keys[i];
		const val = source[key];
		instances[i] = readonly(val.inner);
	}

	return instances;
});

/**
 * @param {symbol} id
 */
function close_snack(id) {
	instances.update((stack) => {
		delete stack[id];
		return stack;
	});
}

export function pause_snack_timeouts() {
	const source = get(instances);
	const keys = Object.getOwnPropertySymbols(source);

	for (let i = 0; i < keys.length; i++) {
		const key = keys[i];
		const val = source[key];

		if (val.timeout !== undefined) {
			window.clearTimeout(val.timeout);
			val.timeout = -1;
		}
	}
}

export function resume_snack_timeouts() {
	const source = get(instances);
	const keys = Object.getOwnPropertySymbols(source);

	for (let i = 0; i < keys.length; i++) {
		const key = keys[i];
		const val = source[key];

		if (val.timeout !== undefined) {
			val.timeout = window.setTimeout(() => close_snack(key), DEFAULT_SNACK_TIMEOUT);
		}
	}
}

/**
 * @param {import("@/types/snack").SnackInstance} instance
 */
export function create_snack(instance) {
	const id = Symbol();
	const writable_instance = writable(instance);

	/** @type {import("@/types/snack").SnackInstanceUpdater} */
	function update({ label, description = instance.description }) {
		if (!instance.persistent) {
			throw new Error("Updatable notifications should be persistent");
		}

		writable_instance.update((value) => {
			value.label = label;
			value.description = description;
			return value;
		});
	}

	function close() {
		close_snack(id);
	}

	/** @type {number | undefined} */
	let timeout;

	if (!instance.persistent) {
		timeout = window.setTimeout(close, DEFAULT_SNACK_TIMEOUT);
	}

	instances.update((stack) => {
		stack[id] = { inner: writable_instance, timeout };
		return stack;
	});

	function close_after(duration = DEFAULT_SNACK_TIMEOUT) {
		if (timeout === -1) throw new Error("DO NOT try to mutate this instance, it's marked as paused. ");
		if (timeout !== undefined) throw new Error("A timeout for this snack instance is already running");

		timeout = window.setTimeout(close, duration);
	}

	return { update, close, close_after };
}
