export const DEFAULT_WEEKLY_CAPACITY_HOURS: number;

export function loadPercentToHours(
  loadPercent: number,
  capacityHours?: number
): number;

export function hoursToLoadPercent(
  hours: number,
  capacityHours?: number
): number;

export function isoWeekKeyFromDate(dateISO: string): string | null;

export interface MilestoneLike {
  id: string;
  title: string;
  date: string;
}

export function mapMilestonesToWeeks(
  milestones: MilestoneLike[],
  weeks: string[]
): Record<string, MilestoneLike[]>;
