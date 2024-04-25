<script>
	import { open } from "@tauri-apps/api/dialog";

	import SettingsHeading from "@/components/SettingsHeading.svelte";
	import SettingsOptionArea from "@/components/SettingsOptionArea.svelte";
	import Select from "@/components/select/Select.svelte";

	import { download_quality_actions, initialize_state, labels } from "@/routes/settings/Downloads.hooks";

	const { settings, change_property } = initialize_state();
</script>

<article id="downloads">
	<SettingsHeading heading="Downloads" sub="Configure the download location, quality, and many more" />

	<SettingsOptionArea
		class="mt-4"
		label="Track Quality"
		subtitle={[
			"Change the quality of the downloaded tracks",
			"Note: You need to be logged in to use higher quality targets like FLAC",
		]}
	>
		<Select
			disabled={$settings === null}
			label={labels[$settings?.quality ?? "NA"]}
			actions={download_quality_actions}
			class="w-42"
			aria_controls="select-download-quality"
			on_change={async (val) =>
				await change_property("quality", /** @type {import("@/types/config").DownloadQuality}*/ (val))}
		/>
	</SettingsOptionArea>

	<SettingsOptionArea label="Concurrent Downloads" subtitle="Number of downloads to run concurrently">
		<input
			type="number"
			min={1}
			max={10}
			value={$settings?.concurrent}
			disabled={$settings === null}
			on:change={async (e) => await change_property("concurrent", Number.parseInt(e.currentTarget.value))}
			class="w-42 input"
		/>
	</SettingsOptionArea>

	<SettingsOptionArea label="Location" subtitle="Save location for the downloaded tracks" layout="col">
		<button
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
</article>
