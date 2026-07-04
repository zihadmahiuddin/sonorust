import { ChevronDown, Loader2 } from "lucide-react";
import { useCallback, useState } from "react";
import type { EngineArchetypeCallbackType } from "sonorust-debugger-wasm";
import { stripIndents } from "../lib/utils";

export type CallbackTypeOption = {
  value: EngineArchetypeCallbackType;
  label: string;
};

export type CompileFormProps = {
  isSubmitting: boolean;
  submitLabel: string;
  callbackTypeOptions: CallbackTypeOption[];
  initialScript?: string;
  initialCallbackType?: EngineArchetypeCallbackType;
  onSubmit: (callbackType: EngineArchetypeCallbackType, script: string) => void;
  /** Omit to hide the Cancel button entirely (e.g. the page variant, which has nothing to cancel back to). */
  onCancel?: () => void;
};

const SAMPLE_SCRIPT = stripIndents`
  temp_mem_block_id = 10000.0;
  counter_mem_index = 0.0;
  value_mem_index = 1.0;

  get_counter = get(temp_mem_block_id, counter_mem_index);
  get_value = get(temp_mem_block_id, value_mem_index);
  counter_less_5 = less(get_counter, 5.0);

  zero = 0.0;
  one = 1.0;
  incremented_counter = add([get_counter, one]);
  incremented_value = add([get_value, one]);
  increment_counter = set(temp_mem_block_id, counter_mem_index, incremented_counter);
  increment_value = set(temp_mem_block_id, value_mem_index, incremented_value);
  reset_counter = set(temp_mem_block_id, counter_mem_index, zero);

  increment_loop_body = execute([increment_value, increment_counter]);
  increment_loop = while(counter_less_5, increment_loop_body);

  result = execute([increment_loop, reset_counter, get_value]);
  return result;
`.trim();

