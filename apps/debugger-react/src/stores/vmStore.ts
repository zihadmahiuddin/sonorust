import { createContext, useContext } from "react";
import { createStore, useStore, type StoreApi } from "zustand";
import {
  InstructionKind,
  VMChanges,
  VMHandle,
  VMState,
  type CompilationResult,
  type EngineArchetypeCallbackType,
  type InstructionOperand,
  type UsedStackIndices,
} from "sonorust-debugger-wasm";

type VMFields = {
  breakpoints: Set<number>;
  memoryBlockSizes: Map<number, number>;
  pc: number;
  stack: number[];
  state: VMState | null;
};

type VMActions = {
  compileAndLoadScript: (script: string) => CompilationResult;
  pause: () => void;
  resume: () => void;
  run: (e: number, a: number, m: number) => void;
  step: (e: number, a: number) => void;
  stop: () => void;
  toggleBreakpoint: (i: number) => void;
  readMemoryRange: (b: number, s: number, c: number) => void;
  disassemble: (i: number) => void;
  getInstKind: (i: number) => InstructionKind;
  getInstMnemonic: (i: number) => string;
  getInstOperands: (i: number) => InstructionOperand[];
  getInstUsedStackIndices: (i: number) => UsedStackIndices;
  getOperandAsString: (o: InstructionOperand) => string;
};

type ExtraVMState = {
  compilationResult: CompilationResult;
};

export type VMStoreState = VMFields & VMActions & ExtraVMState;

function fullSnapshot(v: VMHandle): VMFields {
  return {
    breakpoints: new Set(v.breakpoints),
    memoryBlockSizes: new Map(v.memoryBlockSizes),
    pc: v.pc,
    stack: [...v.stack],
    state: v.state,
  };
}

function patchSnapshot(v: VMHandle, mask: number): Partial<VMFields> {
  const patch: Partial<VMFields> = {};
  if (mask & VMChanges.breakpoints())
    patch.breakpoints = new Set(v.breakpoints);
  if (mask & VMChanges.memory_block_sizes())
    patch.memoryBlockSizes = new Map(v.memoryBlockSizes);
  if (mask & VMChanges.pc()) patch.pc = v.pc;
  if (mask & VMChanges.stack()) patch.stack = [...v.stack];
  if (mask & VMChanges.state()) patch.state = v.state;
  return patch;
}

export function createVMStore(
  callbackType: EngineArchetypeCallbackType,
  initialScript?: string,
) {
  const vm = new VMHandle(callbackType);

  let initialCompilationResult: CompilationResult = {
    basicBlocks: [],
    instructions: [],
    labels: new Map(),
  };
  if (initialScript) {
    initialCompilationResult = vm.compileAndLoadScript(initialScript);
  }

  const store = createStore<VMStoreState>((set) => ({
    ...fullSnapshot(vm),
    compilationResult: initialCompilationResult,

    compileAndLoadScript: (script) => {
      const result = vm.compileAndLoadScript(script);
      set({ compilationResult: result });
      return result;
    },
    pause: () => vm.pause(),
    resume: () => vm.resume(),
    run: (e, a, m) => vm.run(e, a, m),
    step: (e, a) => vm.step(e, a),
    stop: () => vm.stop(),
    toggleBreakpoint: (i) => vm.toggleBreakpoint(i),
    readMemoryRange: (b, s, c) => vm.readMemoryRange(BigInt(b), s, c),
    disassemble: (i) => vm.disassemble(i),
    getInstKind: (i) => vm.getInstKind(i),
    getInstMnemonic: (i) => vm.getInstMnemonic(i),
    getInstOperands: (i) => [...vm.getInstOperands(i)],
    getInstUsedStackIndices: (i) => vm.getInstUsedStackIndices(i),
    getOperandAsString: (o) => vm.getOperandAsString(o),
  }));

  vm.subscribe((mask: number) => {
    // Why this? Because WASM calls this while borrowing VMHandle, then patchSnapshot tries to borrow it again by calling the getters. And BOOM!
    queueMicrotask(() => {
      store.setState(patchSnapshot(vm, mask));
    });
  });

  return {
    store,
    dispose: () => {
      vm.unsubscribe();
      vm.free();
    },
  };
}

export const VMStoreContext = createContext<StoreApi<VMStoreState> | null>(
  null,
);

export function useVMStore<T>(selector: (state: VMStoreState) => T): T {
  const store = useContext(VMStoreContext);
  if (!store) {
    throw new Error("useVMStore must be used inside a <VMProvider>");
  }
  return useStore(store, selector);
}

export function useVMStoreInstance() {
  const store = useContext(VMStoreContext);
  if (!store) {
    throw new Error("useVMStoreInstance must be used inside a <VMProvider>");
  }
  return store;
}
