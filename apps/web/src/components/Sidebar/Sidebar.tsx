import { ActionIcon } from '@mantine/core'
import { clsx } from 'clsx'
import { useAtom } from 'jotai'
import {
	// HiBars3,
	HiOutlineCalendarDays,
	HiOutlineHome,
	HiPlus,
} from 'react-icons/hi2'
import { Link, useLocation } from 'react-router-dom'

import { spacesProvider } from '@/modules/spaces'

import Logo from '../Logo'
import { Profile } from './Profile'
import { Space } from './Space'

type SidebarItemProps = {
	to: string
	icon: React.ReactNode
	children: React.ReactNode
}

const SidebarItem = ({ to, icon, children }: SidebarItemProps) => {
	const { pathname } = useLocation()
	const active = pathname === to
	const classes = clsx(
		active && 'bg-slate-100',
		'flex items-center gap-2 rounded-lg px-3 py-2 hover:bg-slate-200',
	)

	return (
		<Link to={to} className={classes}>
			{icon}
			<div className="text-sm">{children}</div>
		</Link>
	)
}

export const Sidebar = () => {
	const [spaces] = useAtom(spacesProvider)

	return (
		<div className="flex h-full flex-col rounded-xl bg-slate-300 px-3 py-4">
			<div className="flex items-center justify-between px-2">
				<Logo />
				{/* <ActionIcon variant="light">
					<HiBars3 size={20} />
				</ActionIcon> */}
			</div>
			<div className="flex flex-col gap-2 py-6">
				<SidebarItem to="/" icon={<HiOutlineHome size={19} />} active>
					My day
				</SidebarItem>
				<SidebarItem to="/calendar" icon={<HiOutlineCalendarDays size={19} />}>
					Calendar
				</SidebarItem>
			</div>
			<div className="mb-2 flex justify-between px-2 ">
				<h6 className="text-sm">Spaces</h6>
				<ActionIcon variant="light" size="xs">
					<HiPlus />
				</ActionIcon>
			</div>
			<div className="flex flex-1 flex-col gap-2 overflow-y-auto">
				{spaces?.map((space) => (
					<Space key={space.id} space={space} initialExpanded={space.primary} />
				))}
			</div>
			<Profile />
		</div>
	)
}
