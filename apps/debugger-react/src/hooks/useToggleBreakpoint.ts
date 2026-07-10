import { useCallback } from "react";
import { hex } from "../lib/utils";
import { useLogStore } from "../stores/logStore";
import { useDebuggerStore } from "../stores/debuggerStore";

export function useToggleBreakpoint() {
  const target = useDebuggerStore((s) => s.target);
  const vmToggleBreakpoint = useDebuggerStore((s) => s.toggleBreakpoint);
  const addLog = useLogStore((s) => s.addLog);

  return useCallback(
    (index: number) => {
      const wasSet = useDebuggerStore
        .getState()
        .currentVmState.breakpoints.has(index);
      vmToggleBreakpoint(target.archetypeId, target.callbackType, index);
      addLog(
        "INFO",
        wasSet
          ? `Breakpoint cleared at ${hex(index)}`
          : `Breakpoint set at ${hex(index)}`,
      );
    },
    [target, vmToggleBreakpoint, addLog],
  );
}
