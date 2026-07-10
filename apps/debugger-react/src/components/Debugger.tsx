import { useState, useRef, useEffect, useCallback } from "react";
import { Bug, Hammer } from "lucide-react";
import { wasmResource } from "../lib/wasmLoader";
import DisassemblyPanel, { type DisassemblyViewMode } from "./DisassemblyPanel";
import ToolbarButton from "./ToolbarButton";
import StackPanel from "./StackPanel";
import ExecutionControls from "./ExecutionControls";
import StatusBar from "./StatusBar";
import BottomPanel from "./BottomPanel";
import CompileDialogModal, {
  type CompileDialogHandle,
} from "./CompileDialogModal";
import { useBreakpointHitLog } from "../hooks/useBreakpointHitLog";
import { DebuggerToolbar } from "./DebuggerToolbar";
import type { CompileConfig } from "../stores/playerStore";

const TODO_COMMENTS = new Map<number, string>();

export default function Debugger({
  onCompile,
}: {
  onCompile: (compileConfig: CompileConfig) => Promise<void>;
}) {
  wasmResource.read();

  const [disassemblyViewMode, setDisassemblyViewMode] =
    useState<DisassemblyViewMode>("linear");

  useBreakpointHitLog();

  const compileDialogRef = useRef<CompileDialogHandle>(null);

  useEffect(() => {
    function handleKey(e: KeyboardEvent) {
      if (e.code !== "Space") return;
      const target = e.target as HTMLElement;
      if (
        target.matches(
          "input, textarea, [contenteditable], button, [role='button']",
        )
      ) {
        return;
      }
      e.preventDefault();
      setDisassemblyViewMode((v) => (v === "linear" ? "graph" : "linear"));
    }
    window.addEventListener("keydown", handleKey);
    return () => window.removeEventListener("keydown", handleKey);
  }, []);

  const doCompile = useCallback(() => {
    compileDialogRef.current?.open();
  }, []);

  return (
    <div className="scrollbar-container h-screen min-h-155 w-full flex flex-col bg-bg-primary text-[#d8dee9] font-sans text-[13px] overflow-hidden **:box-border">
      <div className="flex items-center justify-between h-11 min-h-11 px-3.5 bg-bg-secondary border-b border-border">
        <div className="flex items-center gap-2">
          <Bug size={16} className="text-[#e3a13d]" />
          <span className="font-semibold text-[13px] tracking-[0.2px]">
            Sonolus Debugger
          </span>

          <ToolbarButton
            icon={Hammer}
            label="Compile Script"
            onClick={doCompile}
            tone="blue"
          />
        </div>

        <DebuggerToolbar />

        <ExecutionControls />
      </div>

      <div className="flex-1 min-h-0 flex flex-col">
        <div className="flex-1 min-h-0 flex">
          <DisassemblyPanel
            comments={TODO_COMMENTS}
            disassemblyViewMode={disassemblyViewMode}
            setDisassemblyViewMode={setDisassemblyViewMode}
          />

          <StackPanel />
        </div>

        <BottomPanel />
      </div>
      <StatusBar disassemblyViewMode={disassemblyViewMode} />
      <CompileDialogModal ref={compileDialogRef} onCompile={onCompile} />
    </div>
  );
}
