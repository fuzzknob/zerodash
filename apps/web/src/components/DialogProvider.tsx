import { dialogProvider } from '@/modules/dialog/dialog_provider'
import { Button, Modal } from '@mantine/core'
import { useAtom } from 'jotai'

export const DialogProvider = () => {
	const [dialog, setDialog] = useAtom(dialogProvider)

	function close() {
		setDialog(undefined)
	}

	return (
		<Modal
			title={dialog?.title ?? 'Confirmation'}
			onClose={close}
			radius="lg"
			opened={!!dialog}
			centered
		>
			{dialog?.message ?? ''}
			<div className="mt-5 flex justify-end gap-2">
				<Button
					size="xs"
					variant="subtle"
					radius="md"
					onClick={() => {
						dialog?.onDismiss?.()
						close()
					}}
				>
					{dialog?.dismissLabel ?? 'Cancel'}
				</Button>
				<Button
					radius="md"
					size="xs"
					onClick={() => {
						dialog?.onSuccess()
						close()
					}}
				>
					{dialog?.successLabel ?? 'Confirm'}
				</Button>
			</div>
		</Modal>
	)
}
