import Album from "@/routes/album/Album.svelte";
import Artist from "@/routes/artist/Artist.svelte";
import Home from "@/routes/Home.svelte";
import Settings from "@/routes/settings/Settings.svelte";

export const routes = {
	"/": Home,
	"/settings": Settings,
	"/settings/*": Settings,
	"/artist/:id": Artist,
	"/album/:id": Album,
};
