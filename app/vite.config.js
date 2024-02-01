import { svelte } from "@sveltejs/vite-plugin-svelte";
import unocss from "unocss/vite";
import { defineConfig } from "vite";

// https://vitejs.dev/config/
export default defineConfig({
	plugins: [unocss(), svelte()],
});
