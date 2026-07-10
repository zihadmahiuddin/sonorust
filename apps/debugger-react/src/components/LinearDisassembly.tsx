import { useVirtualizer } from "@tanstack/react-virtual";
import { useDebuggerStore } from "../stores/debuggerStore";
import InstructionLine from "./InstructionLine";
import { memo, useRef, type Ref } from "react";

const LinearDisassembly = memo(
  ({
    comments,
    currentLineRef,
  }: {
    comments: Map<number, string>;
    currentLineRef: Ref<HTMLDivElement>;
  }) => {
    const instructionsLength = useDebuggerStore(
      (s) => s.currentVmState?.compilationResult?.instructions?.length ?? 0,
    );
    const labels = useDebuggerStore(
      (s) => s.currentVmState?.compilationResult?.labels ?? new Map(),
    );

    const parentRef = useRef<HTMLDivElement>(null);

    const rowVirtualizer = useVirtualizer({
      count: instructionsLength,
      getScrollElement: () => parentRef.current,
      estimateSize: (i) => (labels.has(i) ? 50 : 20),
      overscan: 10,
    });

    return (
      <div ref={parentRef} className="flex-1 overflow-auto py-1">
        <div
          className="relative w-full"
          style={{
            height: `${rowVirtualizer.getTotalSize()}px`,
          }}
        >
          {rowVirtualizer.getVirtualItems().map((virtualRow) => {
            const addr = virtualRow.index;
            return (
              <div
                key={virtualRow.key}
                style={{
                  height: `${virtualRow.size}px`,
                  transform: `translateY(${virtualRow.start}px)`,
                }}
                className="absolute top-0 left-0 w-full" // for virtualization
              >
                <InstructionLine
                  key={addr}
                  currentLineRef={currentLineRef}
                  addr={addr}
                  comment={comments.get(addr)}
                  label={labels.get(addr)}
                />
              </div>
            );
          })}
        </div>
      </div>
    );
  },
);

LinearDisassembly.displayName = "LinearDisassembly";

export default LinearDisassembly;
