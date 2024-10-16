'use client'

export default function Modal() {
	return (
		<div className="fixed inset-0 flex-center bg-black/20 backdrop-blur transition ease-in-out">
			<div className="rounded-xl bg-background-primary px-6 py-4">
				<h1>This is something</h1>
			</div>
		</div>
	)
}
