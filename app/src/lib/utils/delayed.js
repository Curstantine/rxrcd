/**
 * @template T
 * @param {(arg0: T) => void} func
 * @param {number} delay
 *
 * @return {{ run: (arg0: T) => void, clear: () => void }}
 */
export function debounce(func, delay = 1000) {
	let timeoutId;

	return {
		run: (arg0) => {
			clearTimeout(timeoutId);
			timeoutId = setTimeout(() => func.call(undefined, arg0), delay);
		},
		clear: () => clearTimeout(timeoutId),
	};
}

export async function wait(delay = 1000) {
	await new Promise((resolve) => setTimeout(resolve, delay));
}
