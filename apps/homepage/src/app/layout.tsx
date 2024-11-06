import type { Metadata } from 'next'
import { Inter } from 'next/font/google'
import { ColorSchemeScript, createTheme, MantineProvider } from '@mantine/core'
import clsx from 'clsx'
import { Notifications } from '@mantine/notifications'
import '@mantine/core/styles.css'
import '@mantine/notifications/styles.css'

import './globals.css'

const inter = Inter({
	weight: ['300', '400', '500'],
	subsets: ['latin'],
	variable: '--font-inter',
})

export const metadata: Metadata = {
	title: 'Zerodash',
	description: 'The only dashboard you’ll ever need',
}

const theme = createTheme({
	primaryColor: 'dark',
})

export default function RootLayout({
	children,
}: Readonly<{
	children: React.ReactNode
}>) {
	return (
		<html lang="en" className={clsx(inter.variable)}>
			<head>
				<ColorSchemeScript />
			</head>
			<body>
				<MantineProvider theme={theme} defaultColorScheme="light">
					<Notifications />
					<main className="w-screen h-screen antialiased bg-background-primary">
						{children}
					</main>
				</MantineProvider>
			</body>
		</html>
	)
}
