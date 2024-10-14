import { useEffect, useState } from "react";
import "./App.css";
import { listen } from "@tauri-apps/api/event";

export const App = () => {
  const [tasks, setTasks] = useState<string[]>([]);

  useEffect(() => {
    const unlistenPromise = listen("task", (e) => {
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
