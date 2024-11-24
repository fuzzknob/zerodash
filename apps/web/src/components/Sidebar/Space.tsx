import {ActionIcon, Menu} from '@mantine/core'
import {clsx} from 'clsx'
import {
  HiEllipsisVertical,
  HiOutlineChevronRight,
  HiOutlinePencilSquare,
  HiOutlineTrash,
  HiOutlineViewColumns,
  HiPlus,
} from 'react-icons/hi2'
import {useState} from 'react'
import {IoPlanetOutline} from 'react-icons/io5'
import {Link} from 'react-router-dom'

import {useDialog} from '@/modules/dialog/dialog_provider'
import type {SpaceModel} from '@/modules/spaces/space_model'
import {deleteSpaceProvider} from '@/modules/spaces'

type SpaceMenuProps = {
	space: SpaceModel
	onEdit: (space: SpaceModel) => void
	onDelete: (space: SpaceModel) => void
}

const SpaceMenu = ({ space, onEdit, onDelete }: SpaceMenuProps) => {
	return (
		<Menu width={170} position="right-start" radius="md">
			<Menu.Target>
				<ActionIcon
					onClick={(e) => {
						e.preventDefault()
						e.stopPropagation()
					}}
					size="xs"
					variant="light"
				>
					<HiEllipsisVertical />
				</ActionIcon>
			</Menu.Target>
			<Menu.Dropdown>
				<Menu.Item
					onClick={() => onEdit(space)}
					leftSection={<HiOutlinePencilSquare />}
				>
					<p className="text-xs">Edit Space</p>
				</Menu.Item>
				<Menu.Item
					onClick={() => onDelete(space)}
					className="text-red-500"
					leftSection={<HiOutlineTrash />}
					disabled={space.primary}
				>
					<p className="text-xs">Delete Space</p>
				</Menu.Item>
			</Menu.Dropdown>
		</Menu>
	)
}

export type SpaceProps = {
	initialExpanded?: boolean
	space: SpaceModel
	onEdit: (space: SpaceModel) => void
}

export const Space = ({
	space,
	initialExpanded = false,
	onEdit,
}: SpaceProps) => {
	const [expanded, setExpanded] = useState(initialExpanded)

	const openDialog = useDialog()

	function handleDelete() {
		openDialog({
			title: 'Delete space',
			message: 'Are you sure you want to delete space?',
			onSuccess: () => {
				deleteSpaceProvider(space.id)
			},
		})
	}

	return (
		<div>
			<div className="mb-1 flex w-full cursor-pointer items-center justify-between rounded-lg px-2 py-1 hover:bg-slate-200">
				<button
					type="button"
					onClick={() => setExpanded((expanded) => !expanded)}
					className="flex flex-1 items-center gap-2"
				>
					<IoPlanetOutline />
					<h6 className="max-w-[125px] overflow-hidden text-ellipsis whitespace-nowrap text-sm">
						{space.name}
					</h6>
					<div className={clsx('transition', { 'rotate-90': expanded })}>
						<HiOutlineChevronRight />
					</div>
				</button>
				<div className="flex items-center gap-2">
					<SpaceMenu space={space} onEdit={onEdit} onDelete={handleDelete} />
					<ActionIcon
						onClick={(e) => {
							e.preventDefault()
							e.stopPropagation()
						}}
						size="xs"
						variant="light"
					>
						<HiPlus />
					</ActionIcon>
				</div>
			</div>
			{expanded && (
				<div className="flex flex-col gap-1">
					{space.boards.map((board) => (
						<Link
							key={board.id}
							to="/task-id"
							className="flex items-center gap-2 rounded-lg px-5 py-1 hover:bg-slate-100"
						>
							<HiOutlineViewColumns />
							<h6 className="text-sm">{board.name}</h6>
						</Link>
					))}
				</div>
			)}
		</div>
	)
}
