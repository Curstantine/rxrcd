import { LOCAL_AUTH_DATA } from "@/utils/auth";
import { readonly, writable } from "svelte/store";

/** @type {import("svelte/store").Writable<import("@/types/user").User | null>} */
const user_data_ = writable(null);

export const user_data = readonly(user_data_);

/** @param {import("@/types/user").User | null} data */
export function set_user_data(data) {
	user_data_.set(data);

	if (data === null) return localStorage.removeItem(LOCAL_AUTH_DATA);

	const local = { ...data, timestamp: Date.now() };
	return localStorage.setItem(LOCAL_AUTH_DATA, JSON.stringify(local));
}
