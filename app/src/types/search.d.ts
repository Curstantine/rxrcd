declare type Result<T, E> = { data: T | null; error: E | null };
export declare type SearchEntryBaseResult = Result<SearchEntryBase[], string>;
export declare type SearchEntryIEResult = Result<SearchEntryIE[], string>;

export declare interface SearchEntryBase {
	id: number;
	title: string;
	subtitle: string | null;
}

export declare type SearchEntryIE = SearchEntryBase & { image: string | null };

export declare interface SearchEntries {
	artists: SearchEntryBaseResult | null;
	albums: SearchEntryIEResult | null;
}
