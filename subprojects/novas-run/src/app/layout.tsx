import './globals.css'

export const metadata = {
	title: `Nova's Run`,
}

export default function RootLayout({children}: { children: React.ReactNode })
{
	return (
		<html lang="en">
			<body>{children}</body>
		</html>
	)
}
