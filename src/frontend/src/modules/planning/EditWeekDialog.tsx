import { useState, useEffect } from "react";
import { useQueryClient } from "@tanstack/react-query";
import {
  Dialog,
  DialogContent,
  DialogHeader,
  DialogTitle,
} from "@/components/ui/dialog";
import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import {
  DashboardUser,
  WeeklyStats,
  updateAllocation,
  extractProjects,
  DashboardData,
} from "@/api/dashboard";
import { formatWeekLabel, formatWeekDate } from "./utils";
import { Plus, Trash2 } from "lucide-react";

interface EditRow {
  project: string;
  hours: string; // string for controlled input
}

const DEFAULT_CAPACITY_HOURS = 40;

function percentToHours(loadPercent: number, capacityHours: number): number {
  return Math.round((loadPercent / 100) * capacityHours * 10) / 10;
}

function hoursToPercent(hours: number, capacityHours: number): number {
  if (capacityHours <= 0) return 0;
  return Math.round((hours / capacityHours) * 100);
}

interface Props {
  open: boolean;
  onOpenChange: (open: boolean) => void;
  user: DashboardUser | null;
  slot: WeeklyStats | null;
  dashboardData: DashboardData | null;
}

export function EditWeekDialog({
  open,
  onOpenChange,
  user,
  slot,
  dashboardData,
}: Props) {
  const qc = useQueryClient();
  const [rows, setRows] = useState<EditRow[]>([]);
  const [error, setError] = useState<string | null>(null);
  const [saving, setSaving] = useState(false);
  const capacityHours = slot?.capacity_hours ?? DEFAULT_CAPACITY_HOURS;

  // Reset rows whenever the dialog opens with new data
  useEffect(() => {
    if (open && slot) {
      setRows(
        slot.projects.length > 0
          ? slot.projects.map((p) => ({
              project: p.project,
              hours: String(
                typeof p.planned_hours === "number"
                  ? p.planned_hours
                  : percentToHours(p.load, capacityHours)
              ),
            }))
          : [{ project: "", hours: "0" }]
      );
      setError(null);
    }
  }, [open, slot, capacityHours]);

  const projectOptions = dashboardData ? extractProjects(dashboardData) : [];
  const totalHours = rows.reduce((sum, r) => sum + (Number(r.hours) || 0), 0);
  const totalPercent = Math.round((totalHours / capacityHours) * 100);

  const updateRow = (idx: number, field: keyof EditRow, value: string) => {
    setRows((prev) => {
      const next = [...prev];
      next[idx] = { ...next[idx], [field]: value };
      return next;
    });
  };

  const addRow = () =>
    setRows((prev) => [...prev, { project: "", hours: "0" }]);

  const removeRow = (idx: number) =>
    setRows((prev) => {
      const next = prev.filter((_, i) => i !== idx);
      return next.length > 0 ? next : [{ project: "", hours: "0" }];
    });

  const handleSave = async () => {
    if (!user || !slot) return;
    setError(null);

    const allocations: Array<{
      project: string;
      load: number;
      planned_hours: number;
      capacity_hours: number;
    }> = [];
    for (const row of rows) {
      const project = row.project.trim();
      const hours = Number(row.hours);
      if (!project && !row.hours) continue;
      if (!project) {
        setError("Project name is required for each row.");
        return;
      }
      if (!Number.isFinite(hours) || hours < 0) {
        setError("Hours must be a non-negative number.");
        return;
      }
      allocations.push({
        project,
        planned_hours: Math.round(hours * 10) / 10,
        capacity_hours: capacityHours,
        load: hoursToPercent(hours, capacityHours),
      });
    }

    setSaving(true);
    try {
      await updateAllocation({
        alias: user.alias,
        week: slot.week,
        allocations,
      });
      qc.invalidateQueries({ queryKey: ["dashboard"] });
      onOpenChange(false);
    } catch (err) {
      setError(err instanceof Error ? err.message : "Failed to save");
    } finally {
      setSaving(false);
    }
  };

  if (!user || !slot) return null;

  return (
    <Dialog open={open} onOpenChange={onOpenChange}>
      <DialogContent className="max-w-lg">
        <DialogHeader>
          <DialogTitle>Edit week allocation</DialogTitle>
        </DialogHeader>

        <div className="text-sm text-gray-600 -mt-2">
          <span className="font-medium text-gray-900">{user.display_name}</span>
          {user.alias !== user.display_name && (
            <span className="text-gray-400 ml-1">({user.alias})</span>
          )}
          {" · "}
          {formatWeekLabel(slot.week)} — {formatWeekDate(slot.week)}
        </div>

        {/* Total indicator */}
        <div
          className={`text-sm font-medium px-3 py-1.5 rounded-md inline-flex ${
            totalPercent > 100
              ? "bg-red-50 text-red-700"
              : totalPercent >= 80
              ? "bg-green-50 text-green-700"
              : totalHours > 0
              ? "bg-yellow-50 text-yellow-700"
              : "bg-gray-50 text-gray-500"
          }`}
        >
          Total: {Math.round(totalHours * 10) / 10}h ({totalPercent}%)
        </div>

        {/* Rows */}
        <div className="space-y-2 max-h-72 overflow-y-auto pr-1">
          {rows.map((row, idx) => (
            <div key={idx} className="flex items-center gap-2">
              <input
                list="project-options"
                value={row.project}
                onChange={(e) => updateRow(idx, "project", e.target.value)}
                placeholder="Project"
                className="flex-1 h-8 rounded-md border border-input px-2 text-sm focus:outline-none focus:ring-1 focus:ring-ring"
              />
              <Input
                type="number"
                min={0}
                step={0.5}
                value={row.hours}
                onChange={(e) => updateRow(idx, "hours", e.target.value)}
                className="w-20 h-8 text-sm"
              />
              <span className="text-sm text-gray-400 w-3">h</span>
              <button
                onClick={() => removeRow(idx)}
                className="text-gray-400 hover:text-red-500 transition-colors p-1 rounded"
                title="Remove"
              >
                <Trash2 className="h-4 w-4" />
              </button>
            </div>
          ))}
        </div>

        <datalist id="project-options">
          {projectOptions.map((name) => (
            <option key={name} value={name} />
          ))}
        </datalist>

        <button
          onClick={addRow}
          className="text-sm text-blue-600 hover:text-blue-700 flex items-center gap-1 mt-1"
        >
          <Plus className="h-3.5 w-3.5" />
          Add project
        </button>

        {error && (
          <p className="text-sm text-red-600 bg-red-50 rounded px-3 py-2">
            {error}
          </p>
        )}

        <div className="flex justify-between pt-2 border-t">
          <Button
            variant="outline"
            size="sm"
            onClick={() => onOpenChange(false)}
          >
            Cancel
          </Button>
          <Button size="sm" onClick={handleSave} disabled={saving}>
            {saving ? "Saving…" : "Save changes"}
          </Button>
        </div>
      </DialogContent>
    </Dialog>
  );
}
