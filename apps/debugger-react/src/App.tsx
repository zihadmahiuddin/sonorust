import { Suspense, useCallback, useState } from "react";

import Debugger from "./components/Debugger";
import { CompileForm } from "./components/CompileForm";
import { usePlayerStore, type CompileConfig } from "./stores/playerStore";

function WasmLoading() {
  return (
    <section className="flex h-full w-full justify-center items-center">
      <p>Loading WASM</p>
    </section>
  );
}

function App() {
  const player = usePlayerStore((s) => s.player);
  const initFromCompileConfig = usePlayerStore((s) => s.initFromCompileConfig);
  const [isCompiling, setIsCompiling] = useState(false);

  const onCompile = useCallback(
    async (compileConfig: CompileConfig) => {
      setIsCompiling(true);
      await initFromCompileConfig(compileConfig);
      setIsCompiling(false);
    },
    [initFromCompileConfig],
  );

  return (
    <main className="h-screen w-screen bg-bg-primary text-text-primary">
      <Suspense fallback={<WasmLoading />}>
        {player === null ? (
          <section className="flex h-full w-full items-center justify-center">
            <div className="w-full max-w-lg rounded-lg border border-border-primary bg-bg-secondary text-[#d8dee9] shadow-2xl">
              <div className="flex h-11 min-h-11 items-center gap-2 border-b border-border-primary px-3.5">
                <span className="text-[13px] font-semibold tracking-[0.2px]">
                  Compile something to start debugging
                </span>
              </div>
              <CompileForm
                isSubmitting={isCompiling}
                submitLabel="Compile & start"
                onSubmit={onCompile}
              />
            </div>
          </section>
        ) : (
          <Debugger onCompile={onCompile} />
        )}
      </Suspense>
    </main>
  );
}

export default App;
