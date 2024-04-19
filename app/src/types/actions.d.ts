import type { Action } from "svelte/action";

export declare type ExternalHit<T = HTMLElement> = Action<T, { coupling_ids: string[]; close: () => void }>;
