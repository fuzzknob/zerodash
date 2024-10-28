import { createBrowserRouter } from 'react-router-dom'

import { RootLayout } from './layouts/RootLayout'

import { HomeScreen } from './screens/home/HomeScreen'

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
	},
])
