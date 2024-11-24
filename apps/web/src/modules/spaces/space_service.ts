import request from '@/services/request'
import type {SpaceModel} from './space_model'

export async function fetchSpaces() {
	const { data } = await request.get<SpaceModel[]>('/spaces')
	return data
}

export async function createSpace({
	name,
	slug,
	description,
}: {
	name: string
	slug: string
	description?: string
}) {
	const { data } = await request.post<SpaceModel>('/spaces', {
		name,
		slug,
		description,
	})
	return {
		...data,
		boards: [],
	} as SpaceModel
}

export async function updateSpace(
	spaceId: string,
	{
		name,
		slug,
		description,
	}: {
		name: string
		slug: string
		description?: string
	},
) {
	const { data } = await request.patch<SpaceModel>(`/spaces/${spaceId}`, {
		name,
		slug,
		description,
	})
	return data
}

export async function deleteSpace(spaceId: string) {
	await request.delete(`/spaces/${spaceId}`)
}
