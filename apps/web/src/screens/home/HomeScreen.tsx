import { useEffect } from 'react'
import { useAtom } from 'jotai'

import { getUser, userProvider } from '@/modules/user'

export const HomeScreen = () => {
	const [user] = useAtom(userProvider)
	useEffect(() => {
		getUser()
	}, [])
	return <div>{user?.name}</div>
}
