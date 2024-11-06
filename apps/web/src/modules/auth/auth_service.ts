import request from '@/services/request'

export async function loginWithToken(token: string) {
	await request.get('/auth/login-with-token', {
		params: {
			token,
		},
	})
}
