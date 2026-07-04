import { useEffect, useRef } from "react";
import { VMState } from "sonorust-debugger-wasm";

export function useVmBatchRunner(
  state: VMState,
  run: (entityId: number, archetypeId: number, maxSteps: number) => void,
  options: { entityId: number; archetypeId: number; maxSteps: number },
) {
  const optionsRef = useRef(options);
  optionsRef.current = options;

  const runRef = useRef(run);
  runRef.current = run;

  useEffect(() => {
    if (state !== VMState.Running) {
      return;
    }

    let frameId: number;

    const tick = () => {
      const { entityId, archetypeId, maxSteps } = optionsRef.current;

      runRef.current(entityId, archetypeId, maxSteps);

      frameId = requestAnimationFrame(tick);
    };

    frameId = requestAnimationFrame(tick);

    return () => {
      cancelAnimationFrame(frameId);
    };
  }, [state]);
}
