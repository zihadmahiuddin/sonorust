import { Hammer } from "lucide-react";
import {
  forwardRef,
  memo,
  useCallback,
  useImperativeHandle,
  useRef,
  useState,
} from "react";
import { useLogStore, type LogLevel } from "../stores/logStore";
import { CompileForm } from "./CompileForm";
import type { CompileConfig } from "../stores/playerStore";

export type CompileDialogHandle = {
  open: () => void;
};

export type CompileDialogModalProps = {
  onCompile: (compileConfig: CompileConfig) => Promise<void>;
};

const CompileDialogModal = memo(
  forwardRef<CompileDialogHandle, CompileDialogModalProps>(function (
    { onCompile },
    ref,
  ) {
    const addLog = (level: LogLevel, text: string) => {
      useLogStore.getState().addLog(level, text);
    };
    const dialogRef = useRef<HTMLDialogElement>(null);
    const [isCompiling, setIsCompiling] = useState(false);

    useImperativeHandle(ref, () => ({
      open: () => dialogRef.current?.showModal(),
    }));

    const handleDialogCancel = (e: React.SyntheticEvent<HTMLDialogElement>) => {
      if (isCompiling) e.preventDefault();
    };

    const handleDialogBackdropClick = (
      e: React.MouseEvent<HTMLDialogElement>,
    ) => {
      if (isCompiling) return;
      if (e.target === dialogRef.current) dialogRef.current?.close();
    };

    const handleSubmit = useCallback(
      async (compileConfig: CompileConfig) => {
        setIsCompiling(true);
        try {
          await onCompile(compileConfig);
          dialogRef.current?.close();
        } catch (err) {
          addLog("ERROR", `Compile failed: ${String(err)}`);
        } finally {
          setIsCompiling(false);
        }
      },
      [setIsCompiling, onCompile],
    );

    const handleCancel = useCallback(() => {
      dialogRef.current?.close();
    }, []);

    return (
      <dialog
        ref={dialogRef}
        onCancel={handleDialogCancel}
        onClick={handleDialogBackdropClick}
        onClose={() => setIsCompiling(false)}
        className="m-auto w-full max-w-lg rounded-lg border border-border-primary bg-bg-secondary p-0 text-[#d8dee9] shadow-2xl backdrop:bg-black/60 backdrop:backdrop-blur-[2px]"
      >
        <div className="flex h-11 min-h-11 items-center gap-2 border-b border-border-primary px-3.5">
          <Hammer size={15} className="text-[#4fb3d9]" />
          <span className="text-[13px] font-semibold tracking-[0.2px]">
            Compile script
          </span>
        </div>
        <CompileForm
          isSubmitting={isCompiling}
          submitLabel="Compile"
          onSubmit={handleSubmit}
          onCancel={handleCancel}
        />
      </dialog>
    );
  }),
);

CompileDialogModal.displayName = "CompileDialogModal";

export default CompileDialogModal;
