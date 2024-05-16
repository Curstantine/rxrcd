import { refresh_login } from "@/bindings/user";
import { set_user_data } from "@/stores/user";
import { take_if } from "@/utils/extensions";

// 1000ms -> 1s -> 5m
const STANDARD_SINCE_MS = 60000 * 5;
const LOCAL_AUTH_DATA = "cringily-cached-auth-data";

/**
 * @param {boolean} cringily_use_cache
 */
export async function resume_auth(cringily_use_cache = false) {
	// Note(Curstantine): The development env reruns the setup hook every time a HMR happens,
	// but we don't want to disturb deezer with a bunch of auth requests.
	// We'll cringily cache the user data in those cases.
	// P.S. I know we could import.meta.env.DEV, but where's the fun in that
	if (cringily_use_cache) {
		/** @type {import("@/types/user").UserData & { timestamp: number } | null} */
		const data = take_if(localStorage.getItem(LOCAL_AUTH_DATA), JSON.parse);

		if (data !== null && ((data.timestamp - Date.now()) < STANDARD_SINCE_MS)) {
			// @ts-expect-error We don't want the timestamp property in the user_data object.
			delete data.timestamp;
			set_user_data(data);
		}
	}

	const data = await refresh_login();
	set_user_data(data);

	const local = { ...data, timestamp: Date.now() };
	localStorage.setItem(LOCAL_AUTH_DATA, JSON.stringify(local));
}