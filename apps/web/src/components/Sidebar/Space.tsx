import { ActionIcon, Menu } from '@mantine/core'
import { clsx } from 'clsx'
import { useState } from 'react'
import {
	HiEllipsisVertical,
	HiOutlineChevronRight,
	HiOutlinePencilSquare,
	HiOutlineTrash,
	HiOutlineViewColumns,
	HiPlus,
} from 'react-icons/hi2'
import { IoPlanetOutline } from 'react-icons/io5'
import { Link } from 'react-router-dom'

import type { SpaceModel } from '@/modules/spaces/space_model'

const SpaceMenu = ({ space }: { space: SpaceModel }) => {
	return (
		<Menu width={170} position="right-start">
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
				<Menu.Item leftSection={<HiOutlinePencilSquare />}>
					<p className="text-xs">Edit Space</p>
				</Menu.Item>
				<Menu.Item
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
}

export const Space = ({ space, initialExpanded = false }: SpaceProps) => {
	const [expanded, setExpanded] = useState(initialExpanded)

	return (
		<div>
			<button
				type="button"
				className="mb-1 flex w-full items-center justify-between rounded-lg px-2 py-1 hover:bg-slate-200"
			>
				<button
					type="button"
					onClick={() => setExpanded((expanded) => !expanded)}
					className="flex flex-1 items-center gap-2"
				>
					<IoPlanetOutline />
					<h6 className="text-sm">{space.name}</h6>
					<div className={clsx('transition', { 'rotate-90': expanded })}>
						<HiOutlineChevronRight />
					</div>
				</button>
				<div className="flex items-center gap-2">
					<SpaceMenu space={space} />
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
			</button>
			{expanded && (
				<div className="flex flex-col gap-1">
					{space.boards.map((board) => (
						<Link
							key={board.id}
							to="/task-id"
							className="flex items-center gap-2 rounded-lg px-5 py-1 hover:bg-slate-200"
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
