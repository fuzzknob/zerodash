import { createRequest } from '@zerodash/web-core'

const baseURL = process.env.NEXT_PUBLIC_SERVER_URL

const request = createRequest({
	baseURL,
})

export default request
