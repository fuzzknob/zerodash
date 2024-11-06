import { atom } from 'jotai'

import type { UserModel } from './user_model'
import { store } from '@/services/store'
import { fetchProfile } from './user_service'

export const userProvider = atom<UserModel>()

export async function getUser() {
	const user = await fetchProfile()
	store.set(userProvider, () => user)
}
