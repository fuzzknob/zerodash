import {atom} from 'jotai'

import {getRandomToken, makeSlug} from '@/libs/utils'
import {store} from '@/services/store'
import type {SpaceModel} from './space_model'
import {createSpace, deleteSpace, fetchSpaces, updateSpace,} from './space_service'

export const spacesProvider = atom<SpaceModel[]>()

export async function getSpacesProvider() {
	const spaces = await fetchSpaces()
	store.set(spacesProvider, () => spaces)
}

export async function createSpaceProvider(data: {
	name: string
	description?: string
}) {
	const tempId = `temp-${getRandomToken()}`
	const slug = makeSlug(data.name)
	const newSpace: SpaceModel = {
		id: tempId,
		boards: [],
		primary: false,
		slug,
		...data,
	}
	// optimistic update
	store.set(spacesProvider, (spaces) => spaces?.concat([newSpace]))
	createSpace({
		...data,
		slug,
	}).then(({ id }) => {
		store.set(spacesProvider, (spaces) =>
			spaces?.map((space) => {
				if (space.id === tempId) {
					return {
						...space,
						id,
					}
				}
				return space
			}),
		)
	})
}

export async function updateSpaceProvider(
	id: string,
	data: {
		name: string
		description?: string
	},
) {
	const slug = makeSlug(data.name)
	store.set(spacesProvider, (spaces) =>
		spaces?.map((space) => {
			if (space.id === id) {
				return {
					...space,
					...data,
					slug,
				}
			}
			return space
		}),
	)
	updateSpace(id, {
		...data,
		slug,
	}).then()
}

export function deleteSpaceProvider(spaceId: string) {
	deleteSpace(spaceId).then()
	store.set(spacesProvider, (spaces) =>
		spaces?.filter((space) => space.id !== spaceId),
	)
}