export function CompileForm({
  isSubmitting,
  submitLabel,
  callbackTypeOptions,
  initialScript,
  initialCallbackType,
  onSubmit,
  onCancel,
}: CompileFormProps) {
  const [callbackType, setCallbackType] = useState<
    EngineArchetypeCallbackType | undefined
  >(initialCallbackType ?? callbackTypeOptions[0]?.value);
  const [serverUrl, setServerUrl] = useState("https://sonolus.sekai.best");
  const [levelSlug, setLevelSlug] = useState("sekai-best-429-1416-expert");
  const [compileMode, setMemoryModel] = useState<
    "simpleScript" | "serverAndLevel"
  >("serverAndLevel");
  const [verboseOutput, setVerboseOutput] = useState(false);
  const [stripComments, setStripComments] = useState(true);
  const [compileScript, setCompileScript] = useState(
    initialScript ?? SAMPLE_SCRIPT,
  );

  const handleSubmit = useCallback(
    (e: React.SubmitEvent<HTMLFormElement>) => {
      e.preventDefault();
      if (callbackType === undefined) return;
      onSubmit(callbackType, compileScript);
    },
    [callbackType, compileScript, onSubmit],
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
                value="serverAndLevel"
                checked={compileMode === "serverAndLevel"}
                onChange={() => setMemoryModel("serverAndLevel")}
                className="h-3.5 w-3.5 accent-[#4fb3d9] disabled:cursor-not-allowed disabled:opacity-50"
              />
              Server & Level
            </label>

            <label className="flex items-center gap-2 text-[12.5px] text-[#d8dee9]">
              <input
                type="radio"
                name="compileMode"
                value="simpleScript"
                checked={compileMode === "simpleScript"}
                onChange={() => setMemoryModel("simpleScript")}
                className="h-3.5 w-3.5 accent-[#4fb3d9] disabled:cursor-not-allowed disabled:opacity-50"
              />
              Simple Script
            </label>
          </div>
        </fieldset>

        {compileMode === "serverAndLevel" ? (
          <>
            <label className="flex flex-col gap-1.5">
              <span className="text-[11px] uppercase tracking-wide text-[#8b93a7]">
                Server URL
              </span>
              <input
                type="text"
                value={serverUrl}
                onChange={(e) => setServerUrl(e.target.value)}
                disabled={isSubmitting}
                className="w-full rounded-md border border-border-secondary bg-bg-primary px-2.5 py-1.5 text-[12.5px] text-[#d8dee9] outline-none placeholder:text-[#5b6472] focus:border-[#4fb3d9] disabled:cursor-not-allowed disabled:opacity-50"
                placeholder="https://sonolus.sekai.best"
              />
            </label>

            <label className="flex flex-col gap-1.5">
              <span className="text-[11px] uppercase tracking-wide text-[#8b93a7]">
                Level Slug
              </span>
              <input
                type="text"
                value={levelSlug}
                onChange={(e) => setLevelSlug(e.target.value)}
                disabled={isSubmitting}
                className="w-full rounded-md border border-border-secondary bg-bg-primary px-2.5 py-1.5 text-[12.5px] text-[#d8dee9] outline-none placeholder:text-[#5b6472] focus:border-[#4fb3d9] disabled:cursor-not-allowed disabled:opacity-50"
                placeholder="sekai-best-429-1416-expert"
              />
            </label>
          </>
        ) : (
          <label className="scrollbar-container flex flex-col gap-1.5">
            <span className="text-[11px] uppercase tracking-wide text-[#8b93a7]">
              Script
            </span>
            <textarea
              value={compileScript}
              onChange={(e) => setCompileScript(e.target.value)}
              disabled={isSubmitting}
              spellCheck={false}
              className="h-40 w-full resize-none rounded-md border border-border-secondary bg-bg-primary px-2.5 py-2 font-mono text-[12.5px] leading-relaxed text-[#d8dee9] outline-none placeholder:text-[#5b6472] focus:border-[#4fb3d9] disabled:cursor-not-allowed disabled:opacity-50"
              placeholder="Enter script source…"
            />
          </label>
        )}

        <label className="flex flex-col gap-1.5">
          <span className="text-[11px] uppercase tracking-wide text-[#8b93a7]">
            Callback type
          </span>
          <div className="relative">
            <select
              value={callbackType !== undefined ? String(callbackType) : ""}
              onChange={(e) => {
                const found = callbackTypeOptions.find(
                  (o) => String(o.value) === e.target.value,
                );
                if (found) setCallbackType(found.value);
              }}
              disabled={isSubmitting}
              className="w-full appearance-none rounded-md border border-border-secondary bg-bg-primary px-2.5 py-1.5 pr-8 text-[12.5px] text-[#d8dee9] outline-none focus:border-[#4fb3d9] disabled:cursor-not-allowed disabled:opacity-50"
            >
              {callbackTypeOptions.length === 0 && (
                <option value="">No callback types available</option>
              )}
              {callbackTypeOptions.map((opt) => (
                <option key={String(opt.value)} value={String(opt.value)}>
                  {opt.label}
                </option>
              ))}
            </select>
            <ChevronDown
              size={14}
              className="pointer-events-none absolute right-2.5 top-1/2 -translate-y-1/2 text-[#8b93a7]"
            />
          </div>
        </label>

        <span className="font-bold text-red-400">
          NOTE: The following options are only placeholders and not functional
          yet.
        </span>
        <div className="flex items-center gap-6">
          <label className="flex items-center gap-2 text-[12.5px] text-[#d8dee9]">
            <input
              type="checkbox"
              checked={verboseOutput}
              onChange={(e) => setVerboseOutput(e.target.checked)}
              disabled={isSubmitting}
              className="h-3.5 w-3.5 accent-[#4fb3d9] disabled:cursor-not-allowed disabled:opacity-50"
            />
            Verbose output
          </label>
          <label className="flex items-center gap-2 text-[12.5px] text-[#d8dee9]">
            <input
              type="checkbox"
              checked={stripComments}
              onChange={(e) => setStripComments(e.target.checked)}
              disabled={isSubmitting}
              className="h-3.5 w-3.5 accent-[#4fb3d9] disabled:cursor-not-allowed disabled:opacity-50"
            />
            Strip comments
          </label>
        </div>
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
          disabled={isSubmitting || callbackType === undefined}
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
