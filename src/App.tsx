import { useEffect, useState } from "react";
import "./App.css";
import { events } from "./gen/bindings";

export const App = () => {
	const [tasks, setTasks] = useState<string[]>([]);

	useEffect(() => {
		const unlistenPromise = events.showTaskEvent.listen((e) => {
			const task = e.payload as string;
			if (task === "!reset") {
				setTasks([]);
			} else {
				setTasks((ts) => [...ts, task]);
			}
		});

		return () => {
			unlistenPromise.then((unlisten) => unlisten());
		};
	}, []);

	return (
		<div className="h-screen text-5xl font-bold  flex flex-col items-center pt-10">
			{tasks.map((t) => {
				return <p key={t}>{t}</p>;
			})}
		</div>
	);
};
