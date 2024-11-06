import request from '@/services/request'

export async function login(identity: string, password: string) {
	const { data } = await request.post<{ exchange_token: string }>(
		'auth/login',
		{
			identity,
			password,
		},
	)
	return data.exchange_token
}
