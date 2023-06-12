import turbocharger_init, * as app from "./turbocharger_generated/assets/dioxus";
import wasmData from "./turbocharger_generated/assets/dioxus/index_bg.wasm?url";

turbocharger_init(wasmData).then((wasm) => {
  if (
    window.location.href.startsWith("http://127.0.0.1:5173/") ||
    window.location.href.startsWith("http://localhost:5173/")
  )
    app.set_socket_url("ws://127.0.0.1:8081/turbocharger_socket");

  wasm.main();
});
