<script>
	import { open } from "@tauri-apps/api/dialog";

	import SettingsHeading from "@/components/SettingsHeading.svelte";
	import SettingsOptionArea from "@/components/SettingsOptionArea.svelte";
	import Select from "@/components/Select.svelte";
	import Switch from "@/components/Switch.svelte";

	import { download_quality_actions, initialize_state, labels } from "@/routes/settings/Downloads.hooks";

	const { settings, change_property } = initialize_state();
</script>

<article id="downloads">
	<SettingsHeading heading="Downloads" sub="Configure the download location, quality, and many more" />

	<SettingsOptionArea
		label="Track Quality"
		subtitle={[
			"Change the quality of the downloaded tracks",
			"Note: You need to be logged in to use higher quality targets like FLAC",
		]}
		class="mt-4"
		option_id="select_download_quality"
	>
		<Select
			class="w-42"
			id="select-download-quality"
			disabled={$settings === null}
			label={labels[$settings?.quality ?? "NA"]}
			actions={download_quality_actions}
			on_change={async (val) =>
				await change_property("quality", /** @type {import("@/types/config").DownloadQuality}*/ (val))}
		/>
	</SettingsOptionArea>

	<SettingsOptionArea
		option_id="concurrent_downloads"
		label="Concurrent Downloads"
		subtitle="Number of downloads to run concurrently"
	>
		<input
			id="concurrent_downloads"
			type="number"
			min={1}
			max={10}
			value={$settings?.concurrent}
			disabled={$settings === null}
			class="w-42 input"
			on:change={async (e) => await change_property("concurrent", Number.parseInt(e.currentTarget.value))}
		/>
	</SettingsOptionArea>

	<SettingsOptionArea
		option_id="download_path"
		label="Location"
		subtitle="Save location for the downloaded tracks"
		layout="col"
	>
		<button
			id="download_path"
			disabled={$settings === null}
			class="w-full input"
			on:click={async () => {
				/** @type {string | null} */
				// @ts-expect-error multiple = false returns only string | nul
				const folder = await open({
					title: "Select a download folder",
					directory: true,
					multiple: false,
					defaultPath: $settings?.path,
				});

				if (folder !== null) {
					await change_property("path", folder);
				}
			}}
		>
			<div class="i-symbols-folder-outline-rounded mr-2 h-5 w-5 text-muted-foreground"></div>
			{$settings?.path ?? "N/A"}
		</button>
	</SettingsOptionArea>

	<SettingsOptionArea
		option_id="save_covers"
		label="Save Covers"
		subtitle="Download covers alongside the tracks"
		class="mt-4"
	>
		<Switch
			id="save_covers"
			disabled={$settings === null}
			value={$settings?.save_covers ?? false}
			on_click={async (val) => await change_property("save_covers", val)}
		/>
	</SettingsOptionArea>
</article>
