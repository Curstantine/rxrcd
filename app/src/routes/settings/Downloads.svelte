<script>
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
			class="input max-w-32"
		/>
	</SettingsOptionArea>
</article>
