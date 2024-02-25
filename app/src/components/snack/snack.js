import { derived, writable } from "svelte/store";

/**
 * @typedef {import("svelte/store").Writable<import("@/types/snack").SnackInstance>} WritableInstance
 * @typedef {{ [key: symbol]: WritableInstance }} InstanceMap
 *
 * @type {import("svelte/store").Writable<InstanceMap>}
 */
const data = writable({});

/**
 * @type {import("svelte/store").Readable<WritableInstance[]>}
 */
export const snacks = derived(data, (source) => {
	const properties = Object.getOwnPropertySymbols(source);
	const instances = new Array(properties.length);

	for (let i = 0; i < properties.length; i++) {
		const key = properties[i];
		const val = source[key];
		instances[i] = val;
	}

	return instances;
});

/**
 * @param {import("@/types/snack").SnackInstance} instance
 */
export function pushToSnackStack(instance) {
	const id = Symbol();
	const writableInstance = writable(instance);

	/** @type {number} */
	let closeTimeout;

	data.update((stack) => {
		return { ...stack, [id]: writableInstance };
	});

	/** @type {import("@/types/snack").SnackInstanceUpdater} */
	function update({ label, description = instance.description }) {
		writableInstance.update((value) => {
			if (!instance.persistent) {
				clearTimeout(closeTimeout);
				value.persistent = true;
			}

			value.label = label;
			value.description = description;
			return value;
		});
	}

	function close() {
		data.update((stack) => {
			delete stack[id];
			return stack;
		});
	}

	if (!instance.persistent) {
		closeTimeout = window.setTimeout(() => close(), 5000);
	}

	return {
		update,
		close,
	};
}
