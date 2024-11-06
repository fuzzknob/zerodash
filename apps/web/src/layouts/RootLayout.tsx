import { Outlet } from 'react-router-dom'
import { MantineProvider, createTheme } from '@mantine/core'
import { Provider as JotaiProvider } from 'jotai'
import '@mantine/core/styles.css'

import { store } from '@/services/store'

const theme = createTheme({
	primaryColor: 'dark',
})

export const RootLayout = () => {
	return (
		<main>
			<JotaiProvider store={store}>
				<MantineProvider theme={theme} defaultColorScheme="light">
					<Outlet />
				</MantineProvider>
			</JotaiProvider>
		</main>
	)
}
