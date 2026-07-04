import type { LucideIcon } from "lucide-react";
import { cn } from "../lib/tailwind";
import { memo } from "react";

const ToolbarButton = memo(
  ({
    icon: Icon,
    label,
    onClick,
    disabled,
    tone,
  }: {
    icon: LucideIcon;
    label: string;
    onClick: () => void;
    disabled?: boolean;
    tone?: "red" | "green" | "blue" | "amber";
  }) => {
    return (
      <button
        className={cn(
          "flex items-center justify-center w-7 h-7 rounded-md border border-transparent bg-transparent text-[#a7afc0] cursor-pointer hover:enabled:bg-[#1a2029] hover:enabled:border-[#262e3b] focus-visible:outline-2 focus-visible:outline-[#4fb3d9] focus-visible:outline-offset-1 disabled:text-[#3b4250] disabled:cursor-not-allowed",
          {
            "hover:enabled:text-[#e5534b]": tone === "red",
            "hover:enabled:text-[#4bb679]": tone === "green",
            "hover:enabled:text-[#3da1e3]": tone === "blue",
            "hover:enabled:text-[#e3a13d]": tone === "amber",
          },
        )}
        onClick={onClick}
        disabled={disabled}
        title={label}
        aria-label={label}
      >
        <Icon size={16} strokeWidth={2} />
      </button>
    );
  },
);

ToolbarButton.displayName = "ToolbarButton";
export default ToolbarButton;
