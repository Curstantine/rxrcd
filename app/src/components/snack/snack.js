import { derived, get, readonly, writable } from "svelte/store";

const TIMEOUT_MS = 5000;

/**
 * @typedef {import("svelte/store").Writable<import("@/types/snack").SnackInstance>} WritableInstance
 * @typedef {import("svelte/store").Readable<import("@/types/snack").SnackInstance>} ReadableInstance
 * @typedef {{ [key: symbol]: import("@/types/snack").InstanceMapEntry }} InstanceMap
 *
 * @type {import("svelte/store").Writable<InstanceMap>}
 */
const data = writable({});

/**
 * @type {import("svelte/store").Readable<ReadableInstance[]>}
 */
export const snacks = derived(data, (source) => {
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
	data.update((stack) => {
		delete stack[id];
		return stack;
	});
}

export function pause_snack_timeouts() {
	const source = get(data);
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
	const source = get(data);
	const keys = Object.getOwnPropertySymbols(source);

	for (let i = 0; i < keys.length; i++) {
		const key = keys[i];
		const val = source[key];

		if (val.timeout !== undefined) {
			val.timeout = window.setTimeout(() => close_snack(key), TIMEOUT_MS);
		}
	}
}

/**
 * @param {import("@/types/snack").SnackInstance} instance
 */
export function create_snack(instance) {
	const id = Symbol();
	const writableInstance = writable(instance);

	/** @type {import("@/types/snack").SnackInstanceUpdater} */
	function update({ label, description = instance.description }) {
		if (!instance.persistent) {
			throw new Error("Updatable notifications should be persistent");
		}

		writableInstance.update((value) => {
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
		timeout = window.setTimeout(() => close_snack(id), TIMEOUT_MS);
	}

	data.update((stack) => {
		stack[id] = { inner: writableInstance, timeout };
		return stack;
	});

	return { update, close };
}
