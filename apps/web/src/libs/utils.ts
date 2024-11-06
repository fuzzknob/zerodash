export function redirectToLogin() {
	const HOME_PAGE_URL = import.meta.env.VITE_HOME_PAGE_URL
	window.location.href = `${HOME_PAGE_URL}/login`
}
