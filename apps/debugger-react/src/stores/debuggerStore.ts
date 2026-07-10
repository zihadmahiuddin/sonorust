import { create } from "zustand";
import { usePlayerStore } from "./playerStore";
import {
  PlayEngineArchetypeCallbackType,
  SonorustPlayerHandle,
  VMState,
  type CompilationResult,
  type ExecutionTarget,
  type MemoryBlockSizes,
} from "sonorust-debugger-wasm";

type VMFields = {
  breakpoints: Set<number>;
  compilationResult: CompilationResult;
  pc: number;
  stack: number[];
  state: VMState;
};

type DebuggerStore = {
  currentVmState: VMFields;
  memoryBlockSizes: MemoryBlockSizes;
  target: ExecutionTarget | null;

  setTarget: (target: ExecutionTarget | null) => void;
  syncVmState: () => void;
  toggleBreakpoint: (
    archetypeId: number,
    callbackType: PlayEngineArchetypeCallbackType,
    pc: number,
  ) => void;
  validateOrSelectTarget: (player: SonorustPlayerHandle) => void;
};

const DEFAULT_COMPILATION_RESULT: CompilationResult = {
  basicBlocks: [],
  instructions: [],
  labels: new Map(),
};

export const useDebuggerStore = create<DebuggerStore>()((set, get) => ({
  currentVmState: {
    breakpoints: new Set(),
    compilationResult: DEFAULT_COMPILATION_RESULT,
    pc: 0,
    stack: [],
    state: VMState.Stopped,
  },
  memoryBlockSizes: new Map(),
  target: null,

  setTarget: (target) => {
    const player = usePlayerStore.getState().player;
    if (!player) return;
    const oldTarget = get().target;

    const needsVmResync =
      !oldTarget ||
      target.archetypeId !== oldTarget.archetypeId ||
      target.callbackType !== oldTarget.callbackType;

    set({ target: target });
    if (needsVmResync) {
      get().syncVmState();
    }
  },

  syncVmState: () => {
    const { target } = get();
    const player = usePlayerStore.getState().player;
    if (!player) return;

    const breakpoints = new Set(
      player.getVmBreakpoints(target.archetypeId, target.callbackType) ?? [],
    );
    const compilationResult =
      player.getCompilationResult(target.archetypeId, target.callbackType) ??
      DEFAULT_COMPILATION_RESULT;
    const memoryBlockSizes = player.memoryBlockSizes;
    const pc = player.getVmPc(target.archetypeId, target.callbackType);
    const stack = player.getVmStack(target.archetypeId, target.callbackType);
    const state = player.getVmState(target.archetypeId, target.callbackType);

    set({
      memoryBlockSizes,
      currentVmState: {
        pc,
        stack: stack ? [...stack] : [],
        state,
        compilationResult,
        breakpoints,
      },
    });
  },

  toggleBreakpoint: (archetypeId, callbackType, pc) => {
    const player = usePlayerStore.getState().player;
    if (!player) return;

    player.toggleBreakpoint(archetypeId, callbackType, pc);
    get().syncVmState();
  },

  validateOrSelectTarget: (player: SonorustPlayerHandle) => {
    const current = get().target;
    const archetypes = player.getArchetypes();

    if (current && archetypes.has(current.archetypeId)) {
      const entities = player.getEntitiesForArchetype(current.archetypeId);
      if (entities.includes(current.entityId)) return;
    }

    const archetypeId = archetypes.keys().next().value;
    if (!archetypeId) {
      get().setTarget(null);
    }

    const entityIds = player.getEntitiesForArchetype(archetypeId) ?? [];

    get().setTarget({
      archetypeId,
      entityId: entityIds[0],
      callbackType: PlayEngineArchetypeCallbackType.Preprocess,
    });
  },
}));

usePlayerStore.subscribe((currentState, prevState) => {
  if (currentState.updateTick === prevState.updateTick) return;

  const player = currentState.player;
  if (!player) return;

  const debuggerStore = useDebuggerStore.getState();
  const pState = currentState.playerState;

  if (pState.type === "paused") {
    debuggerStore.setTarget({
      entityId: pState.entityId,
      archetypeId: pState.archetypeId,
      callbackType:
        PlayEngineArchetypeCallbackType[
          pState.callbackType as unknown as string
        ], // Tsify sends the callbackType as string :/
    });
  } else {
    debuggerStore.validateOrSelectTarget(currentState.player);
  }

  if (debuggerStore.target) {
    debuggerStore.syncVmState();
  }
});
