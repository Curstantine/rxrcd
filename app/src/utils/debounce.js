/**
 * @template T
 * @param {(arg0: T) => void} func
 * @param {number} delay
 *
 * @return {{ run: (arg0: T) => void, clear: () => void }}
 */
export default function debounce(func, delay = 1000) {
	let timeoutId;

	return {
		run: (arg0) => {
			clearTimeout(timeoutId);
			timeoutId = setTimeout(() => func.call(undefined, arg0), delay);
		},
		clear: () => clearTimeout(timeoutId),
	};
}
