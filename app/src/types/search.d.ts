declare type Result<T, E> = { data: T | null; error: E | null };

export declare interface SearchEntryBase {
	id: number;
	title: string;
}

export declare type SearchEntrySub = SearchEntryBase & { subtitle: string };

export declare type SearchEntryIE = SearchEntrySub & {
	image: string | null;
};

export declare type SearchEntries = {
	artists: Result<SearchEntryBase[], string> | null;
	albums: Result<SearchEntryIE[], string> | null;
};
