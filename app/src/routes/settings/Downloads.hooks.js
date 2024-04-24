import { onMount } from "svelte";
import { get, readonly, writable } from "svelte/store";

import { config_get_download, config_set_download } from "@/bindings/config";
import { create_snack } from "@/components/snack/snack";

/** @type {{ label: string; value: import("@/types/config").DownloadQuality }[]} */
export const download_quality_actions = [
	{ label: "MP3 128K (Low Quality)", value: "Mp3_128" },
	{ label: "MP3 320K (High Quality)", value: "Mp3_320" },
	{ label: "FLAC (Lossless Quality)", value: "Flac" },
];

export function initialize_state() {
	/** @type {import("svelte/store").Writable<import("@/types/config").ConfigurationDownload | null>} */
	const settings = writable(null);

	onMount(async () => {
		const conf = await config_get_download();
		settings.set(conf);
	});

	/**
	 * @template {keyof import("@/types/config").ConfigurationDownload} T
	 * @param {T} property
	 * @param {import("@/types/config").ConfigurationDownload[T]} value
	 */
	async function change_property(property, value) {
		try {
			const snap = get(settings);
			if (snap === null || snap[property] === value) return;

			snap[property] = value;
			await config_set_download(snap);
			settings.set(snap);
		} catch (e) {
			create_snack({
				label: `Caught an error while changing ${property} to ${value}`,
				description: e?.toString() ?? "N/A",
			});
		}
	}

	return { change_property, settings: readonly(settings) };
}
