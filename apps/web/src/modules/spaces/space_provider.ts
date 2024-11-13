import { atom } from 'jotai'

import { store } from '@/services/store'
import type { SpaceModel } from './space_model'
import { fetchSpaces } from './space_service'

export const spacesProvider = atom<SpaceModel[]>()

export async function getSpaces() {
	try {
		const spaces = await fetchSpaces()
		store.set(spacesProvider, () => spaces)
	} catch (_) {
		// todo: add sentry capture
	}
}
