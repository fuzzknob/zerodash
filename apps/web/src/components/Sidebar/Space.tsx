import { ActionIcon } from '@mantine/core'
import { clsx } from 'clsx'
import { useState } from 'react'
import {
	HiOutlineChevronRight,
	HiOutlineViewColumns,
	HiPlus,
} from 'react-icons/hi2'
import { IoPlanetOutline } from 'react-icons/io5'
import { Link } from 'react-router-dom'

export type SpaceProps = {
	initialExpanded?: boolean
}

export const Space = ({ initialExpanded = false }: SpaceProps) => {
	const [expanded, setExpanded] = useState(initialExpanded)

	return (
		<div>
			<Link
				to="/personal"
				className="mb-1 flex items-center justify-between rounded-lg px-2 py-1 hover:bg-slate-200"
			>
				<div className="flex items-center gap-2">
					<IoPlanetOutline />
					<h6 className="text-sm">Gagan's space</h6>
				</div>
				<div className="flex items-center gap-2">
					<ActionIcon
						onClick={(e) => e.preventDefault()}
						size="xs"
						variant="light"
					>
						<HiPlus />
					</ActionIcon>
					<ActionIcon
						onClick={(e) => {
							e.preventDefault()
							setExpanded((expanded) => !expanded)
						}}
						size="xs"
						variant="light"
						className={clsx('transition', { 'rotate-90': expanded })}
					>
						<HiOutlineChevronRight />
					</ActionIcon>
				</div>
			</Link>
			{expanded && (
				<div className="flex flex-col gap-1">
					<Link
						to="/task-id"
						className="flex items-center gap-2 rounded-lg px-5 py-1 hover:bg-slate-200"
					>
						<HiOutlineViewColumns />
						<h6 className="text-sm">Personal</h6>
					</Link>
					<Link
						to="/task-id"
						className="flex items-center gap-2 rounded-lg px-5 py-1 hover:bg-slate-200"
					>
						<HiOutlineViewColumns />
						<h6 className="text-sm">Reminders</h6>
					</Link>
				</div>
			)}
		</div>
	)
}
