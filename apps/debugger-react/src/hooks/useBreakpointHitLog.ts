import { useEffect, useRef } from "react";
import { VMState } from "sonorust-debugger-wasm";
import { hex } from "../lib/utils";
import { useVMStoreInstance } from "../stores/vmStore";
import { useLogStore } from "../stores/logStore";

export function useBreakpointHitLog() {
  const store = useVMStoreInstance();
  const addLog = useLogStore((s) => s.addLog);
  const lastHitBreakpoint = useRef<number | null>(null);

  useEffect(() => {
    return store.subscribe((s) => {
      const isAtBreakpoint = s.breakpoints.has(s.pc);
      if (s.state === VMState.Paused && isAtBreakpoint) {
        if (lastHitBreakpoint.current !== s.pc) {
          addLog("WARN", `Hit breakpoint at ${hex(s.pc)}`);
          lastHitBreakpoint.current = s.pc;
        }
      } else {
        lastHitBreakpoint.current = null;
      }
    });
  }, [store, addLog]);
}
