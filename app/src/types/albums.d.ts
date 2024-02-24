import type { AlbumRecordType, ArtistRelAlbum, DeezerPaginatedList } from "@/types/deezer";

export declare type DerivedAlbumList = Omit<DeezerPaginatedList<null>, "data"> & {
	data: Record<AlbumRecordType, ArtistRelAlbum[]>;
};
