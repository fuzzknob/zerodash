import { MantineProvider, createTheme } from '@mantine/core'
import { Notifications } from '@mantine/notifications'
import { Provider as JotaiProvider } from 'jotai'
import { Outlet } from 'react-router-dom'
import '@mantine/core/styles.css'
import '@/styles/tiptap.css'

import { DialogProvider } from '@/components/DialogProvider'
import { store } from '@/services/store'

const theme = createTheme({
	primaryColor: 'dark',
})

export const RootLayout = () => {
	return (
		<main>
			<JotaiProvider store={store}>
				<MantineProvider theme={theme} defaultColorScheme="light">
					<Notifications />
					<DialogProvider />
					<Outlet />
				</MantineProvider>
			</JotaiProvider>
		</main>
	)
}
