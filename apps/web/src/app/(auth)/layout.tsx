export default function RootLayout({
	children,
}: Readonly<{
	children: React.ReactNode
}>) {
	return <div className="h-full w-full flex-center">{children}</div>
}
