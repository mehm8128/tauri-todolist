import { useEffect, useState } from "react"
import { invoke } from "@tauri-apps/api/tauri"

interface Task {
	id: string
	name: string
	completed: boolean
}

function App() {
	const [name, setName] = useState("")
	const [tasks, setTasks] = useState<Task[]>([])
	const completedTasks = tasks.filter((task) => task.completed)
	const uncompletedTasks = tasks.filter((task) => !task.completed)

	const createTask = async () => {
		const newTask = {
			name,
			completed: false,
		}
		const newTaskRes: Task = await invoke("create_task", { newTask })
    setTasks([...tasks, newTaskRes])
    setName("")
	}

	const toggleTask = async (id: string) => {
		await invoke("toggle_task", { id })
		await fetchTasks()
	}

	const fetchTasks = async () => {
		const tasks: Task[] = await invoke("list_tasks")
		setTasks(tasks)
	}

	useEffect(() => {
		fetchTasks()
	}, [])

	return (
		<main
			style={{
				maxWidth: "400px",
			}}
		>
			<h1>todo list</h1>
			<section>
				<h2>未完タスク</h2>
				<ul>
					{uncompletedTasks.map((task) => (
						<li
							key={task.id}
							style={{
								display: "grid",
								gridAutoFlow: "column",
								gap: "1rem",
								gridTemplateColumns: "1fr auto",
							}}
						>
							{task.name}
							<button onClick={() => toggleTask(task.id)}>
								{task.completed ? "uncomplete" : "complete"}
							</button>
						</li>
					))}
				</ul>
			</section>
			<section>
				<h2>完了タスク</h2>
				<ul>
					{completedTasks.map((task) => (
						<li
							key={task.id}
							style={{
								display: "grid",
								gridAutoFlow: "column",
								gap: "1rem",
								gridTemplateColumns: "1fr auto",
							}}
						>
							{task.name}
							<button onClick={() => toggleTask(task.id)}>
								{task.completed ? "uncomplete" : "complete"}
							</button>
						</li>
					))}
				</ul>
			</section>

			<form
				onSubmit={(e) => {
					e.preventDefault()
					createTask()
				}}
			>
        <input
          value={name}
					onChange={(e) => setName(e.target.value)}
					placeholder="new task name"
				/>
				<button type="submit">create task</button>
			</form>
		</main>
	)
}

export default App
