import { VMState } from "sonorust-debugger-wasm";
import type { DisassemblyViewMode } from "./DisassemblyPanel";
import { hex } from "../lib/utils";
import { useShallow } from "zustand/shallow";
import { useDebuggerStore } from "../stores/debuggerStore";

function StateDot({ state }: { state: VMState }) {
  const bg =
    state === VMState.Paused
      ? "bg-[#e3a13d]"
      : state === VMState.Running
        ? "bg-[#4bb679]"
        : "bg-[#e5534b]";
  return <span className={`w-1.75 h-1.75 rounded-full inline-block ${bg}`} />;
}

export default function StatusBar({
  disassemblyViewMode: viewMode,
}: {
  disassemblyViewMode: DisassemblyViewMode;
}) {
  const { pc, state, totalBreakpoints, totalInstructions } = useDebuggerStore(
    useShallow((s) => ({
      pc: s.currentVmState.pc,
      state: s.currentVmState.state,
      totalBreakpoints: s.currentVmState.breakpoints.size,
      totalInstructions: s.currentVmState.compilationResult.instructions.length,
    })),
  );

  return (
    <div className="h-6 min-h-6 flex items-center justify-between px-3 bg-bg-secondary border-t border-border-primary text-[11px] text-[#8b93a7]">
      <div className="flex items-center gap-3.5">
        <StateDot state={state} />
        <span>
          {state === VMState.Paused
            ? "Paused"
            : state === VMState.Running
              ? "Running\u2026"
              : state === VMState.Stopped
                ? "Stopped"
                : state === VMState.Done
                  ? "Done"
                  : "Unknown"}
        </span>
        {pc < totalInstructions ? (
          <span className="font-mono text-[#4fb3d9]">{hex(pc)}</span>
        ) : (
          <></>
        )}
      </div>
      <div className="flex items-center gap-3.5">
        <span>
          {totalBreakpoints} breakpoint
          {totalBreakpoints === 1 ? "" : "s"}
        </span>
        <span>{viewMode === "linear" ? "Linear view" : "Graph view"}</span>
      </div>
    </div>
  );
}
