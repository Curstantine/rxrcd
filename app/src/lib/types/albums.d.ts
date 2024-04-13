import type { AlbumRecordType, ArtistRelAlbum, DeezerPaginatedList } from "$lib/types/deezer";

export declare type DerivedAlbumList = Omit<DeezerPaginatedList<null>, "data"> & {
	data: Record<AlbumRecordType, ArtistRelAlbum[]>;
};
