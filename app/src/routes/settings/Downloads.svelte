<script>
	import SettingsHeading from "@/components/SettingsHeading.svelte";
	import SettingsOptionArea from "@/components/SettingsOptionArea.svelte";
	import Select from "@/components/select/Select.svelte";

	let selected_download_quality = "Mp3_128";
	let download_qualities = [
		{ label: "MP3 128K", value: "Mp3_128" },
		{ label: "MP3 320K", value: "Mp3_320" },
		{ label: "FLAC", value: "FLAC" },
	];

	/** @param {string} changed */
	const change_download_quality = (changed) => {
		const x = download_qualities.find(({ value }) => value === changed);
		if (x !== undefined) selected_download_quality = x.value;
	};
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
			label={selected_download_quality}
			class="w-42"
			aria_controls="select-color-scheme"
			actions={download_qualities}
			on_change={change_download_quality}
		/>
	</SettingsOptionArea>
</article>
