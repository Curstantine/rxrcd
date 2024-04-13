import { sveltekit } from "@sveltejs/kit/vite";
import unocss from "@unocss/svelte-scoped/vite";
import { resolve as resolvePath } from "node:path";
import { defineConfig } from "vite";

export default defineConfig({
	plugins: [unocss({ injectReset: "@unocss/reset/tailwind.css" }), sveltekit()],
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
