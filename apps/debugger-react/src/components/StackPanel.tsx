import { useVirtualizer } from "@tanstack/react-virtual";
import { cn } from "@/lib/utils";
import { memo, useMemo, useRef, useState } from "react";
import { beginResize } from "../lib/resize";
import { useShallow } from "zustand/react/shallow";
import { useDebuggerStore } from "../stores/debuggerStore";
import { usePlayerStore } from "../stores/playerStore";
import type { UsedStackIndices } from "sonorust-debugger-wasm";

type StackRow = {
  addr: number;
  value: number;
};

const StackPanel = memo(() => {
  const { pc, stack, target } = useDebuggerStore(
    useShallow((s) => ({
      pc: s.currentVmState.pc,
      stack: s.currentVmState.stack,
      target: s.target,
    })),
  );
  const player = usePlayerStore((s) => s.player);

  const [stackWidth, setStackWidth] = useState(400);

  const stackRows = useMemo<StackRow[]>(
    () => stack.map((value, index) => ({ addr: index, value })),
    [stack],
  );
  const usedStackIndices = useMemo(() => {
    let result: UsedStackIndices;
    if (target) {
      result = player.getInstUsedStackIndices(
        target.archetypeId,
        target.callbackType,
        pc,
      );
    }
    return result ?? new Map<number, string>();
  }, [pc, target]);

  const parentRef = useRef<HTMLDivElement>(null);

  const rowVirtualizer = useVirtualizer({
    count: stackRows.length,
    getScrollElement: () => parentRef.current,
    estimateSize: () => 24,
    overscan: 10,
  });

  return (
    <>
      <div
        id="stack-panel-resize-handle"
        className="w-1.25 cursor-col-resize bg-transparent shrink-0 hover:bg-[#1c2634]"
        onMouseDown={(e) =>
          beginResize(e, stackWidth, "x", true, setStackWidth, 220, 480)
        }
      />

      <div
        id="stack-panel"
        className="flex flex-col"
        style={{ width: stackWidth }}
      >
        <div className="flex flex-col bg-bg-primary min-w-0 flex-1 border-l border-border-primary">
          <div className="flex items-center gap-2.5 h-8.5 min-h-8.5 px-3 bg-bg-secondary border-b border-border-primary">
            <span className="text-[11px] font-bold uppercase tracking-[0.6px] text-[#9aa3b5]">
              Stack
            </span>
          </div>
          <div ref={parentRef} className="flex-1 overflow-y-auto py-1">
            <div
              className="relative w-full"
              style={{
                height: `${rowVirtualizer.getTotalSize()}px`,
              }}
            >
              {rowVirtualizer.getVirtualItems().map((virtualRow) => {
                const invertedIndex = stackRows.length - 1 - virtualRow.index;
                const row = stackRows[invertedIndex];
                const rowLabel = usedStackIndices.get(invertedIndex) ?? "";

                return (
                  <div
                    key={virtualRow.key}
                    style={{
                      height: `${virtualRow.size}px`,
                      transform: `translateY(${virtualRow.start}px)`,
                    }}
                    className={cn(
                      "absolute top-0 left-0 w-full", // for virtualization
                      "grid grid-cols-[1fr_110px] items-center py-0.75 px-2.5 font-mono text-[12px] text-[#b6bece] border-l-2 whitespace-nowrap group/stack even:bg-bg-secondary/70",
                      {
                        "border-l-[#4fb3d9]": !!rowLabel,
                        "border-transparent": !rowLabel,
                      },
                    )}
                  >
                    <span className="text-[#d8dee9] overflow-hidden text-ellipsis">
                      {row.value}
                    </span>
                    <span className="text-[#5b6474] italic text-xs text-right">
                      {rowLabel}
                    </span>
                  </div>
                );
              })}
            </div>
          </div>
        </div>
      </div>
    </>
  );
});

StackPanel.displayName = "StackPanel";

export default StackPanel;
