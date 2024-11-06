export function getEnvValue(
	key: string,
	options = { isOptional: false },
): string {
	// @ts-ignore
	const value = process.env[key]
	if (!value && !options.isOptional) {
		throw new Error(`${key} is required`)
	}
	return value || ''
}
