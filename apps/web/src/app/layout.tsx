import type { Metadata } from 'next'
import { Inter } from 'next/font/google'
import clsx from 'clsx'
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

export default function RootLayout({
	children,
}: Readonly<{
	children: React.ReactNode
}>) {
	return (
		<html lang="en" className={clsx(inter.variable)}>
			<body className="bg-background-primary h-screen w-screen antialiased">
				{children}
			</body>
		</html>
	)
}
