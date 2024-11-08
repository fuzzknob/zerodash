import { redirectToLogin } from '@/libs/utils'
import { createRequest, UnAuthenticatedException } from '@zerodash/web-core'

const baseURL = import.meta.env.VITE_SERVER_URL

const request = createRequest({
	baseURL,
	withCredentials: true,
})

request.interceptors.response.use(
	(response) => response,
	(error) => {
		if (error instanceof UnAuthenticatedException) {
			redirectToLogin()
			throw new UnAuthenticatedException()
		}
	},
)

export * from '@zerodash/web-core/request'
export default request
