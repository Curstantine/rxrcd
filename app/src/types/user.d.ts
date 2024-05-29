import type { DownloadQuality } from "@/types/config";

declare interface UserAuthTypeData {
	arl: { arl: string };
	credentials: { email: string; password: string };
}
export declare type UserAuthType<T extends "arl" | "credentials"> = T extends "arl" ? { type: "arl"; arl: string }
	: { type: "credentials"; email: string; password: string };

export declare interface UserAuthState<T extends "arl" | "credentials" | unknown> {
	type: "logged_in" | "incomplete" | "not_logged_in";
	data: UserAuthType<T>;
}

export declare interface User {
	id: number;
	name: string;
	email: string;
	country: string;
	offer_name: string;
	sound_quality: DownloadQuality[];
}
