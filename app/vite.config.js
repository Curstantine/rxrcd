import { resolve as resolvePath } from "node:path";

import { svelte } from "@sveltejs/vite-plugin-svelte";
import unocss from "unocss/vite";
import { defineConfig } from "vite";

export default defineConfig({
	plugins: [unocss(), svelte()],
	server: {
		port: 5173,
		strictPort: true,
	},
	resolve: {
		alias: [
			{
				find: "@",
				replacement: resolvePath("src"),
			},
		],
	},
});
