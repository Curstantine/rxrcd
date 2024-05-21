import type { DownloadQuality } from "@/types/config";

export declare interface User {
	id: number;
	name: string;
	email: string;
	country: string;
	offer_name: string;
	sound_quality: DownloadQuality[];
}
