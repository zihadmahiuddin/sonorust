import { memo, useCallback } from "react";
import { Play, Pause, Square, RotateCcw, ArrowDown } from "lucide-react";
import { VMState } from "sonorust-debugger-wasm";
import ToolbarButton from "./ToolbarButton";
import { hex } from "../lib/utils";
import { useLogStore, type LogLevel } from "../stores/logStore";
import { useVMStore, useVMStoreInstance } from "../stores/vmStore";

function addLog(level: LogLevel, text: string) {
  useLogStore.getState().addLog(level, text);
}

const ExecutionControls = memo(() => {
  const state = useVMStore((s) => s.state);
  console.log("state:", state);
  const store = useVMStoreInstance();

  const doContinue = useCallback(() => {
    const { pc, resume } = store.getState();
    resume();
    addLog("INFO", `Execution resumed at ${hex(pc)}`);
  }, [store]);

  const doPause = useCallback(() => {
    const { pc, pause } = store.getState();
    pause();
    addLog("WARN", `Execution paused at ${hex(pc)}`);
  }, [store]);

  const doStep = useCallback(() => {
    const { step } = store.getState();
    step(1, 2);
    // Why this? Because otherwise you might get the old pc!
    queueMicrotask(() => {
      const { pc } = store.getState();
      addLog("INFO", `Step: now at ${hex(pc)}`);
    });
  }, [store]);

  const doStop = useCallback(() => {
    const { stop } = store.getState();
    stop();
    addLog("ERROR", "Process terminated by user");
  }, [store]);

  const doRestart = useCallback(() => {
    const { resume, stop } = store.getState();
    stop();
    addLog("INFO", "Process restarted");
    resume();
  }, [store]);

  const canContinue = state !== VMState.Done && state !== VMState.Running;
  const canPause = state === VMState.Running;
  const canStep = state === VMState.Paused;
  const canStop = state !== VMState.Done && state !== VMState.Stopped;

  return (
    <div className="flex items-center gap-1">
      <ToolbarButton
        icon={Play}
        label="Continue (F5)"
        onClick={doContinue}
        disabled={!canContinue}
        tone="green"
      />
      <ToolbarButton
        icon={Pause}
        label="Pause"
        onClick={doPause}
        disabled={!canPause}
        tone="amber"
      />
      <ToolbarButton
        icon={Square}
        label="Stop"
        onClick={doStop}
        disabled={!canStop}
        tone="red"
      />
      <ToolbarButton icon={RotateCcw} label="Restart" onClick={doRestart} />
      <span className="w-px h-5 bg-border-secondary mx-1.5" />
      <ToolbarButton
        icon={ArrowDown}
        label="Step (F10)"
        onClick={doStep}
        disabled={!canStep}
      />
    </div>
  );
});

ExecutionControls.displayName = "ExecutionControls";
export default ExecutionControls;
