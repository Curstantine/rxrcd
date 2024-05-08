import { readonly, writable } from "svelte/store";

/** @type {import("svelte/store").Writable<import("@/types/user").UserData | null>} */
const user_data_ = writable(null);

export const set_user_data = user_data_.set;
export const user_data = readonly(user_data_);
