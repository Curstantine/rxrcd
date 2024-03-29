export declare type SearchData<T = "track" | "artist" | "album", D> = D & { type: T };

export declare type AlbumSearch = DeezerPaginatedList<SearchRelAlbum>;
export declare type ArtistAlbumList = DeezerPaginatedList<ArtistRelAlbum>;

export declare type ArtistSearch = DeezerPaginatedList<Artist>;

export declare interface DeezerList<T> {
	data: T[];
}

export declare interface DeezerPaginatedList<T> {
	data: T[];
	total: number;
	next: string | null;
}

export declare interface Genre {
	id: number;
	name: string;
}

export declare interface Track {
	id: number;
	title: string;
	explicit_lyrics: boolean;
	preview: string;
	duration: number;
}

export declare interface Artist {
	id: number;
	name: string;
	tracklist: string;
	picture_small: string | null;
	picture_big: string | null;
	nb_album: number;
	nb_fan: number;
}

export declare interface AlbumRelArtist {
	id: number;
	name: string;
	picture_small: string | null;
	picture_big: string | null;
}

export declare type AlbumRecordType = "album" | "ep" | "single" | "compilation";

export declare interface Album {
	id: number;
	title: string;
	link: string;
	cover_small: string | null;
	cover_medium: string | null;
	cover_big: string | null;
	artist: AlbumRelArtist;
	genres: DeezerList<Genre>;
	tracks: DeezerList<Track>;
	release_date: string;
	record_type: AlbumRecordType;
	explicit_lyrics: boolean;
}

export declare type SearchRelAlbum = Omit<Album, "genres" | "tracks" | "explicit_lyrics" | "record_type">;

export declare type ArtistRelAlbum = Omit<Album, "artist" | "genres" | "tracks">;
