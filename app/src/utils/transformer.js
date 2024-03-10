/**
 * @template {({ [key: string] : unknown })} T
 *
 * @param {T} source
 * @param {Array<[keyof T, "LOCALE_DATE_STRING"]>} key_type_map
 *
 * @returns void
 */
export function transform_to_map(source, key_type_map) {
	for (let i = 0; i < key_type_map.length; i++) {
		const [key, type] = key_type_map[i];
		const val = source[key];

		switch (type) {
			case "LOCALE_DATE_STRING":
				if (typeof val !== "string") return;

				// @ts-ignore
				source[key] = new Date(val).toLocaleDateString();
				break;
		}
	}
}
