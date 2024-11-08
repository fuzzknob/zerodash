import { redirectToLogin } from '@/libs/utils'
import { logout } from '@/modules/auth/auth_service'
import { useEffect } from 'react'

export const Logout = () => {
	useEffect(() => {
		logout().finally(() => {
			redirectToLogin()
		})
	}, [])
	return <div />
}
