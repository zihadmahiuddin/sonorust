import { AlignLeft, GitBranch } from "lucide-react";
import { useRef, useEffect, memo } from "react";
import GraphDisassembly from "./GraphDisassembly";
import { useVMStore } from "../stores/vmStore";
import LinearDisassembly from "./LinearDisassembly";

export type DisassemblyViewMode = "linear" | "graph";

const DisassemblyPanel = memo(
  ({
    comments,
    disassemblyViewMode: viewMode,
    setDisassemblyViewMode: onSetViewMode,
  }: {
    comments: Map<number, string>;
    disassemblyViewMode: DisassemblyViewMode;
    setDisassemblyViewMode: (viewMode: DisassemblyViewMode) => void;
  }) => {
    const pc = useVMStore((s) => s.pc);

    const currentLineRef = useRef(null);

    useEffect(() => {
      if (viewMode === "linear" && currentLineRef.current) {
        currentLineRef.current.scrollIntoView({
          block: "nearest",
          behavior: "smooth",
        });
      }
    }, [pc, viewMode]);

    return (
      <div className="flex flex-col bg-bg-primary min-w-0 flex-1">
        <div className="flex items-center gap-2.5 h-8.5 min-h-8.5 px-3 bg-bg-secondary border-b border-border-primary">
          <span className="text-[11px] font-bold uppercase tracking-[0.6px] text-[#9aa3b5]">
            Disassembly
          </span>
          <div className="flex bg-[#171c26] rounded-md p-0.5 gap-0.5">
            <button
              className={`flex items-center gap-1.25 border-none text-[11px] py-1 px-2.25 rounded-[5px] cursor-pointer ${viewMode === "linear" ? "bg-border-secondary text-[#d8dee9]" : "bg-transparent text-[#6b7484]"}`}
              onClick={() => onSetViewMode("linear")}
            >
              <AlignLeft size={13} /> Linear
            </button>
            <button
              className={`flex items-center gap-1.25 border-none text-[11px] py-1 px-2.25 rounded-[5px] cursor-pointer ${viewMode === "graph" ? "bg-border-secondary text-[#d8dee9]" : "bg-transparent text-[#6b7484]"}`}
              onClick={() => onSetViewMode("graph")}
            >
              <GitBranch size={13} /> Graph
            </button>
          </div>
          <span className="ml-auto text-[11px] text-[#48505f]">
            space toggles view
          </span>
        </div>

        {viewMode === "linear" ? (
          <LinearDisassembly
            comments={comments}
            currentLineRef={currentLineRef}
          />
        ) : (
          <GraphDisassembly
            comments={comments}
            currentLineRef={currentLineRef}
          />
        )}
      </div>
    );
  },
);

DisassemblyPanel.displayName = "DisassemblyPanel";

export default DisassemblyPanel;
