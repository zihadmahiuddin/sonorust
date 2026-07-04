import { type ReactNode, useEffect, useState } from "react";
import type { EngineArchetypeCallbackType } from "sonorust-debugger-wasm";
import {
  createVMStore,
  VMStoreContext,
  type VMStoreState,
} from "../stores/vmStore";
import type { StoreApi } from "zustand";

export default function VMProvider({
  callbackType,
  initialScript,
  children,
}: {
  callbackType: EngineArchetypeCallbackType;
  initialScript?: string;
  children: ReactNode;
}) {
  const [store, setStore] = useState<StoreApi<VMStoreState> | null>(null);

  useEffect(() => {
    const vm = createVMStore(callbackType, initialScript);

    setStore(vm.store);

    return () => {
      vm.dispose();
    };
  }, [callbackType, initialScript]);

  if (!store) {
    return null;
  }

  return (
    <VMStoreContext.Provider value={store}>{children}</VMStoreContext.Provider>
  );
}
