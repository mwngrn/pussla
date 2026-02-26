import { create } from "zustand";

interface SettingsState {
  includePii: boolean;
  underAllocationThreshold: number;
  setIncludePii: (v: boolean) => void;
  setUnderAllocationThreshold: (v: number) => void;
}

export const useSettingsStore = create<SettingsState>((set) => ({
  includePii: true,
  underAllocationThreshold: 80,
  setIncludePii: (v) => set({ includePii: v }),
  setUnderAllocationThreshold: (v) => set({ underAllocationThreshold: v }),
}));
