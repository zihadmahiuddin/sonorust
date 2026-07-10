import { memo, useCallback } from "react";
import { Play, Pause, Square, RotateCcw, ArrowDown } from "lucide-react";
import {
  PlayEngineArchetypeCallbackType,
  VMState,
} from "sonorust-debugger-wasm";
import ToolbarButton from "./ToolbarButton";
import { hex } from "../lib/utils";
import { useLogStore, type LogLevel } from "../stores/logStore";
import { useDebuggerStore } from "../stores/debuggerStore";
import { usePlayerStore } from "../stores/playerStore";

function addLog(level: LogLevel, text: string) {
  useLogStore.getState().addLog(level, text);
}

const ExecutionControls = memo(() => {
  const state = useDebuggerStore((s) => s.currentVmState.state);
  console.log("state:", state, VMState[state]);

  const doContinue = useCallback(() => {
    const { target } = useDebuggerStore.getState();
    const { player, start } = usePlayerStore.getState();
    const pc = player.getVmPc(target.archetypeId, target.callbackType);
    start();
    addLog(
      "INFO",
      `Execution resumed from ${hex(pc)} (entity ${target.entityId}, archetype ${target.archetypeId}, callback ${PlayEngineArchetypeCallbackType[target.callbackType]})`,
    );
  }, []);

  const doPause = useCallback(() => {
    const { pause, player } = usePlayerStore.getState();
    const target = pause();
    const pc = player.getVmPc(target.archetypeId, target.callbackType);
    useDebuggerStore.getState().setTarget(target);
    addLog(
      "WARN",
      `Execution paused at ${hex(pc)} (entity ${target.entityId}, archetype ${target.archetypeId}, callback ${PlayEngineArchetypeCallbackType[target.callbackType]})`,
    );
  }, []);

  const doStep = useCallback(() => {
    const { player, stepOver } = usePlayerStore.getState();
    stepOver();
    const { target } = useDebuggerStore.getState();
    const pc = player.getVmPc(target.archetypeId, target.callbackType);
    addLog("INFO", `Step: now at ${hex(pc)}`);
  }, []);

  const doStop = useCallback(() => {
    // const { player } = usePlayerStore.getState();
    // player.stop();
    addLog("ERROR", "Process terminated by user");
  }, []);

  const doRestart = useCallback(() => {
    // const { player } = usePlayerStore.getState();
    // player.stop();
    addLog("INFO", "Process restarted");
    // player.resume();
  }, []);

  const canContinue = state !== VMState.Done && state !== VMState.Running;
  const canPause = state === VMState.Running;
  const canStep = state === VMState.Paused;
  // const canStop = state !== VMState.Done && state !== VMState.Stopped;

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
        disabled
        // disabled={!canStop}
        tone="red"
      />
      <ToolbarButton
        disabled
        icon={RotateCcw}
        label="Restart"
        onClick={doRestart}
      />
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
