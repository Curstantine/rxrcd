export declare interface Action<T extends string = string> {
	label: string;
	value: string;
	sub?: string;
}
