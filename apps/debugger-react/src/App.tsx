import { Suspense, useRef, useState } from "react";
import {
  PlayEngineArchetypeCallbackType,
  type EngineArchetypeCallbackType,
} from "sonorust-debugger-wasm";

import Debugger from "./components/Debugger";
import { CompileForm, type CallbackTypeOption } from "./components/CompileForm";
import VMProvider from "./components/VMProvider";

const CALLBACK_TYPE_OPTIONS: CallbackTypeOption[] = Object.keys(
  PlayEngineArchetypeCallbackType,
)
  .filter((x) => isNaN(parseInt(x))) // Remove the number representations
  .map((key) => {
    return { label: key, value: PlayEngineArchetypeCallbackType[key] };
  });

function WasmLoading() {
  return (
    <section className="flex h-full w-full justify-center items-center">
      <p>Loading WASM</p>
    </section>
  );
}

type VMConfig = {
  id: number;
  callbackType: EngineArchetypeCallbackType;
  script: string;
};

function App() {
  const [vmConfig, setVmConfig] = useState<VMConfig | null>(null);
  const nextId = useRef(0);

  const compile = (
    callbackType: EngineArchetypeCallbackType,
    script: string,
  ) => {
    setVmConfig({ id: nextId.current++, callbackType, script });
  };

  return (
    <main className="h-screen w-screen bg-bg-primary text-text-primary">
      <Suspense fallback={<WasmLoading />}>
        {vmConfig === null ? (
          <section className="flex h-full w-full items-center justify-center">
            <div className="w-full max-w-lg rounded-lg border border-border-primary bg-bg-secondary text-[#d8dee9] shadow-2xl">
              <div className="flex h-11 min-h-11 items-center gap-2 border-b border-border-primary px-3.5">
                <span className="text-[13px] font-semibold tracking-[0.2px]">
                  Compile something to start debugging
                </span>
              </div>
              <CompileForm
                isSubmitting={false}
                submitLabel="Compile & start"
                callbackTypeOptions={CALLBACK_TYPE_OPTIONS}
                onSubmit={compile}
              />
            </div>
          </section>
        ) : (
          <VMProvider
            key={vmConfig.id}
            callbackType={vmConfig.callbackType}
            initialScript={vmConfig.script}
          >
            <Debugger
              currentScript={vmConfig.script}
              currentCallbackType={vmConfig.callbackType}
              callbackTypeOptions={CALLBACK_TYPE_OPTIONS}
              onRecompile={compile}
            />
          </VMProvider>
        )}
      </Suspense>
    </main>
  );
}

export default App;
