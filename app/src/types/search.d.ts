export declare interface SearchEntry {
	title: string;
	href: string;
	subtitle?: string;
	image?: string;
}

export declare type SearchEntries = [string, "list" | "grid", SearchEntry[]][];
