import { useAtom } from 'jotai'

import { userProvider } from '@/modules/user'

export const HomeScreen = () => {
	const [user] = useAtom(userProvider)
	return <div>{user?.name}</div>
}
