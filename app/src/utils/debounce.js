/**
 * @param {() => void} func
 * @param {number} delay
 */
export default function debounce(func, delay = 1000) {
	let timeoutId;

	return () => {
		clearTimeout(timeoutId);
		timeoutId = setTimeout(() => func.call(undefined), delay);
	};
}
