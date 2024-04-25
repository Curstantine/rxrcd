import { onMount } from "svelte";
import { get, readonly, writable } from "svelte/store";

import { config_get_download, config_set_download } from "@/bindings/config";
import { create_snack } from "@/components/snack/snack";

/**
 * @type {Record<import("@/types/config").DownloadQuality | "NA", string>}
 * @constant
 */
export const download_quality_labels = {
	Mp3_128: "MP3 120K",
	Mp3_320: "MP3 320K",
	Flac: "FLAC",
	NA: "-",
};

/**
 * @type {Record<import("@/types/config").CoverQuality | "NA", string>}
 * @constant
 */
export const cover_quality_labels = {
	Small: "Small",
	Medium: "Medium",
	Big: "Big",
	Xl: "XL",
	NA: "-",
};

/** @type {import("@/types/select").Action<import("@/types/config").DownloadQuality>[]} */
export const download_quality_actions = [
	{ value: "Mp3_128", label: download_quality_labels["Mp3_128"], sub: "(Low Quality)" },
	{ value: "Mp3_320", label: download_quality_labels["Mp3_320"], sub: "(High Quality)" },
	{ value: "Flac", label: download_quality_labels["Flac"], sub: "(Lossless Quality)" },
];

/** @type {import("@/types/select").Action<import("@/types/config").CoverQuality>[]} */
export const cover_quality_actions = [
	{ value: "Small", label: cover_quality_labels["Small"], sub: "(56px)" },
	{ value: "Medium", label: cover_quality_labels["Medium"], sub: "(250px)" },
	{ value: "Big", label: cover_quality_labels["Big"], sub: "(500px)" },
	{ value: "Xl", label: cover_quality_labels["Xl"], sub: "(1000px)" },
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
