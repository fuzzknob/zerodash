import { Outlet } from 'react-router-dom'

import { Sidebar } from '@/components/Sidebar'
import { InitializationLayer } from '@/layers/initialization_layer'

export const DashboardLayout = () => {
	return (
		<InitializationLayer>
			<div className="relative flex h-screen">
				<div className="w-full max-w-[300px] p-5">
					<Sidebar />
				</div>
				<main className="w-full p-6">
					<Outlet />
				</main>
			</div>
		</InitializationLayer>
	)
}
