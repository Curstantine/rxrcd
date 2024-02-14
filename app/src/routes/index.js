import Artist from "@/routes/artist/Artist.svelte";
import Home from "@/routes/Home.svelte";
import Settings from "@/routes/Settings.svelte";

export const routes = {
	"/": Home,
	"/settings": Settings,
	"/artist/:id": Artist,
};
