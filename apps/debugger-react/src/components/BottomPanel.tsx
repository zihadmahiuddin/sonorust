import { memo, useState } from "react";
import LogsTab from "./LogsTab";
import MemoryTabs from "./MemoryTabs";
import { beginResize } from "../lib/resize";

const BottomPanel = memo(() => {
  const [bottomHeight, setBottomHeight] = useState(230);

  return (
    <>
      <div
        id="bottom-panel-resize-handle"
        className="h-1.25 cursor-row-resize bg-transparent border-t border-border-primary hover:bg-[#1c2634]"
        onMouseDown={(e) =>
          beginResize(e, bottomHeight, "y", true, setBottomHeight, 120, 500)
        }
      />
      <div data-bottom-panel style={{ height: bottomHeight }}>
        <div className="flex h-full w-full bg-bg-primary divide-x divide-border-primary">
          <LogsTab />
          <MemoryTabs />
        </div>
      </div>
    </>
  );
});

BottomPanel.displayName = "BottomPanel";
export default BottomPanel;
