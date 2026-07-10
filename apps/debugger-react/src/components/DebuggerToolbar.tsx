import { useEffect, useState } from "react";
import { usePlayerStore } from "../stores/playerStore";
import { useDebuggerStore } from "../stores/debuggerStore";
import {
  PlayEngineArchetypeCallbackType,
  type ArchetypeId,
  type EntityId,
} from "sonorust-debugger-wasm";
import { useShallow } from "zustand/shallow";
import { Select } from "radix-ui";
import { cn } from "@/lib/utils";
import { ChevronDown, Check } from "lucide-react";

const CALLBACK_TYPES = [
  { value: PlayEngineArchetypeCallbackType.Preprocess, label: "Preprocess" },
  { value: PlayEngineArchetypeCallbackType.SpawnOrder, label: "Spawn Order" },
  { value: PlayEngineArchetypeCallbackType.ShouldSpawn, label: "Should Spawn" },
  { value: PlayEngineArchetypeCallbackType.Initialize, label: "Initialize" },
  {
    value: PlayEngineArchetypeCallbackType.UpdateSequential,
    label: "Update (Sequential)",
  },
  { value: PlayEngineArchetypeCallbackType.Touch, label: "Touch" },
  {
    value: PlayEngineArchetypeCallbackType.UpdateParallel,
    label: "Update (Parallel)",
  },
  { value: PlayEngineArchetypeCallbackType.Terminate, label: "Terminate" },
];

export function DebuggerToolbar() {
  const { player, playerState } = usePlayerStore(
    useShallow((s) => ({
      player: s.player,
      playerState: s.playerState,
    })),
  );
  const { target, setTarget } = useDebuggerStore(
    useShallow((s) => ({
      target: s.target,
      setTarget: s.setTarget,
    })),
  );

  const [archetypes, setArchetypes] = useState<[ArchetypeId, string][]>([]);
  const [entities, setEntities] = useState<EntityId[]>([]);

  useEffect(() => {
    if (player) {
      const archs = player.getArchetypes();
      setArchetypes([...archs.entries()].sort((a, b) => a[0] - b[0]));

      const archetypeId = archs.size ? 0 : undefined;
      const ents = archetypeId
        ? player.getEntitiesForArchetype(archetypeId)
        : [];
      const entityId = ents[0] ?? undefined;

      setTarget({
        archetypeId,
        callbackType: CALLBACK_TYPES[0].value,
        entityId: entityId,
      });
    }
  }, [player, setTarget]);

  useEffect(() => {
    if (player && target?.archetypeId !== undefined) {
      const ents = player.getEntitiesForArchetype(target.archetypeId);
      setEntities([...ents]);

      if (ents.includes(target.entityId)) {
        return;
      }

      setTarget({ ...target, entityId: ents[0] });
    }
  }, [player, target?.archetypeId, setTarget]);

  if (!player || !target) return <div>Loading...</div>;

  return (
    <div className="flex items-center gap-4 p-2">
      <DebuggerToolbarSelect
        value={target.archetypeId?.toString() ?? ""}
        onValueChange={(value) =>
          setTarget({ ...target, archetypeId: Number(value) })
        }
        placeholder="Select archetype..."
      >
        {archetypes.map(([id, name]) => (
          <DebuggerToolbarSelectItem key={id} value={id.toString()}>
            {name} ({id})
          </DebuggerToolbarSelectItem>
        ))}
      </DebuggerToolbarSelect>

      <DebuggerToolbarSelect
        value={target.entityId?.toString() ?? ""}
        onValueChange={(value) =>
          setTarget({ ...target, entityId: Number(value) })
        }
        placeholder="Select entity..."
      >
        {entities.map((eId) => (
          <DebuggerToolbarSelectItem key={eId} value={eId.toString()}>
            Entity #{eId}
          </DebuggerToolbarSelectItem>
        ))}
      </DebuggerToolbarSelect>

      <DebuggerToolbarSelect
        value={target.callbackType?.toString() ?? ""}
        onValueChange={(value) =>
          setTarget({ ...target, callbackType: Number(value) })
        }
        placeholder="Select callback type..."
      >
        {CALLBACK_TYPES.map((callbackType) => {
          return (
            <DebuggerToolbarSelectItem
              key={callbackType.value}
              value={callbackType.value.toString()}
            >
              {callbackType.label}
            </DebuggerToolbarSelectItem>
          );
        })}
      </DebuggerToolbarSelect>

      <button
        onClick={() => {
          useDebuggerStore
            .getState()
            .toggleBreakpoint(target.archetypeId, target.callbackType, 0);
        }}
        className="px-3 py-1 bg-red-900/50 text-red-400 border border-red-500 rounded"
      >
        Set Entry Breakpoint
      </button>

      <button
        disabled={playerState.type !== "waiting"}
        onClick={() => usePlayerStore.getState().start()}
        className="px-3 py-1 bg-green-900/50 text-green-400 border border-green-500 rounded"
      >
        Run Game Loop
      </button>
    </div>
  );
}

function DebuggerToolbarSelect({
  value,
  onValueChange,
  children,
  placeholder,
}: {
  value?: string;
  onValueChange: (val: string) => void;
  children: React.ReactNode;
  placeholder?: string;
}) {
  return (
    <Select.Root value={value} onValueChange={onValueChange}>
      <Select.Trigger
        className={cn(
          "flex h-8 items-center justify-between gap-2 px-3",
          "border border-[#2a3140] bg-[#1c2634] text-[#d8dee9] rounded-md text-[12.5px]",
          "hover:bg-[#252e3d] focus:outline-none focus:ring-1 focus:ring-[#4fb3d9]",
        )}
      >
        <Select.Value placeholder={placeholder} />
        <Select.Icon>
          <ChevronDown size={14} className="text-[#5b6472]" />
        </Select.Icon>
      </Select.Trigger>

      <Select.Portal>
        <Select.Content
          position="popper"
          sideOffset={4}
          className="z-50 overflow-hidden rounded-md border border-[#2a3140] bg-[#1c2634] shadow-xl"
        >
          <Select.Viewport className="p-1 min-w-(--radix-select-trigger-width)">
            {children}
          </Select.Viewport>
        </Select.Content>
      </Select.Portal>
    </Select.Root>
  );
}

export function DebuggerToolbarSelectItem({
  value,
  children,
}: {
  value: string;
  children: React.ReactNode;
}) {
  return (
    <Select.Item
      value={value}
      className={cn(
        "relative flex h-8 w-full cursor-pointer select-none items-center rounded px-3 py-1.5 text-[12.5px] text-[#d8dee9] outline-none",
        "data-highlighted:bg-[#4fb3d9]/15 data-highlighted:text-[#4fb3d9]",
      )}
    >
      <Select.ItemText>{children}</Select.ItemText>
      <Select.ItemIndicator className="absolute right-2">
        <Check size={14} />
      </Select.ItemIndicator>
    </Select.Item>
  );
}
