import { IconAlarm } from "@tabler/icons-react";
import { invoke } from "@tauri-apps/api/core";
import { useEffect } from "react";
import "./App.css";
import { getCurrentWindow } from "@tauri-apps/api/window";

export const Bar: React.FC = () => {
  useEffect(() => {
    const handleEscape = (e: KeyboardEvent) => {
      if (e.key === "Escape") {
        e.preventDefault();
        invoke("hide");
      }
    };

    window.addEventListener("keydown", handleEscape);
    return () => window.removeEventListener("keydown", handleEscape);
  }, []);

  return (
    <div
      className="bg-neutral-800 justify-center gap-1 border py-2 border-neutral-600 w-full rounded-full cursor-pointer h-[50px] shadow-lg flex items-center px-4"
      onMouseDown={(e) => {
        const appWindow = getCurrentWindow();
        if (e.buttons === 1) {
          appWindow.startDragging();
        }
      }}
    >
      <IconAlarm className="text-neutral-400 size-8" />
      <input
        className="bg-transparent focus-visible:outline-none w-full placeholder:text-neutral-400"
        placeholder="タスクを入力してください..."
        onMouseDown={(e) => e.stopPropagation()}
        onKeyDown={() => console.log("key")}
      />
    </div>
  );
};
