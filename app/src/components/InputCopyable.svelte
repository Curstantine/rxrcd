<script>
	import { clipboard } from "@tauri-apps/api";

	import { create_snack } from "@/components/snack/snack";

	export let grammar;
	export let value;

	const push_to_clipboard = async () => {
		try {
			await clipboard.writeText(value);
			create_snack({ label: `Copied ${grammar} to the clipboard` });
		} catch (e) {
			create_snack({ label: `Failed to copy ${grammar} to the clipboard`, description: e?.toString() });
		}
	};
</script>

<div class="relative">
	<slot />

	<button class="clip icon-button" on:click|preventDefault={push_to_clipboard}>
		<div class="i-symbols-content-copy-outline-rounded size-4"></div>
	</button>
</div>

<style>
	.clip {
		--at-apply: size-7 flex justify-center items-center rounded-md text-muted-foreground absolute right-1.25
			inset-y-1;
	}
</style>
