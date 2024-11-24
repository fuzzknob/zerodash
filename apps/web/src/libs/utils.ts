export function redirectToLogin() {
	const HOME_PAGE_URL = import.meta.env.VITE_HOME_PAGE_URL
	window.location.href = `${HOME_PAGE_URL}/login`
}

export function getRandomToken() {
	const uuid = globalThis.crypto.randomUUID()
	return uuid.split('-').join('')
}

export function makeSlug(value: string) {
	const slugged = value
		.trim()
		.replace(/[^a-zA-Z0-9 ]+/gm, '')
		.toLowerCase()
		.split(' ')
		.join('-')
	return `${slugged || 'untitled'}-${getRandomToken().substring(0, 5)}`
}
