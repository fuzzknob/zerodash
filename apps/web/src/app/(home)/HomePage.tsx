import { Task } from '@/components/Task'

export default function Home() {
	return (
		<main className="w-full h-full flex-center">
			<div className="p-4 bg-background-secondary rounded-lg min-w-[30rem] h-[70%] max-h-[70%] flex flex-col">
				<h1 className="pb-5 text-2xl font-black text-center">My Day</h1>
				<div className="flex flex-col flex-1 gap-4 overflow-y-auto">
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
