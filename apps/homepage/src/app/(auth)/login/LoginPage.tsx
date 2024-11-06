'use client'
import { z } from 'zod'
import { zodResolver } from 'mantine-form-zod-resolver'
import { Button, PasswordInput, TextInput } from '@mantine/core'
import { useForm } from '@mantine/form'
import { notifications } from '@mantine/notifications'
import Link from 'next/link'

import { AppException, ValidationException } from '@zerodash/web-core'
import { login } from '@/modules/auth/auth_service'

import Logo from '@/components/Logo'
import { useState } from 'react'

const formSchema = z.object({
	identity: z.string().min(1, { message: 'This field cannot be empty' }),
	password: z.string().min(1, { message: 'This field cannot be empty' }),
	// identity: z.string(),
	// password: z.string(),
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
	const [isSubmitting, setSubmitting] = useState(false)

	async function handleLogin(identity: string, password: string) {
		setSubmitting(true)
		try {
			const exchangeToken = await login(identity, password)
			window.location.href = `${process.env.NEXT_PUBLIC_WEB_APP_URL}/callback?token=${exchangeToken}`
		} catch (error) {
			if (error instanceof ValidationException) {
				if (error.errors) form.setErrors(error.errors)
			}
			if (error instanceof AppException) {
				notifications.show({
					color: 'red',
					title: 'Oops',
					message: error.message,
				})
			}
		} finally {
			setSubmitting(false)
		}
	}

	return (
		<form
			onSubmit={form.onSubmit((values) => {
				handleLogin(values.identity, values.password)
			})}
			className="flex w-[350px] flex-col"
		>
			<div className="self-center mb-12">
				<Logo />
			</div>
			<div className="flex flex-col gap-6">
				<TextInput
					placeholder="Username or Email address"
					{...form.getInputProps('identity')}
				/>
				<PasswordInput
					placeholder="Password"
					{...form.getInputProps('password')}
				/>
			</div>
			<Link
				className="self-end mt-1 mb-4 text-sm text-blue-700"
				href="/reset-password"
			>
				Forget password?
			</Link>
			<Button type="submit" loading={isSubmitting} fullWidth>
				Login
			</Button>
			<div className="self-center mt-4 text-sm">
				New to Zerodash?{' '}
				<Link className="text-blue-700" href="/signup">
					Sign up
				</Link>
			</div>
		</form>
	)
}

export default LoginPage
