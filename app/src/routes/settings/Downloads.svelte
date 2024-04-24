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
</article>
