import InstructionLine from "./InstructionLine";
import { useVMStore } from "../stores/vmStore";
import { memo, type Ref } from "react";

const LinearDisassembly = memo(
  ({
    comments,
    currentLineRef,
  }: {
    comments: Map<number, string>;
    currentLineRef: Ref<HTMLDivElement>;
  }) => {
    const compilationResult = useVMStore((s) => s.compilationResult);

    return (
      <div className="flex-1 overflow-auto py-1">
        {compilationResult.instructions.map((_, addr) => {
          return (
            <InstructionLine
              key={addr}
              currentLineRef={currentLineRef}
              addr={addr}
              comment={comments.get(addr)}
              label={compilationResult.labels.get(addr)}
            />
          );
        })}
      </div>
    );
  },
);

LinearDisassembly.displayName = "LinearDisassembly";

export default LinearDisassembly;
