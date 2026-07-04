import init, { initLogging } from "sonorust-debugger-wasm";
import { useLogStore } from "../stores/logStore";

async function initWasmWithLogging() {
  await init();

  initLogging((level, text) => {
    useLogStore.getState().addLog(level, text);
  });
}

let status: "pending" | "error" | "success" = "pending";
let result: any;
let suspender = initWasmWithLogging().then(
  (res) => {
    status = "success";
    result = res;
  },
  (err) => {
    status = "error";
    result = err;
  },
);

export const wasmResource = {
  read() {
    if (status === "pending") {
      throw suspender;
    } else if (status === "error") {
      throw result;
    } else if (status === "success") {
      return result;
    }
  },
};
