export declare type SearchData<T = "track" | "artist" | "album", D> = D & { type: T };

export declare type AlbumSearch = DeezerPaginatedList<SearchRelAlbum>;

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
	artist: TrackRelArtist;
}

export declare interface Artist {
	id: number;
	name: string;
	tracklist: string;
	picture_small: string;
	picture_big: string;
	nb_album: number;
	nb_fan: number;
}

export declare interface AlbumRelArtist {
	id: number;
	name: string;
	picture_small: string;
	picture_big: string;
}

export declare interface TrackRelArtist {
	id: number;
	name: string;
}

export declare interface Album {
	id: number;
	title: string;
	link: string;
	cover_small: string;
	cover_big: string;
	artist: AlbumRelArtist;
	genres: DeezerList<Genre>;
	tracks: DeezerList<Track>;
}

export declare interface SearchRelAlbum {
	id: number;
	title: string;
	link: string;
	cover_small: string;
	cover_big: string;
	artist: AlbumRelArtist;
}
