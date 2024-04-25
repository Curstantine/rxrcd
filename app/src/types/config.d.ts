export declare interface Configuration {
	appearance: ConfigurationAppearance;
	download: ConfigurationDownload;
}

export declare type Theme = "system" | "light" | "dark";
export declare interface ConfigurationAppearance {
	theme: Theme;
}

export declare type DownloadQuality = "Flac" | "Mp3_320" | "Mp3_128";
export declare type CoverQuality = "Small" | "Medium" | "Big" | "Xl";
export declare interface ConfigurationDownload {
	concurrent: number;
	quality: DownloadQuality;
	path: string;

	save_covers: boolean;
	embed_covers: boolean;
	cover_resolution: CoverQuality;
	cover_embed_resolution: CoverQuality;
}
