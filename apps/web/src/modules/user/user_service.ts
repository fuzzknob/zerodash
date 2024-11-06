import request from '@/services/request'
import type { UserModel } from './user_model'

export async function fetchProfile() {
	const { data } = await request.get<UserModel>('/users/me')
	return data
}
