import { Eraser, Logs } from "lucide-react";
import { useEffect, useRef } from "react";
import { useLogStore } from "../stores/logStore";
import { cn } from "../lib/tailwind";
import { useShallow } from "zustand/shallow";

export type LogEntry = {
  id: string;
  level: string;
  text: string;
  ts: string;
};

export default function LogsTab() {
  const { clearLogs, logs } = useLogStore(
    useShallow((s) => ({ clearLogs: s.clearLogs, logs: s.logs })),
  );

  const logsEndRef = useRef<HTMLDivElement>(null);

  useEffect(() => {
    if (logsEndRef.current) {
      logsEndRef.current.scrollIntoView({ block: "end" });
    }
  }, [logs]);

  return (
    <div className="flex flex-col flex-1 min-w-0 h-full">
      <div
        data-logs-header
        className="flex items-center bg-bg-secondary border-b border-border-primary px-3.5 min-h-8 text-[12px] text-[#d8dee9] gap-1.5 font-medium select-none"
      >
        <div className="flex flex-row w-full h-full gap-1.5 items-center">
          <Logs size={12} className="text-[#8b93a7]" />
          <span>Output</span>
        </div>
        <Eraser
          size={12}
          className="text-[#8b93a7] cursor-pointer"
          onClick={clearLogs}
        />
      </div>

      <div data-logs-content className="flex-1 min-h-0 overflow-auto py-1.5">
        {logs.map((l) => (
          <div
            key={l.id}
            className="flex gap-3 py-0.75 px-3.5 font-mono text-[12px] even:bg-bg-secondary/70"
          >
            <span className="text-[#454e5f] shrink-0">{l.timestamp}</span>
            <span
              className={cn("text-[#b6bece]", {
                "text-[#e5534b]": l.level === "ERROR",
                "text-[#e3a13d]": l.level === "WARN",
                "text-[#4fb3d9]": l.level === "INFO",
                "text-[#8d92b1]": l.level === "DEBUG",
                "text-[#6b7280]": l.level === "TRACE",
              })}
            >
              {l.text}
            </span>
          </div>
        ))}
        <div ref={logsEndRef} />
      </div>
    </div>
  );
}
