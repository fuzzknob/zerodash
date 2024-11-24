import { getSpacesProvider } from '@/modules/spaces'
import { getUser } from '@/modules/user'
import { useEffect, useState } from 'react'

export const InitializationLayer = ({
	children,
}: { children: React.ReactNode }) => {
	const [loaded, setLoaded] = useState(false)
	useEffect(() => {
		Promise.all([getUser(), getSpacesProvider()]).then(() => {
			setLoaded(true)
		})
	}, [])

	if (!loaded) {
		return <div />
	}
	return <>{children}</>
}
