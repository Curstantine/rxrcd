declare type Result<T, E> = { data: T | null; error: E | null };
export declare type SearchEntryBaseResult = Result<SearchEntryBase[], string>;
export declare type SearchEntryIEResult = Result<SearchEntryIE[], string>;

export declare interface SearchEntryBase {
	id: number;
	title: string;
}

export declare type SearchEntrySub = SearchEntryBase & { subtitle: string };
export declare type SearchEntryIE = SearchEntrySub & { image: string | null };

export declare interface SearchEntries {
	artists: SearchEntryBaseResult | null;
	albums: SearchEntryIEResult | null;
}
