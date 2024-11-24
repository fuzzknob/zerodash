import {Menu} from '@mantine/core'
import {useAtom} from 'jotai'
import {HiArrowLeftStartOnRectangle, HiEllipsisVertical, HiOutlineUser,} from 'react-icons/hi2'
import {Link} from 'react-router-dom'

import {userProvider} from '@/modules/user'
import {ProfilePicture} from '../ProfilePicture'

export const Profile = () => {
	const [user] = useAtom(userProvider)
	if (user == null) return <div />

	return (
		<Menu width={170} position="right-end" radius="md">
			<Menu.Target>
				<button
					type="button"
					className="flex items-center justify-between rounded-xl p-2 hover:bg-slate-100"
				>
					<div className="flex items-center gap-2">
						<ProfilePicture profileUrl={user.profile} />
						<div>
							<h6 className="text-sm">{user.name}</h6>
							<p className="text-slate-500 text-xs">@{user.username}</p>
						</div>
					</div>
					<HiEllipsisVertical size={23} />
				</button>
			</Menu.Target>
			<Menu.Dropdown>
				<Menu.Item leftSection={<HiOutlineUser />}>
					<p className="text-xs">View Profile</p>
				</Menu.Item>
				<Menu.Item leftSection={<HiArrowLeftStartOnRectangle />}>
					<Link to="logout">
						<p className="text-xs">Logout</p>
					</Link>
				</Menu.Item>
			</Menu.Dropdown>
		</Menu>
	)
}
