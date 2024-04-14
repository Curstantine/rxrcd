import { readonly, writable } from "svelte/store";

/**
 * @typedef {{ id: string, label: string, rel_type: import("$lib/types/deezer").AlbumRecordType | null }} TabItem
 * @type {TabItem[]}
 */
export const tabs = [
	{ id: "tab_discography", rel_type: null, label: "Discography" },
	{ id: "tab_albums", rel_type: "album", label: "Albums" },
	{ id: "tab_eps", rel_type: "ep", label: "EPs" },
	{ id: "tab_singles", rel_type: "single", label: "Singles" },
	{ id: "tab_compilations", rel_type: "compilation", label: "Compilations" },
];

export function extort_tab_state() {
	const active_tab = writable(tabs[0]);

	/** @param e {MouseEvent & { currentTarget: HTMLButtonElement }} */
	function on_tab_click(e) {
		const new_tab = tabs.find(({ id }) => e.currentTarget.id === id);
		if (new_tab === undefined) throw new Error(`Tab hit area out-of-bound with #${e.currentTarget.id}`);
		active_tab.set(new_tab);
	}

	return { active_tab: readonly(active_tab), on_tab_click };
}

/**
 *  @param {HTMLDivElement} node
 *  @param {import("svelte/store").Readable<TabItem>} active_tab
 */
export function style_tab(node, active_tab) {
	/**
	 * @param {TabItem} param0
	 */
	const update_styles = ({ id }) => {
		const element = document.getElementById(id);
		if (element === null) throw new Error(`Tab by #${id} doesn't exist`);

		node.style.width = `${element.offsetWidth}px`;
		node.style.transform = `translateX(${element.offsetLeft}px)`;
	};

	const un_sub = active_tab.subscribe(update_styles);

	return {
		destroy: () => un_sub(),
	};
}

/**
 *  @param {HTMLDivElement} node
 */
export function style_tab_bar(node) {
	const app = document.getElementById("app");
	let marked = false;

	if (app === null) throw new Error("#app is not mounted");

	const styler = () => {
		if (!marked && app.scrollTop >= node.offsetTop) {
			node.style.setProperty("--un-bg-color-opacity", "0.95");
			return marked = true;
		}

		if (marked && node.offsetTop > app.scrollTop) {
			node.style.setProperty("--un-bg-color-opacity", "1");
			return marked = false;
		}
	};

	app.addEventListener("scroll", styler);

	return {
		destroy: () => app.removeEventListener("scroll", styler),
	};
}
