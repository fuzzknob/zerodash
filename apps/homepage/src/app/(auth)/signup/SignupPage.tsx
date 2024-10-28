'use client'
import { useState } from 'react'

import { z } from 'zod'
import { useForm } from '@mantine/form'
import { zodResolver } from 'mantine-form-zod-resolver'
import { Button, PasswordInput, TextInput } from '@mantine/core'

import Link from 'next/link'
import Logo from '@/components/Logo'

const formSchema = z
	.object({
		name: z.string().trim().min(3),
		email: z.string().trim().email(),
		username: z
			.string()
			.trim()
			.min(4, 'Username must be 4 characters long')
			.regex(
				/^[a-zA-Z0-9]+$/gm,
				'Username must not contain special characters',
			),
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

const SignupPage = () => {
	const [passwordVisible, setPasswordVisible] = useState(false)
	const form = useForm({
		mode: 'uncontrolled',
		initialValues: {
			name: '',
			email: '',
			username: '',
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
			<div className="mb-12 self-center">
				<Logo />
			</div>
			<div className="flex flex-col gap-6">
				<TextInput
					placeholder="Your name"
					key={form.key('name')}
					{...form.getInputProps('name')}
				/>
				<TextInput
					placeholder="Your email address"
					key={form.key('email')}
					{...form.getInputProps('email')}
				/>
				<TextInput
					placeholder="Pick a username"
					key={form.key('username')}
					{...form.getInputProps('username')}
				/>
				<PasswordInput
					placeholder="Password"
					visible={passwordVisible}
					onVisibilityChange={(visible) => setPasswordVisible(visible)}
					key={form.key('password')}
					{...form.getInputProps('password')}
				/>
				<PasswordInput
					placeholder="Repeat Password"
					visible={passwordVisible}
					onVisibilityChange={(visible) => setPasswordVisible(visible)}
					key={form.key('confirm')}
					{...form.getInputProps('confirm')}
				/>
			</div>

			<Button className="mt-8" type="submit" fullWidth>
				Sign Up
			</Button>
			<div className="mt-4 self-center text-sm">
				Already have an account?{' '}
				<Link className="text-blue-700" href="/login">
					Log in
				</Link>
			</div>
		</form>
	)
}

export default SignupPage
