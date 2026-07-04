import { create } from "zustand";

export type LogLevel = "TRACE" | "DEBUG" | "INFO" | "WARN" | "ERROR";

export interface LogEntry {
  id: string;
  level: LogLevel | string;
  text: string;
  timestamp: string;
}

interface LogStore {
  logs: LogEntry[];
  addLog: (level: LogLevel | string, text: string) => void;
  clearLogs: () => void;
}

const MAX_LOGS = 1000;

export const useLogStore = create<LogStore>((set, get) => ({
  logs: [],

  addLog: (level, text) => {
    const now = new Date();
    const timestamp =
      now.toTimeString().slice(0, 8) +
      "." +
      String(now.getMilliseconds()).padStart(3, "0");
    const id = `${now.getTime()}-${Math.random().toString(36).slice(2, 7)}`;

    const newLog: LogEntry = { id, level, text, timestamp };

    set((state) => {
      const nextLogs = [...state.logs, newLog];
      return {
        logs:
          nextLogs.length > MAX_LOGS
            ? nextLogs.slice(nextLogs.length - MAX_LOGS)
            : nextLogs,
      };
    });
  },

  clearLogs: () => set({ logs: [] }),
}));
