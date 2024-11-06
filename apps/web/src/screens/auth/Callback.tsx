import { useEffect } from 'react'
import { useNavigate, useSearchParams } from 'react-router-dom'

import { redirectToLogin } from '@/libs/utils'
import { loginWithToken } from '@/modules/auth/auth_service'

export const Callback = () => {
	const [searchParams] = useSearchParams()
	const navigate = useNavigate()

	useEffect(() => {
		const token = searchParams.get('token')
		if (token) {
			login(token)
		} else {
			redirectToLogin()
		}
	}, [searchParams])

	async function login(token: string) {
		try {
			await loginWithToken(token)
			navigate('/')
		} catch (_) {
			redirectToLogin()
		}
	}

	return (
		<div className="h-screen flex-center">
			<h3 className="text-2xl">Logging In...</h3>
		</div>
	)
}
