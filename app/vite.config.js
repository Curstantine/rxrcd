import { svelte } from "@sveltejs/vite-plugin-svelte";
import { resolve as resolvePath } from "node:path";
import unocss from "unocss/vite";
import { defineConfig } from "vite";

export default defineConfig({
	plugins: [unocss(), svelte()],
	resolve: {
		alias: [
			{
				find: "@",
				replacement: resolvePath("src"),
			},
		],
	},
});
