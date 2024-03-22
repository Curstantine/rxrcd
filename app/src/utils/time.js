export function secondsToFormattedDuration(seconds) {
	const hours = Math.floor(seconds / 3600);
	const minutes = Math.floor((seconds % 3600) / 60);
	const remainingSeconds = seconds % 60;

	const minutesString = minutes.toString().padStart(2, "0");
	const secondsString = remainingSeconds.toString().padStart(2, "0");

	return `${hours > 0 ? hours + ":" : ""}${minutesString}:${secondsString}`;
}
