import {useEffect, useState} from 'react'
import {Button, Modal} from '@mantine/core'
import {notifications} from '@mantine/notifications'
import {IoPlanetOutline} from 'react-icons/io5'
import {EditorContent, useEditor} from '@tiptap/react'
import LinkExtension from '@tiptap/extension-link'
import Placeholder from '@tiptap/extension-placeholder'
import StarterKit from '@tiptap/starter-kit'

import {createSpaceProvider, updateSpaceProvider} from '@/modules/spaces'
import {AppException} from '@zerodash/web-core'
import {TitleInput} from '@/components/ui/TitleInput'
import type {SpaceModel} from '@/modules/spaces/space_model'

type SpaceModalProps = {
	space?: SpaceModel
	opened: boolean
	close: () => void
}

export const SpaceModal = ({ opened, close, space }: SpaceModalProps) => {
	const [spaceName, setSpaceName] = useState(space?.name ?? '')
	const [isSubmitting, setSubmitting] = useState(false)
	const isEditMode = !!space

	const editor = useEditor({
		content: space?.description ?? '',
		extensions: [
			StarterKit,
			Placeholder.configure({
				placeholder: 'Add description for the space in markdown...',
			}),
			LinkExtension.configure({
				openOnClick: true,
				autolink: true,
				linkOnPaste: true,
			}),
		],
	})

	useEffect(() => {
		setSpaceName(space?.name ?? '')
		editor?.commands.setContent(space?.description ?? '')
	}, [space, editor])

	async function handleSubmit() {
		setSubmitting(true)
		try {
			if (isEditMode) {
				await updateSpaceProvider(space.id, {
					name: spaceName,
					description: editor?.getHTML(),
				})
			} else {
				await createSpaceProvider({
					name: spaceName,
					description: editor?.getHTML(),
				})
			}
			handleClose()
		} catch (e) {
			let errorMessage = 'There was an error while creating space'
			if (e instanceof AppException) {
				errorMessage = e.message
			}
			notifications.show({
				color: 'red',
				title: 'Oops',
				message: errorMessage,
			})
		} finally {
			setSubmitting(false)
		}
	}

	function handleClose() {
		if (!isEditMode) {
			// Clears out fields after modal close animation is done
			setTimeout(() => {
				setSpaceName('')
				editor?.commands.setContent('')
			}, 200)
		}
		close()
	}

	return (
		<Modal
			title={
				<span className="flex gap-1 rounded-lg bg-zerodash-gradient px-2 py-1">
					<IoPlanetOutline />
					<span className="font-bold">
						{isEditMode ? 'Edit space' : 'New space'}
					</span>
				</span>
			}
			size="xl"
			radius="lg"
			opened={opened}
			onClose={handleClose}
			centered
			closeOnEscape={false}
		>
			<form>
				<div className="mb-4 flex flex-col gap-3">
					<TitleInput
						placeholder="Space name"
						value={spaceName}
						onChange={setSpaceName}
					/>
					<EditorContent className="h-24" editor={editor} />
				</div>
				<div className="flex justify-end gap-2 border-t pt-4">
					<Button radius="md" variant="subtle" size="xs" onClick={handleClose}>
						Cancel
					</Button>
					<Button
						size="xs"
						radius="md"
						disabled={!spaceName}
						onClick={handleSubmit}
						loading={isSubmitting}
					>
						{isEditMode ? 'Save space' : 'Create space'}
					</Button>
				</div>
			</form>
		</Modal>
	)
}
