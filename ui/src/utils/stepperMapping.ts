import type { AppState } from "../types/api";

export function getStepperPosition(state: AppState): number {
  switch (state.step) {
    case "idle":
    case "searching":
      return 0;
    case "selecting":
    case "torrent_selecting":
    case "torrent_parsed":
    case "torrent_no_results":
      return 1;
    case "game_extras":
    case "movie_extras":
    case "app_form":
      return 2;
    case "template_pick":
      return 3;
    case "generating":
      return -2; // keep last position (generating is used both for loading details and final generation)
    case "done":
      return 4;
    default:
      // torrent_creator, torrent_creating, error
      return -1;
  }
}

export const STEPPER_LABELS = [
  "stepper.search",
  "stepper.selection",
  "stepper.details",
  "stepper.template",
  "stepper.result",
] as const;
