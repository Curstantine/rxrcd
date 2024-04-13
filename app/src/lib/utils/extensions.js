/**
 * @template T,R
 * @param {T | null | undefined} source
 * @param {(arg0: T) => R} map
 *
 * @returns {R | null}
 */
export function take_if(source, map) {
	if (source !== null || source !== undefined) return map.call(this, source);
	else return null;
}

/**
 * @param {number} source
 * @param {number} min
 * @param {number} max
 */
export function clamp(source, min, max) {
	return Math.min(Math.max(source, min), max);
}
