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

export interface ActivityLike {
  id: string;
  label: string;
  start_date: string;
  end_date: string;
}

export function normalizeDraggedWeekRange(
  startWeek: string,
  endWeek: string
): { startWeek: string; endWeek: string } | null;

export function listWeeksInRange(
  weeks: string[],
  startWeek: string,
  endWeek: string
): string[];

export function mapActivitiesToWeeks(
  activities: ActivityLike[],
  weeks: string[]
): Record<string, Array<ActivityLike & { isStart: boolean; isEnd: boolean }>>;

export function buildActivityRowLayout(
  activities: ActivityLike[],
  weeks: string[]
): {
  rows: ActivityLike[];
  byWeek: Record<
    string,
    Array<(ActivityLike & { isStart: boolean; isEnd: boolean; rowIndex: number }) | null>
  >;
};
