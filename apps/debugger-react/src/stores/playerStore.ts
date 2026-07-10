import {
  initSonorustPlayer,
  type ExecutionTarget,
  type SonorustPlayerHandle,
  type SonorustPlayerState,
} from "sonorust-debugger-wasm";
import { create } from "zustand";

export type CompileConfig =
  | {
      mode: "scpFile";
      level: string;
      file?: File;
    }
  | {
      mode: "serverUrl";
      url: string;
      level: string;
    };

type PlayerStore = {
  player: SonorustPlayerHandle | null;
  playerState: SonorustPlayerState;
  animationLoopRunning: boolean;
  animationFrameId: number | null;
  lastUpdateMask: number;
  updateTick: number;

  initFromCompileConfig: (compileConfig: CompileConfig) => Promise<void>;
  deinit: () => void;
  start: () => void;
  pause: () => ExecutionTarget;
  stepOver: () => void;
};

export const usePlayerStore = create<PlayerStore>()((set, get) => ({
  player: null,
  playerState: { type: "waiting" },
  animationLoopRunning: false,
  animationFrameId: null,
  lastUpdateMask: 0,
  updateTick: 0,

  initFromCompileConfig: async (compileConfig: CompileConfig) => {
    if (compileConfig.mode === "serverUrl") {
      alert("TODO: CORS issue :/");
      return;
    }

    const bytes = new Uint8Array(await compileConfig.file.arrayBuffer());
    const player = await initSonorustPlayer(
      bytes,
      encodeURIComponent(compileConfig.level),
    );
    set({
      player,
      playerState: player.state,
    });

    player.subscribe((mask: number) => {
      usePlayerStore.setState((prev) => ({
        playerState: player.state,
        lastUpdateMask: mask,
        updateTick: prev.updateTick + 1,
      }));
    });
  },

  deinit: () => {
    const { player } = get();
    if (player) player.free();
    set({ player: null });
  },

  start: () => {
    const { player, animationLoopRunning } = get();
    if (!player || animationLoopRunning) return;

    set({ animationLoopRunning: true });

    const tick = () => {
      const state = player.runUntilBreak(1000);

      set((prev) => {
        const stateChanged =
          JSON.stringify(prev.playerState) !== JSON.stringify(state);

        return {
          playerState: state,
          updateTick: stateChanged ? prev.updateTick + 1 : prev.updateTick,
        };
      });

      if (state.type === "paused") {
        set({ animationLoopRunning: false, animationFrameId: null });
        return;
      }

      if (get().animationLoopRunning) {
        const id = requestAnimationFrame(tick);
        set({ animationFrameId: id });
      }
    };

    const id = requestAnimationFrame(tick);
    set({ animationFrameId: id });
  },
  pause: () => {
    const { animationFrameId, player } = get();
    if (animationFrameId) cancelAnimationFrame(animationFrameId);
    set({ animationLoopRunning: false, animationFrameId: null });
    return player.getCurrentExecutionTarget();
  },
  stepOver: () => {
    const { player } = get();
    if (!player) return;

    const state = player.runUntilBreak(1);
    set((prev) => ({
      playerState: state,
      updateTick: prev.updateTick + 1,
    }));
  },
}));
