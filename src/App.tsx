import { getCurrentWindow } from "@tauri-apps/api/window";
import "./App.css";
import { IconAlarm, type Icon } from "@tabler/icons-react";
import { type ComponentPropsWithoutRef } from "react";
import { hide } from "@tauri-apps/api/app";

function App() {
  return (
    <div
      className="bg-neutral-800 justify-center gap-1 border py-2 border-neutral-600 w-[300px] rounded-full cursor-pointer h-[50px] shadow-lg flex items-center px-4"
      onMouseDown={(e) => {
        const appWindow = getCurrentWindow();
        if (e.buttons === 1) {
          // appWindow.startDragging();
        }
      }}
    >
      <IconAlarm className="text-neutral-500 size-8" />
      <input
        className="bg-transparent focus-visible:outline-none w-full placeholder:text-neutral-500"
        placeholder="タスクを入力してください..."
        onMouseDown={(e) => e.stopPropagation()}
        onKeyDown={() => console.log("key")}
      />
      <button
        className="rounded p-2 bg-blue-500"
        onClick={() => {
          hide();
        }}
      >
        Hide
      </button>
      <button
        onClick={() => {
          getCurrentWindow().close();
        }}
      >
        Close
      </button>
    </div>
  );
}

export default App;

const Separator: React.FC = () => {
  return <div className="w-[2px] rounded-lg bg-neutral-700 h-full" />;
};

const IconButton: React.FC<
  { icon: Icon } & ComponentPropsWithoutRef<"button">
> = ({ icon: Icon, ...props }) => {
  return (
    <button
      {...props}
      className="size-10 text-sm grid place-items-center transition-colors hover:bg-white/10 rounded-full text-neutral-200"
    >
      <Icon className="size-6" />
    </button>
  );
};
