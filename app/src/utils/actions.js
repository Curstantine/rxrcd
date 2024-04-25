/**
 * @template {HTMLElement} T
 * @param {T} node
 * @param {{ close: () => void, coupling_ids: string[] }} arg1
 */
export function external_hit(node, { close, coupling_ids }) {
	/** @param e {MouseEvent & { target: HTMLElement }} */
	function listener(e) {
		if (coupling_ids.includes(e.target.id)) return;
		if (!node.contains(e.target)) close.call(null);
	}

	// @ts-expect-error DOM is like this
	document.addEventListener("click", listener);

	return {
		// @ts-expect-error DOM is like this
		destroy: () => document.removeEventListener("click", listener),
	};
}
