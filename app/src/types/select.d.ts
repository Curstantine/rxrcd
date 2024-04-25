export declare interface Action<T extends string = string> {
	value: T;
	label: string;
	sub?: string;
}
