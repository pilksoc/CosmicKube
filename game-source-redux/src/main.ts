import "./app.css";
import App from "./App.svelte";
import KubeWebSocket from "./lib/websockets";

const app = new App({
  target: document.getElementById("app")!,
});

KubeWebSocket.onError = (err) => {
  console.error(err);
};

KubeWebSocket.onClose = () => {
  console.error("Connection closed");
};

export default app;
