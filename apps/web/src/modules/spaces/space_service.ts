import request from '@/services/request'
import type { SpaceModel } from './space_model'

export async function fetchSpaces() {
	const { data } = await request.get<SpaceModel[]>('/spaces')
	return data
}
