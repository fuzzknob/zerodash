'use client'
import { z } from 'zod'
import { zodResolver } from 'mantine-form-zod-resolver'
import { Button, PasswordInput, TextInput } from '@mantine/core'
import { useForm } from '@mantine/form'
import Link from 'next/link'

import Logo from '@/components/Logo'

const formSchema = z.object({
	identity: z.string(),
	password: z.string(),
})

const LoginPage = () => {
	const form = useForm({
		mode: 'uncontrolled',
		initialValues: {
			identity: '',
			password: '',
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
					placeholder="Username or Email address"
					key={form.key('identity')}
					{...form.getInputProps('identity')}
				/>
				<PasswordInput
					placeholder="Password"
					key={form.key('password')}
					{...form.getInputProps('password')}
				/>
			</div>
			<Link
				className="mt-1 mb-4 self-end text-blue-700 text-sm"
				href="/reset-password"
			>
				Forget password?
			</Link>
			<Button type="submit" fullWidth>
				Login
			</Button>
			<div className="mt-4 self-center text-sm">
				New to Zerodash?{' '}
				<Link className="text-blue-700" href="/signup">
					Sign up
				</Link>
			</div>
		</form>
	)
}

export default LoginPage
