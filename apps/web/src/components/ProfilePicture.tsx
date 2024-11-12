import { HiOutlineUser } from 'react-icons/hi2'

export type ProfileProps = {
	profileUrl?: string
}

export const ProfilePicture = ({ profileUrl }: ProfileProps) => {
	return (
		<div className="flex h-[35px] w-[35px] flex-center rounded-full bg-slate-100">
			{profileUrl ? (
				<img
					className="w-full rounded-full"
					src={profileUrl}
					alt="profile-picture"
				/>
			) : (
				<HiOutlineUser />
			)}
		</div>
	)
}
