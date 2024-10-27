import type { Metadata } from 'next'
import { Inter } from 'next/font/google'
import { ColorSchemeScript, createTheme, MantineProvider } from '@mantine/core'
import clsx from 'clsx'
import './globals.css'
import '@mantine/core/styles.css'

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
			<body className="h-screen w-screen bg-background-primary antialiased">
				<MantineProvider theme={theme} defaultColorScheme="light">
					{children}
				</MantineProvider>
			</body>
		</html>
	)
}
