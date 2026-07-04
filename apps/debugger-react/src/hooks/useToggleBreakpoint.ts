import { useCallback } from "react";
import { hex } from "../lib/utils";
import { useVMStore, useVMStoreInstance } from "../stores/vmStore";
import { useLogStore } from "../stores/logStore";

export function useToggleBreakpoint() {
  const store = useVMStoreInstance();
  const vmToggleBreakpoint = useVMStore((s) => s.toggleBreakpoint);
  const addLog = useLogStore((s) => s.addLog);

  return useCallback(
    (index: number) => {
      const wasSet = store.getState().breakpoints.has(index);
      vmToggleBreakpoint(index);
      addLog(
        "INFO",
        wasSet
          ? `Breakpoint cleared at ${hex(index)}`
          : `Breakpoint set at ${hex(index)}`,
      );
    },
    [store, vmToggleBreakpoint, addLog],
  );
}
