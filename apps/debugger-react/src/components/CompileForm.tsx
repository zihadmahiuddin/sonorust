import { Loader2 } from "lucide-react";
import { useCallback, useState } from "react";
import type { CompileConfig } from "../stores/playerStore";

export type CompileFormProps = {
  isSubmitting: boolean;
  submitLabel: string;
  onSubmit: (compileMode: CompileConfig) => void;
  /** Omit to hide the Cancel button entirely (e.g. the page variant, which has nothing to cancel back to). */
  onCancel?: () => void;
};

export function CompileForm({
  isSubmitting,
  submitLabel,
  onSubmit,
  onCancel,
}: CompileFormProps) {
  const [compileConfig, setCompileConfig] = useState<CompileConfig>({
    mode: "scpFile",
    file: null,
    level: "beatconnect-mania-1796206-[4K] Close to normal",
  });

  const handleSubmit = useCallback(
    (e: React.SubmitEvent<HTMLFormElement>) => {
      e.preventDefault();
      onSubmit(compileConfig);
    },
    [compileConfig, onSubmit, compileConfig],
  );

  return (
    <form onSubmit={handleSubmit} className="flex flex-col">
      <div className="flex flex-col gap-4 px-4 py-4">
        <fieldset className="flex flex-col gap-1.5" disabled={isSubmitting}>
          <legend className="text-[11px] uppercase tracking-wide text-[#8b93a7] mb-1">
            Compile Mode
          </legend>
          <div className="flex items-center gap-4">
            <label className="flex items-center gap-2 text-[12.5px] text-[#d8dee9]">
              <input
                type="radio"
                name="compileMode"
                value="scpFile"
                checked={compileConfig.mode === "scpFile"}
                onChange={() =>
                  setCompileConfig((prev) => {
                    if (prev.mode === "scpFile") {
                      return { ...prev };
                    } else {
                      return { ...prev, mode: "scpFile", file: null };
                    }
                  })
                }
                className="h-3.5 w-3.5 accent-[#4fb3d9] disabled:cursor-not-allowed disabled:opacity-50"
              />
              SCP File
            </label>

            <label className="flex items-center gap-2 text-[12.5px] text-[#d8dee9]">
              <input
                type="radio"
                name="compileMode"
                value="serverUrl"
                checked={compileConfig.mode === "serverUrl"}
                onChange={() =>
                  setCompileConfig((prev) => {
                    if (prev.mode === "serverUrl") {
                      return { ...prev };
                    } else {
                      return { ...prev, mode: "serverUrl", url: "" };
                    }
                  })
                }
                className="h-3.5 w-3.5 accent-[#4fb3d9] disabled:cursor-not-allowed disabled:opacity-50"
              />
              Server & Level
            </label>
          </div>
        </fieldset>

        {compileConfig.mode === "serverUrl" && (
          <label className="flex flex-col gap-1.5">
            <span className="text-[11px] uppercase tracking-wide text-[#8b93a7]">
              Server URL
            </span>
            <input
              type="text"
              value={compileConfig.url}
              onChange={(e) =>
                setCompileConfig((prev) => {
                  if (prev.mode === "serverUrl") {
                    return { ...prev, url: e.target.value };
                  } else {
                    return { ...prev };
                  }
                })
              }
              disabled={isSubmitting}
              className="w-full rounded-md border border-border-secondary bg-bg-primary px-2.5 py-1.5 text-[12.5px] text-[#d8dee9] outline-none placeholder:text-[#5b6472] focus:border-[#4fb3d9] disabled:cursor-not-allowed disabled:opacity-50"
              placeholder="https://sonolus.sekai.best"
            />
          </label>
        )}

        {compileConfig.mode === "scpFile" && (
          <label className="flex flex-col gap-1.5">
            <span className="text-[11px] uppercase tracking-wide text-[#8b93a7]">
              SCP File
            </span>
            <input
              type="file"
              accept=".scp"
              onChange={(e) =>
                setCompileConfig((prev) => {
                  if (prev.mode === "scpFile") {
                    return { ...prev, file: e.target.files?.[0] ?? null };
                  } else {
                    return { ...prev };
                  }
                })
              }
              disabled={isSubmitting}
              className="w-full rounded-md border border-border-secondary bg-bg-primary px-2.5 py-1.5 text-[12.5px] text-[#d8dee9] outline-none file:mr-2.5 file:cursor-pointer file:rounded file:border-0 file:bg-[#4fb3d9]/15 file:px-2.5 file:py-1 file:text-[12.5px] file:text-[#4fb3d9] placeholder:text-[#5b6472] hover:file:bg-[#4fb3d9]/25 focus:border-[#4fb3d9] disabled:cursor-not-allowed disabled:opacity-50 disabled:file:cursor-not-allowed"
            />
            {compileConfig.file && (
              <span className="text-[11px] text-[#5b6472]">
                {compileConfig.file.name} ·{" "}
                {(compileConfig.file.size / (1024 * 1024)).toFixed(1)} MB
              </span>
            )}
          </label>
        )}

        <label className="flex flex-col gap-1.5">
          <span className="text-[11px] uppercase tracking-wide text-[#8b93a7]">
            Level
          </span>
          <input
            type="text"
            value={compileConfig.level}
            onChange={(e) =>
              setCompileConfig((prev) => {
                return { ...prev, level: e.target.value };
              })
            }
            disabled={isSubmitting}
            className="w-full rounded-md border border-border-secondary bg-bg-primary px-2.5 py-1.5 text-[12.5px] text-[#d8dee9] outline-none placeholder:text-[#5b6472] focus:border-[#4fb3d9] disabled:cursor-not-allowed disabled:opacity-50"
            placeholder="sekai-best-429-1416-expert"
          />
        </label>
      </div>

      <div className="flex items-center justify-end gap-2 border-t border-border-primary px-4 py-3">
        {onCancel && (
          <button
            type="button"
            onClick={onCancel}
            disabled={isSubmitting}
            className="rounded-md border border-[#2a3140] px-3 py-1.5 text-[12.5px] text-[#d8dee9] hover:bg-[#1c2634] disabled:cursor-not-allowed disabled:opacity-50"
          >
            Cancel
          </button>
        )}
        <button
          type="submit"
          disabled={isSubmitting}
          className="flex items-center gap-1.5 rounded-md border border-[#4fb3d9]/40 bg-[#4fb3d9]/15 px-3 py-1.5 text-[12.5px] text-[#4fb3d9] hover:bg-[#4fb3d9]/25 disabled:cursor-not-allowed disabled:opacity-60"
        >
          {isSubmitting ? (
            <>
              <Loader2 size={13} className="animate-spin" />
              Compiling…
            </>
          ) : (
            submitLabel
          )}
        </button>
      </div>
    </form>
  );
}
