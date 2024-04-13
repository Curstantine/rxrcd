import type { Readable, Writable } from "svelte/store";

export declare type SnackInstanceUpdater = (arg0: Pick<SnackInstance, "label" | "description">) => void;

export declare interface SnackInstance {
	label: string;
	description?: string;
	persistent?: boolean;
}

export declare interface InstanceMapEntry {
	timeout: number | undefined;
	inner: Writable<SnackInstance>;
}
