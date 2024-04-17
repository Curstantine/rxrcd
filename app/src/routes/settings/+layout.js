import { redirect } from "@sveltejs/kit";

/** @type {import('./$types').LayoutLoad} */
export async function load() {
	return redirect(308, "/settings/account");
}
