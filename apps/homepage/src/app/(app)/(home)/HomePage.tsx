import { Task } from '@/components/Task'

export default function Home() {
	return (
		<main className="h-full w-full flex-center">
			<div className="flex h-[70%] max-h-[70%] min-w-[30rem] flex-col rounded-lg bg-background-secondary p-4">
				<h1 className="pb-5 text-center font-black text-2xl">My Day</h1>
				<div className="flex flex-1 flex-col gap-4 overflow-y-auto">
					<Task />
					<Task />
					<Task />
					<Task />
					<Task />
					<Task />
					<Task />
					<Task />
					<Task />
					<Task />
					<Task />
				</div>
			</div>
		</main>
	)
}
