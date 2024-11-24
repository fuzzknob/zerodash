import { atom, useAtom } from 'jotai'

interface DialogOption {
	title?: string
	message: string
	successLabel?: string
	dismissLabel?: string
	onSuccess: () => void
	onDismiss?: () => void
}

export const dialogProvider = atom<DialogOption>()

export const useDialog = () => {
	const [_, setDialog] = useAtom(dialogProvider)
	return (dialog: DialogOption) => {
		setDialog(dialog)
	}
}
