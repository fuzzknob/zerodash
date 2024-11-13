import type { BoardModel } from '../boards/board_model'

export interface SpaceModel {
	id: string
	name: string
	primary: boolean
	boards: BoardModel[]
	description?: string
	icon?: string
}
