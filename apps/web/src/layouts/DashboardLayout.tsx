import { Outlet } from 'react-router-dom'

import { Sidebar } from '@/components/Sidebar'

export const DashboardLayout = () => {
	return (
		<div className="relative flex h-screen">
			<div className="w-full max-w-[300px] p-5">
				<Sidebar />
			</div>
			<main className="w-full p-6">
				<Outlet />
			</main>
		</div>
	)
}
