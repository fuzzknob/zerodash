import axios, {
	type CreateAxiosDefaults as RequestOption,
	AxiosError as RequestError,
} from 'axios'
import {
	AppException,
	NotFoundException,
	UnAuthenticatedException,
	ValidationException,
} from './exception'

export function createRequest(options: RequestOption) {
	const request = axios.create(options)
	request.interceptors.response.use(
		(response) => response,
		(error) => {
			if (!(error instanceof RequestError) || !error.response) {
				throw new AppException(
					'There was an error while connecting to the server',
				)
			}
			const { response } = error
			if (response.status === 401) {
				throw new UnAuthenticatedException()
			}
			if (response.status === 404) {
				throw new NotFoundException(response.data.message)
			}
			if (response.status === 400) {
				const data: {
					message: string
					errors: Record<string, string[]> | undefined
				} = response.data
				throw new ValidationException(data.message, data.errors)
			}
			throw new AppException('There was an error from the server')
		},
	)
	return request
}

export { AxiosError as RequestError } from 'axios'
