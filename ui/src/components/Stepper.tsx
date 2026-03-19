import { useTranslation } from "react-i18next";
import { getStepperPosition, STEPPER_LABELS } from "../utils/stepperMapping";
import type { AppState } from "../types/api";

interface Props {
  state: AppState;
}

export default function Stepper({ state }: Props) {
  const { t } = useTranslation();
  const position = getStepperPosition(state);

  // Hide when position = -1 (torrent_creator, error) or 0 (idle/searching)
  if (position <= 0) return null;

  return (
    <div className="bg-surface border-b border-edge px-6 py-3">
      <div className="flex items-center justify-center max-w-2xl mx-auto">
        {STEPPER_LABELS.map((label, i) => (
          <div key={label} className="flex items-center">
            {i > 0 && (
              <div
                className={`w-8 sm:w-12 h-0.5 transition-colors ${
                  i <= position ? "bg-green-500" : "bg-edge"
                }`}
              />
            )}
            <div className="flex flex-col items-center gap-1">
              <div
                className={`w-8 h-8 rounded-full flex items-center justify-center text-sm font-medium transition-colors ${
                  i < position
                    ? "bg-green-500 text-white"
                    : i === position
                    ? "bg-blue-600 text-white"
                    : "bg-input text-fg-faint border border-edge"
                }`}
              >
                {i < position ? (
                  <svg
                    xmlns="http://www.w3.org/2000/svg"
                    viewBox="0 0 24 24"
                    fill="none"
                    stroke="currentColor"
                    strokeWidth="3"
                    strokeLinecap="round"
                    strokeLinejoin="round"
                    className="w-4 h-4"
                  >
                    <polyline points="20 6 9 17 4 12" />
                  </svg>
                ) : (
                  i + 1
                )}
              </div>
              <span
                className={`text-xs whitespace-nowrap ${
                  i <= position ? "text-fg" : "text-fg-faint"
                }`}
              >
                {t(label)}
              </span>
            </div>
          </div>
        ))}
      </div>
    </div>
  );
}
