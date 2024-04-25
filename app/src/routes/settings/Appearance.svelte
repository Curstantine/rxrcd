<script>
	import { set_theme, selected_theme, themes } from "@/utils/theme";

	import SettingsHeading from "@/components/SettingsHeading.svelte";
	import Select from "@/components/select/Select.svelte";
	import SettingsOptionArea from "@/components/SettingsOptionArea.svelte";

	const actioned_themes = themes.map(({ label, id }) => ({ label, value: id }));

	/** @param {string} changed */
	const change_theme = async (changed) => {
		// @ts-expect-error changed is of type Theme as we feed values in themes
		if (changed !== $selected_theme?.id) await set_theme(changed);
	};
</script>

<article id="appearance">
	<SettingsHeading heading="Appearance" sub="Tweak rxrcd's look and feel" />

	<SettingsOptionArea
		label="Theme"
		subtitle="Change application-wide color scheme"
		class="mt-4"
		option_id="color-scheme"
	>
		<Select
			id="color-scheme"
			label={$selected_theme?.label ?? "N/A"}
			actions={actioned_themes}
			on_change={change_theme}
			class="w-42"
		/>
	</SettingsOptionArea>
</article>
