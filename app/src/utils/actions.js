/**
 * @template {HTMLElement} T
 * @satisfies {import("@/types/actions").ExternalHit<T>}
 */
export function external_hit(node, { close, coupling_ids }) {
	/** @param e {PointerEvent & { target: T }} */
	function listener(e) {
		if (coupling_ids.contains(e.target.id)) return;
		if (!node.contains(e.target)) close.call(null);
	}

	document.addEventListener("click", listener);

	return {
		destroy: () => document.removeEventListener("click", listener),
	};
}
