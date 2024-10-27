'use client'
import Logo from '@/components/Logo'
import { useForm } from '@mantine/form'
import { z } from 'zod'
import { zodResolver } from 'mantine-form-zod-resolver'
import { Button, PasswordInput, TextInput } from '@mantine/core'
import Link from 'next/link'

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
			className="flex w-[300px] flex-col"
		>
			<div className="mb-10 self-center">
				<Logo />
			</div>
			<div className="flex flex-col gap-6">
				<TextInput placeholder="Username or Email address" />
				<PasswordInput placeholder="Password" />
			</div>
			<Link
				className="mt-1 mb-4 self-end text-blue-700 text-sm"
				href="/forget-password"
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
