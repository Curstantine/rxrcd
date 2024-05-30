import type { DownloadQuality } from "@/types/config";

declare interface UserAuthTypeData {
	arl: { arl: string };
	credentials: { email: string; password: string };
}

export declare type UserAuthType = { type: "Arl"; arl: string } | {
	type: "Credentials";
	email: string;
	password: string;
};

export declare interface UserAuthState {
	type: "LoggedIn" | "Incomplete" | "NotLoggedIn";
	data?: UserAuthType;
}

export declare interface User {
	id: number;
	name: string;
	email: string;
	country: string;
	offer_name: string;
	sound_quality: DownloadQuality[];
}
