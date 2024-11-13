import { createBrowserRouter } from 'react-router-dom'

import { RootLayout } from './layouts/RootLayout'

import { DashboardLayout } from './layouts/DashboardLayout'
import { Callback } from './screens/auth/Callback'
import { Logout } from './screens/auth/Logout'
import { CalendarScreen } from './screens/calendar/CalendarScreen'
import { HomeScreen } from './screens/home/HomeScreen'

export const router = createBrowserRouter([
	{
		path: '/',
		element: <RootLayout />,
		children: [
			{
				path: '',
				element: <DashboardLayout />,
				children: [
					{
						path: '',
						element: <HomeScreen />,
					},
					{
						path: 'calendar',
						element: <CalendarScreen />,
					},
				],
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
