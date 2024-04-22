import type { Writable } from "svelte/store";

import type { Theme } from "@/bindings/config";

export declare type WritableTheme = Writable<Theme | null>;
