import type { Writable } from "svelte/store";

export declare type Theme = "system" | "light" | "dark";
export declare type WritableTheme = Writable<Theme | null>;
