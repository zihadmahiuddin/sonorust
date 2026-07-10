import { HardDrive, X, Plus } from "lucide-react";
import { useRef, useMemo, useState, useCallback, memo } from "react";
import { hex } from "../lib/utils";
import {
  useVirtualizer,
  type ReactVirtualizer,
  type VirtualItem,
} from "@tanstack/react-virtual";
import { useLogStore } from "../stores/logStore";
import { beginResize } from "../lib/resize";
import { usePlayerStore } from "../stores/playerStore";
import { useDebuggerStore } from "../stores/debuggerStore";

export type MemoryTab = {
  blockId: number;
  totalItems: number;
};

const MemoryTabs = memo(() => {
  const memoryBlockSizes = useDebuggerStore((s) => s.memoryBlockSizes);

  const addLog = useLogStore((s) => s.addLog);
  const [memTabs, setMemTabs] = useState<MemoryTab[]>([]);
  const [activeTabBlockId, setActiveTabBlockId] = useState(10000);
  const [addingTab, setAddingTab] = useState(false);
  const [newTabValue, setNewTabValue] = useState("");

  const handleConfirmAddTab = useCallback(() => {
    const id = newTabValue.trim();
    if (!id) {
      setAddingTab(false);
      return;
    }
    const blockId = parseInt(id);
    if (isNaN(blockId)) {
      setAddingTab(false);
      return;
    }
    if (!memoryBlockSizes.has(blockId)) {
      setAddingTab(false);
      return;
    }

    const memoryBlockSize = memoryBlockSizes.get(blockId) ?? 0;

    setMemTabs((prev) => {
      if (prev.some((t) => t.blockId === blockId)) return prev;
      return [...prev, { blockId: blockId, totalItems: memoryBlockSize }];
    });
    setActiveTabBlockId(blockId);
    setAddingTab(false);
    setNewTabValue("");
    addLog("INFO", `Opened memory view for block "${id}"`);
  }, [newTabValue, memoryBlockSizes, addLog]);

  const handleCloseTab = useCallback(
    (id: number) => {
      setMemTabs((prev) => prev.filter((t) => t.blockId !== id));
      setActiveTabBlockId((prevActive) => {
        if (prevActive !== id) return prevActive;
        const remaining = memTabs.filter((t) => t.blockId !== id);
        return remaining.length
          ? remaining[remaining.length - 1].blockId
          : 10000; // Temporary Memory
      });
    },
    [memTabs],
  );

  const scrollParentRef = useRef<HTMLDivElement>(null);

  const activeMemTab = memTabs.find((t) => t.blockId === activeTabBlockId);
  const itemsPerRow = 4;
  const totalRows = activeMemTab
    ? Math.ceil(activeMemTab.totalItems / itemsPerRow)
    : 0;
  const rowVirtualizer = useVirtualizer({
    count: totalRows,
    getScrollElement: () => scrollParentRef.current,
    estimateSize: () => 24,
    overscan: 10,
  });

  const virtualItems = rowVirtualizer.getVirtualItems();

  const [memoryTabsWidth, setMemoryTabsWidth] = useState(500);

  return (
    <>
      <div
        id="memory-tabs-panel-resize-handle"
        className="w-1.25 cursor-col-resize bg-transparent shrink-0 hover:bg-[#1c2634]"
        onMouseDown={(e) =>
          beginResize(
            e,
            memoryTabsWidth,
            "x",
            true,
            setMemoryTabsWidth,
            300,
            900,
          )
        }
      />

      <div
        id="memory-tabs-panel"
        style={{ width: memoryTabsWidth }}
        className="flex flex-col"
      >
        <div
          data-memory-tab-header
          className="flex items-center bg-bg-secondary border-b border-border-primary px-1 min-h-8 relative select-none"
        >
          {memTabs.map((t) => (
            <div
              key={t.blockId}
              className={`flex items-center gap-1.5 px-2.5 h-8 text-[12px] cursor-pointer border-r border-[#1a2029] border-b-2 whitespace-nowrap ${
                activeTabBlockId === t.blockId
                  ? "text-[#d8dee9] border-b-[#e3a13d] bg-bg-primary"
                  : "text-[#8b93a7] border-transparent hover:text-[#d8dee9] hover:bg-[#171c26]"
              }`}
              onClick={() => setActiveTabBlockId(t.blockId)}
            >
              <HardDrive size={12} />
              <span className="max-w-55 overflow-hidden text-ellipsis">
                {t.blockId}
              </span>
              <button
                className="flex border-none bg-transparent text-[#5b6474] p-0.5 rounded-[3px] cursor-pointer hover:bg-border-secondary hover:text-[#e5534b]"
                onClick={(e) => {
                  e.stopPropagation();
                  handleCloseTab(t.blockId);
                }}
                title="Close"
              >
                <X size={11} />
              </button>
            </div>
          ))}

          <div className="relative flex items-center">
            <button
              className="flex items-center justify-center w-6.5 h-6.5 ml-0.5 border-none bg-transparent text-[#6b7484] rounded-[5px] cursor-pointer hover:bg-[#171c26] hover:text-[#d8dee9]"
              onClick={() => setAddingTab(true)}
              title="Open memory block"
            >
              <Plus size={14} />
            </button>
            {addingTab && (
              <div className="absolute top-9.5 left-0 z-10 flex gap-1.5 bg-[#171c26] border border-[#2a3241] rounded-lg p-2 shadow-[0_8px_24px_rgba(0,0,0,0.4)]">
                <input
                  autoFocus
                  className="bg-[#0e1218] border border-[#2a3241] rounded-[5px] text-[#d8dee9] font-mono text-[12px] py-1.5 px-2 w-60 focus:outline-none focus:border-[#4fb3d9]"
                  value={newTabValue}
                  onChange={(e) => setNewTabValue(e.target.value)}
                  placeholder="Memory block ID (e.g. 0x7fff2000)"
                  onKeyDown={(e) => {
                    if (e.key === "Enter") handleConfirmAddTab();
                    else if (e.key === "Escape") {
                      setAddingTab(false);
                      setNewTabValue("");
                    }
                  }}
                />
                <button
                  className="bg-[#e3a13d] border-none rounded-[5px] text-[#17120a] font-semibold text-[12px] px-3 cursor-pointer"
                  onClick={handleConfirmAddTab}
                >
                  Add
                </button>
              </div>
            )}
          </div>
        </div>

        <div
          data-memory-content-viewer
          ref={scrollParentRef}
          className="flex-1 min-h-0 overflow-auto relative"
        >
          {activeMemTab ? (
            <MemoryTab
              itemsPerRow={itemsPerRow}
              rowVirtualizer={rowVirtualizer}
              tabInfo={activeMemTab}
              virtualItems={virtualItems}
            />
          ) : (
            <div className="text-[#4b5568] text-[12px] p-4 italic">
              No memory block open. Click + to view one by ID.
            </div>
          )}
        </div>
      </div>
    </>
  );
});

