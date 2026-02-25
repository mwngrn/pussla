import { useQuery } from "@tanstack/react-query";
import { fetchDashboardData } from "@/api/dashboard";
import { useSettingsStore } from "@/store/settings.store";
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import { Input } from "@/components/ui/input";
import { Label } from "@/components/ui/label";

export function SettingsPage() {
  const {
    includePii,
    setIncludePii,
    underAllocationThreshold,
    setUnderAllocationThreshold,
  } = useSettingsStore();

  const { data } = useQuery({
    queryKey: ["dashboard", includePii],
    queryFn: () => fetchDashboardData(includePii),
  });

  return (
    <div className="p-6 max-w-xl">
      <h2 className="text-xl font-semibold text-gray-900 mb-6">Settings</h2>

      <div className="space-y-4">
        {/* Privacy */}
        <Card>
          <CardHeader>
            <CardTitle className="text-base">Privacy</CardTitle>
          </CardHeader>
          <CardContent className="space-y-3">
            <div className="flex items-center justify-between">
              <div>
                <Label htmlFor="pii-toggle" className="text-sm font-medium">
                  Show real names (PII)
                </Label>
                <p className="text-xs text-gray-500 mt-0.5">
                  When disabled, only aliases are shown (AI-safe mode).
                </p>
              </div>
              <button
                id="pii-toggle"
                role="switch"
                aria-checked={includePii}
                onClick={() => setIncludePii(!includePii)}
                className={`relative inline-flex h-5 w-9 items-center rounded-full transition-colors focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-offset-1 ${
                  includePii ? "bg-blue-600" : "bg-gray-300"
                }`}
              >
                <span
                  className={`inline-block h-3.5 w-3.5 transform rounded-full bg-white shadow-sm transition-transform ${
                    includePii ? "translate-x-4.5" : "translate-x-0.5"
                  }`}
                />
              </button>
            </div>
          </CardContent>
        </Card>

        {/* Thresholds */}
        <Card>
          <CardHeader>
            <CardTitle className="text-base">Utilization thresholds</CardTitle>
          </CardHeader>
          <CardContent className="space-y-4">
            <div className="space-y-1.5">
              <Label htmlFor="threshold">
                Under-allocation warning threshold
              </Label>
              <div className="flex items-center gap-2">
                <Input
                  id="threshold"
                  type="number"
                  min={0}
                  max={100}
                  step={5}
                  value={underAllocationThreshold}
                  onChange={(e) =>
                    setUnderAllocationThreshold(Number(e.target.value))
                  }
                  className="w-24"
                />
                <span className="text-sm text-gray-500">%</span>
              </div>
              <p className="text-xs text-gray-500">
                Weeks below this % are highlighted yellow. Default: 80%.
              </p>
            </div>

            <div className="rounded-lg bg-gray-50 border p-3 text-xs text-gray-600 space-y-1">
              <p>
                <span className="inline-block w-3 h-3 rounded-sm bg-gray-200 mr-1.5 align-middle" />
                0% — unallocated
              </p>
              <p>
                <span className="inline-block w-3 h-3 rounded-sm bg-red-200 mr-1.5 align-middle" />
                {"< "}
                {Math.round(underAllocationThreshold * 0.9)}% — severely
                under-allocated
              </p>
              <p>
                <span className="inline-block w-3 h-3 rounded-sm bg-yellow-200 mr-1.5 align-middle" />
                {Math.round(underAllocationThreshold * 0.9)}–
                {underAllocationThreshold - 1}% — under-allocated
              </p>
              <p>
                <span className="inline-block w-3 h-3 rounded-sm bg-green-200 mr-1.5 align-middle" />
                {underAllocationThreshold}–100% — healthy
              </p>
              <p>
                <span className="inline-block w-3 h-3 rounded-sm bg-blue-200 mr-1.5 align-middle" />
                {">"} 100% — over-allocated
              </p>
            </div>
          </CardContent>
        </Card>

        {/* Data info */}
        {data && (
          <Card>
            <CardHeader>
              <CardTitle className="text-base">Data</CardTitle>
            </CardHeader>
            <CardContent className="text-sm text-gray-600 space-y-1.5">
              <p>
                <span className="font-medium text-gray-900">
                  {data.metrics.users_count}
                </span>{" "}
                people tracked
              </p>
              <p>
                <span className="font-medium text-gray-900">
                  {data.weeks.length}
                </span>{" "}
                weeks in dataset
              </p>
              <p>
                <span className="font-medium text-gray-900">
                  {data.metrics.average_utilization}%
                </span>{" "}
                average utilization
              </p>
              <p>
                <span className="font-medium text-gray-900">
                  {data.metrics.overbooked_slots}
                </span>{" "}
                over-allocated week slots
              </p>
              <p className="text-xs text-gray-400 pt-1">
                Generated: {data.generated_at}
              </p>
            </CardContent>
          </Card>
        )}
      </div>
    </div>
  );
}
