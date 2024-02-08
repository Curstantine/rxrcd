/**
 * @param {() => void} func
 * @param {number} delay
 */
export default function debounce(func, delay = 1000) {
	let timeoutId;

	return {
		run: () => {
			clearTimeout(timeoutId);
			timeoutId = setTimeout(() => func.call(undefined), delay);
		},
		clear: () => clearTimeout(timeoutId),
	};
}
