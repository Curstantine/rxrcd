/**
 * @param {string} msg
 * @param {URL | null} url
 * @returns {never}
 */
export const fetch_error = (msg, url) => {
	throw new Error(`${msg}\n\tAt URL: ${url?.toString() ?? "N/A"}`);
};
