export interface UserModel {
	id: string
	name: string
	username: string
	email: string
	profile?: string
	email_verified_at: string | null
}