MemoryTabs.displayName = "MemoryTabs";

export default MemoryTabs;

const MemoryTab = memo(
  ({
    itemsPerRow,
    rowVirtualizer,
    tabInfo,
    virtualItems,
  }: {
    itemsPerRow: number;
    rowVirtualizer: ReactVirtualizer<HTMLDivElement, Element>;
    tabInfo: MemoryTab;
    virtualItems: VirtualItem[];
  }) => {
    const pc = useDebuggerStore((s) => s.currentVmState.pc);
    const player = usePlayerStore((s) => s.player);

    const fetchStartIndex =
      virtualItems.length > 0 ? virtualItems[0].index * itemsPerRow : 0;
    const fetchCount = virtualItems.length * itemsPerRow;

    const visibleFloats = useMemo(() => {
      if (fetchCount === 0) return new Float32Array(0);

      return player.readMemoryRange(
        BigInt(tabInfo.blockId),
        fetchStartIndex,
        fetchCount,
      );
      // pc to track memory updates. maybe in the future make it so it only happens when memory mutation opcodes run?
    }, [tabInfo?.blockId, fetchStartIndex, fetchCount, pc]);

    return (
      <div
        style={{
          height: `${rowVirtualizer.getTotalSize()}px`,
        }}
        className="relative w-full"
      >
        {virtualItems.map((virtualRow) => {
          const localIndex = virtualRow.index * itemsPerRow - fetchStartIndex;

          const rowValues = Array.from({ length: itemsPerRow }).map((_, i) => {
            const val = visibleFloats[localIndex + i];
            return val !== undefined ? val : null;
          });

          const byteAddress = virtualRow.index * itemsPerRow * 4;

          return (
            <div
              key={virtualRow.key}
              className="absolute top-0 left-0 w-full grid grid-cols-[100px_1fr] items-center py-px px-3 font-mono text-[12px] whitespace-nowrap hover:bg-[#10141c] even:bg-bg-secondary/70"
              style={{
                height: `${virtualRow.size}px`,
                transform: `translateY(${virtualRow.start}px)`,
              }}
            >
              <span className="text-[#4fb3d9]">{hex(byteAddress, 8)}</span>

              <span className="text-[#b6bece] tracking-[1px] flex gap-4">
                {rowValues.map((val, i) => (
                  <span
                    key={i}
                    className={`w-20 text-right ${val === 0 ? "text-[#3b4250]" : ""}`}
                  >
                    {val !== null ? val.toPrecision(6) : ""}
                  </span>
                ))}
              </span>
            </div>
          );
        })}
      </div>
    );
  },
);

MemoryTab.displayName = "MemoryTab";
