'use client'
import { useState } from 'react'

import { z } from 'zod'
import { useForm } from '@mantine/form'
import { zodResolver } from 'mantine-form-zod-resolver'
import { Button, PasswordInput } from '@mantine/core'

import Logo from '@/components/Logo'

const formSchema = z
	.object({
		password: z
			.string()
			.min(8, 'Password must be at least 8 characters long')
			.regex(
				/^(?=.*[a-z])(?=.*[A-Z])(?=.*\d)(?=.*[!@#\$%\^&\*\(\)_\+-=\{\}\[\]\|\\:;"'<>,\.\?\/~` ])[A-Za-z\d!@#\$%\^&\*\(\)_\+-=\{\}\[\]\|\\:;"'<>,\.\?\/~` ]+$/g,
				'Password must contain at least one special character, one uppercase letter, one lowercase letter and one number',
			),
		confirm: z.string({ message: 'Repeat password is required' }),
	})
	.refine((data) => data.password === data.confirm, {
		message: "Password don't match",
		path: ['confirm'],
	})

const ChangePasswordPage = () => {
	const [passwordVisible, setPasswordVisible] = useState(false)
	const form = useForm({
		mode: 'uncontrolled',
		initialValues: {
			password: '',
			confirm: '',
		},
		validate: zodResolver(formSchema),
	})

	return (
		<form
			onSubmit={form.onSubmit((values) => {
				form.validate()
				console.log(values)
			})}
			className="flex w-[350px] flex-col"
		>
			<div className="mb-6 self-center">
				<Logo />
			</div>
			<h4 className="mb-2 font-semibold text-xl">Set new password</h4>
			<p className="mb-4 text-gray-600 text-sm">
				Set a new password for your <span className="line-through">zero</span>{' '}
				account
			</p>
			<div className="flex flex-col gap-6">
				<PasswordInput
					placeholder="New Password"
					visible={passwordVisible}
					onVisibilityChange={(visible) => setPasswordVisible(visible)}
					key={form.key('password')}
					{...form.getInputProps('password')}
				/>
				<PasswordInput
					placeholder="Repeat New Password"
					visible={passwordVisible}
					onVisibilityChange={(visible) => setPasswordVisible(visible)}
					key={form.key('confirm')}
					{...form.getInputProps('confirm')}
				/>
			</div>
			<Button className="mt-8" type="submit" fullWidth>
				Change password
			</Button>
		</form>
	)
}

export default ChangePasswordPage
