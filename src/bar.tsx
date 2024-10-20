import { IconAlarm } from "@tabler/icons-react";
import { useEffect } from "react";
import "./App.css";
import { getCurrentWindow } from "@tauri-apps/api/window";
import { commands } from "./gen/bindings";

export const Bar: React.FC = () => {
	useEffect(() => {
		const handleEscape = (e: KeyboardEvent) => {
			if (e.key === "Escape") {
				e.preventDefault();
				commands.hide();
			}
		};

		window.addEventListener("keydown", handleEscape);
		return () => window.removeEventListener("keydown", handleEscape);
	}, []);

	const handleMouseDown = (e: React.MouseEvent) => {
		const appWindow = getCurrentWindow();
		if (e.buttons === 1) {
			appWindow.startDragging();
		}
	};

	const handleSubmit = (e: React.FocusEvent<HTMLFormElement>) => {
		e.preventDefault();

		const formData = new FormData(e.currentTarget);
		const task = formData.get("task")?.toString() ?? "";

		commands.showTask(task);
		e.currentTarget.reset();
	};

	return (
		<div
			className="bg-neutral-800 justify-center gap-1 border py-2 border-neutral-600 w-full rounded-full cursor-pointer h-[50px] shadow-lg flex items-center px-4"
			onMouseDown={handleMouseDown}
		>
			<IconAlarm className="text-neutral-400 size-8" />
			<form className="w-full" onSubmit={handleSubmit}>
				<input
					name="task"
					className="bg-transparent focus-visible:outline-none w-full placeholder:text-neutral-400"
					placeholder="タスクを入力してください..."
					onMouseDown={(e) => e.stopPropagation()}
				/>
			</form>
		</div>
	);
};
