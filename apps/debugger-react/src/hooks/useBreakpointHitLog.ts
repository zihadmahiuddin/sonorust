import { useEffect, useRef } from "react";
import { VMState } from "sonorust-debugger-wasm";
import { hex } from "../lib/utils";
import { useLogStore } from "../stores/logStore";
import { useDebuggerStore } from "../stores/debuggerStore";

export function useBreakpointHitLog() {
  const lastHitBreakpoint = useRef<number | null>(null);

  useEffect(() => {
    return useDebuggerStore.subscribe((s) => {
      if (!s.currentVmState?.breakpoints) return;

      const isAtBreakpoint = s.currentVmState.breakpoints.has(
        s.currentVmState.pc,
      );
      if (s.currentVmState.state === VMState.Paused && isAtBreakpoint) {
        if (lastHitBreakpoint.current !== s.currentVmState.pc) {
          useLogStore
            .getState()
            .addLog("WARN", `Hit breakpoint at ${hex(s.currentVmState.pc)}`);
          lastHitBreakpoint.current = s.currentVmState.pc;
        }
      } else {
        lastHitBreakpoint.current = null;
      }
    });
  }, []);
}
