type TitleInputProps = {
	placeholder?: string
	value: string
	onChange: (value: string) => void
}

export const TitleInput = ({
	placeholder = '',
	value,
	onChange,
}: TitleInputProps) => {
	return (
		<input
			value={value}
			onChange={(e) => onChange(e.target.value)}
			className="font-semibold text-xl outline-none"
			placeholder={placeholder}
		/>
	)
}
