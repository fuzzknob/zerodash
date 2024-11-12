'use client'
import { useState } from 'react'
import { z } from 'zod'
import { zodResolver } from 'mantine-form-zod-resolver'

import { Button, TextInput } from '@mantine/core'
import Logo from '@/components/Logo'
import { useForm } from '@mantine/form'
import Link from 'next/link'

const formSchema = z.object({
	email: z.string().email('Email is required to send'),
})

const ResetPassword = () => {
	const [hasSent, setSent] = useState(false)
	const form = useForm({
		mode: 'uncontrolled',
		initialValues: {
			email: '',
		},
		validate: zodResolver(formSchema),
	})

	return (
		<div className="flex w-[350px] flex-col">
			<div className="self-center mb-10">
				<Logo />
			</div>
			{!hasSent ? (
				<form
					onSubmit={form.onSubmit((values) => {
						form.validate()
						console.log(values)
						setSent(true)
					})}
				>
					<h4 className="text-2xl font-semibold">Reset your password</h4>
					<p className="mb-3 text-sm text-gray-600">
						We'll send you a reset password link for your account if it exists
					</p>
					<TextInput className="mb-8" placeholder="Enter your email address" />
					<Button fullWidth>Send reset link</Button>
				</form>
			) : (
				<div>
					We've sent you a reset link check your inbox to reset your password
				</div>
			)}
			<div className="self-center mt-4 text-sm">
				<Link className="text-blue-700" href="/login">
					Go back to Login
				</Link>
			</div>
		</div>
	)
}

export default ResetPassword
