import { Outlet } from 'react-router-dom'

import { MantineProvider, createTheme } from '@mantine/core'

import '@mantine/core/styles.css'

const theme = createTheme({
	primaryColor: 'dark',
})

export const RootLayout = () => {
	return (
		<main>
			<MantineProvider theme={theme} defaultColorScheme="light">
				<Outlet />
			</MantineProvider>
		</main>
	)
}
