import { createSignal } from "solid-js";
import logo from "./assets/logo.svg";
import { invoke } from "@tauri-apps/api/tauri";
import "./App.css";

function App() {
  const [greetMsg, setGreetMsg] = createSignal("");
  const [name, setName] = createSignal("");

  async function greet() {
    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
    setGreetMsg(await invoke("greet", { name: name() }));
    setGreetMsg(await invoke("copy_to_clipboard"));
  }

  // setTimeout(() => {
  //   invoke("hide_window");
  // }, 1000);

  // setInterval(async () => {
  //   setGreetMsg(await invoke("copy_to_clipboard"));
  // }, 1000);

  return (
    <span data-tauri-drag-region class="container">
      已复制
    </span>
  );
}

export default App;
