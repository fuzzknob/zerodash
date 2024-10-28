import { cva, type VariantProps } from 'class-variance-authority'

const buttonStyles = cva('button', {
	variants: {
		variant: {
			primary: [''],
		},
		size: {
			sm: [],
		},
	},
	defaultVariants: {
		size: 'sm',
		variant: 'primary',
	},
})

type ButtonProps = React.ButtonHTMLAttributes<HTMLButtonElement> &
	VariantProps<typeof buttonStyles>

export const Button = ({ variant, size, className, ...props }: ButtonProps) => {
	return (
		<button className={buttonStyles({ variant, size, className })} {...props}>
			Hello
		</button>
	)
}
