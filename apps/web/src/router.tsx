import { createBrowserRouter } from 'react-router-dom'

import { RootLayout } from './layouts/RootLayout'

import { HomeScreen } from './screens/home/HomeScreen'
import { Callback } from './screens/auth/Callback'
import { Logout } from './screens/auth/Logout'

export const router = createBrowserRouter([
	{
		path: '/',
		element: <RootLayout />,
		children: [
			{
				path: '',
				element: <HomeScreen />,
			},
		],
	},
	{
		path: '/callback',
		element: <Callback />,
	},
	{
		path: '/logout',
		element: <Logout />,
	},
])
