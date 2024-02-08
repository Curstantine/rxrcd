export declare interface SearchEntry {
	title: string;
	href: string;
	subtitle?: string;
	image?: string;
}

export declare type SearchEntries = [{ title: string; href: string; list_type: "list" | "grid" }, SearchEntry[]][];
