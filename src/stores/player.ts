import { create } from "zustand";

/** UI-side player state. Real playback lives in the Rust player module (milestone 5). */
interface PlayerState {
  title: string | null;
  isPlaying: boolean;
  setPlaying: (playing: boolean) => void;
}

export const usePlayerStore = create<PlayerState>((set) => ({
  title: null,
  isPlaying: false,
  setPlaying: (isPlaying) => set({ isPlaying }),
}));
