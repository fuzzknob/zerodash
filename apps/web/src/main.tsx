import { StrictMode } from 'react'
import { createRoot } from 'react-dom/client'
import { RouterProvider } from 'react-router-dom'

import { router } from './router'

import './index.css'

function initialize() {
	const root = document.getElementById('root')
	if (!root) {
		document.write('id with root element not found')
		return
	}
	createRoot(root).render(
		<StrictMode>
			<RouterProvider router={router} />
		</StrictMode>,
	)
}

initialize()
