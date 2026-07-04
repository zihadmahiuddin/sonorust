import { Circle } from "lucide-react";
import { memo, type Ref } from "react";
import { cn } from "../lib/tailwind";
import { hex } from "../lib/utils";
import { InstructionKind } from "sonorust-debugger-wasm";
import { useVMStore, useVMStoreInstance } from "../stores/vmStore";
import { useShallow } from "zustand/shallow";
import { useToggleBreakpoint } from "../hooks/useToggleBreakpoint";

function colorClassForInstKind(instKind: InstructionKind) {
  if (instKind === InstructionKind.StackManipulation) {
    return "text-inst-stack";
  }
  if (instKind === InstructionKind.MemoryManipulation) {
    return "text-inst-memory";
  }
  if (instKind === InstructionKind.Comparison) {
    return "text-inst-comparison";
  }
  if (instKind === InstructionKind.Branching) {
    return "text-inst-branch";
  }
  if (instKind === InstructionKind.Math) {
    return "text-inst-math";
  }
  if (instKind === InstructionKind.SideEffects) {
    return "text-inst-side-effect";
  }

  return "text-inst-others";
}

const InstructionLine = memo(
  ({
    addr,
    comment,
    currentLineRef,
    label,
  }: {
    addr: number;
    comment?: string;
    currentLineRef?: Ref<HTMLDivElement>;
    label?: string;
  }) => {
    const { isBreakpoint, isCurrent } = useVMStore(
      useShallow((s) => ({
        isBreakpoint: s.breakpoints.has(addr),
        isCurrent: s.pc === addr,
      })),
    );
    const toggleBreakpoint = useToggleBreakpoint();
    const store = useVMStoreInstance();

    return (
      <>
        {label && (
          <div className="font-mono text-[#4fb3d9] text-[12px] pt-1.5 pb-0.5 pr-3 pl-10.5 font-semibold">
            {label}:
          </div>
        )}
        <div
          ref={isCurrent ? currentLineRef : null}
          className={cn(
            "group grid grid-cols-[22px_16px_100px_56px_1fr] items-center p-0.5 whitespace-nowrap border-l-2 font-mono text-[12.5px] hover:bg-[#10141c]",
            {
              "bg-[#e3a13d]/10 border-l-[#e3a13d] text-[#b6bece]": isCurrent,
              "border-transparent text-[#b6bece] even:bg-bg-secondary/70":
                !isCurrent,
            },
          )}
        >
          <div
            className="flex items-center justify-center h-full cursor-pointer text-[#e5534b]"
            onClick={() => toggleBreakpoint(addr)}
            title={isBreakpoint ? "Remove breakpoint" : "Add breakpoint"}
          >
            <Circle
              size={8}
              className={`opacity-0 group-hover:opacity-40 transition-opacity ${isBreakpoint ? "opacity-100!" : ""}`}
              fill={isBreakpoint ? "currentColor" : "none"}
            />
          </div>
          <div className="text-[#e3a13d] text-[10px]">
            {isCurrent ? "\u25B6" : ""}
          </div>
          <div className="text-[#4fb3d9]">{hex(addr, 8)}</div>
          <div
            className={cn(
              "font-semibold overflow-hidden text-ellipsis",
              colorClassForInstKind(store.getState().getInstKind(addr)),
            )}
          >
            {store.getState().getInstMnemonic(addr)}
          </div>
          <div className="flex items-center overflow-hidden">
            <span className="text-[#d8dee9] overflow-hidden text-ellipsis whitespace-nowrap">
              {store
                .getState()
                .getInstOperands(addr)
                .map(store.getState().getOperandAsString)
                .join(", ")}
            </span>
            {comment && (
              <span className="text-[#4b5568] italic ml-2.5 overflow-hidden text-ellipsis whitespace-nowrap">
                ; {comment}
              </span>
            )}
          </div>
        </div>
      </>
    );
  },
);

InstructionLine.displayName = "InstructionLine";

export default InstructionLine;
