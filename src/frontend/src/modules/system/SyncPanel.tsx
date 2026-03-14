import { useState } from "react";
import { useQuery, useQueryClient } from "@tanstack/react-query";
import {
  fetchSyncStatus,
  publishPlanningChanges,
  refreshPlanningData,
} from "@/api/dashboard";
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import { Button } from "@/components/ui/button";
import {
  Dialog,
  DialogContent,
  DialogHeader,
  DialogTitle,
} from "@/components/ui/dialog";
import { Upload, Download, GitBranch, AlertTriangle, CheckCircle2 } from "lucide-react";

function statusTone(scopedDirty: boolean, ahead: number | null, behind: number | null) {
  if (scopedDirty || (ahead ?? 0) > 0) {
    return "bg-amber-50 text-amber-800 border-amber-200";
  }
  if ((behind ?? 0) > 0) {
    return "bg-blue-50 text-blue-700 border-blue-200";
  }
  return "bg-emerald-50 text-emerald-700 border-emerald-200";
}

export function SyncPanel() {
  const qc = useQueryClient();
  const [publishOpen, setPublishOpen] = useState(false);
  const [refreshOpen, setRefreshOpen] = useState(false);
  const [summary, setSummary] = useState("");
  const [busy, setBusy] = useState<"publish" | "refresh" | null>(null);
  const [feedback, setFeedback] = useState<{ tone: "success" | "error"; message: string } | null>(null);

  const { data, isLoading, error } = useQuery({
    queryKey: ["sync-status"],
    queryFn: fetchSyncStatus,
    refetchInterval: 30_000,
  });

  const refreshQueries = async () => {
    await Promise.all([
      qc.invalidateQueries({ queryKey: ["dashboard"] }),
      qc.invalidateQueries({ queryKey: ["sync-status"] }),
    ]);
  };

  const handlePublish = async () => {
    setBusy("publish");
    setFeedback(null);
    try {
      const result = await publishPlanningChanges(summary);
      setPublishOpen(false);
      setSummary("");
      await refreshQueries();
      setFeedback({
        tone: "success",
        message: `Published ${result.commit_id} to ${result.publish_target}.`,
      });
    } catch (err) {
      setFeedback({
        tone: "error",
        message: err instanceof Error ? err.message : "Publish failed",
      });
    } finally {
      setBusy(null);
    }
  };

  const handleRefresh = async () => {
    setBusy("refresh");
    setFeedback(null);
    try {
      const result = await refreshPlanningData();
      setRefreshOpen(false);
      await refreshQueries();
      setFeedback({
        tone: "success",
        message: result.message,
      });
    } catch (err) {
      setFeedback({
        tone: "error",
        message: err instanceof Error ? err.message : "Refresh failed",
      });
    } finally {
      setBusy(null);
    }
  };

  const publishBlockedReason = !data
    ? ""
    : !data.has_publish_target
    ? "No shared repository is configured."
    : !data.scoped_dirty
    ? "No local planning changes to publish."
    : "";

  const refreshBlockedReason = !data
    ? ""
    : !data.has_publish_target
    ? "No shared repository is configured."
    : data.repo_dirty
    ? "Local unpublished changes would be affected. Publish or discard them first."
    : (data.ahead ?? 0) > 0
    ? "Local unpublished commits exist. Publish before refreshing."
    : "";

  return (
    <>
      <Card className="border-slate-200 bg-gradient-to-b from-white to-slate-50">
        <CardHeader className="pb-2">
          <CardTitle className="flex items-center gap-2 text-sm">
            <GitBranch className="h-4 w-4 text-slate-500" />
            Planning Sync
          </CardTitle>
        </CardHeader>
        <CardContent className="space-y-3 text-xs">
          {isLoading && <p className="text-slate-500">Checking repository status...</p>}
          {error && (
            <div className="rounded-lg border border-red-200 bg-red-50 p-3 text-red-700">
              {error instanceof Error ? error.message : "Failed to load sync status"}
            </div>
          )}

          {data && (
            <>
              <div
                className={`rounded-lg border px-3 py-2 ${statusTone(
                  data.scoped_dirty,
                  data.ahead,
                  data.behind
                )}`}
              >
                <div className="flex items-center justify-between gap-2">
                  <span className="font-medium">Branch {data.branch}</span>
                  <span>{data.head}</span>
                </div>
                <div className="mt-1 flex flex-wrap gap-x-3 gap-y-1 text-[11px] opacity-90">
                  <span>{data.scoped_changes.length} local planning change(s)</span>
                  <span>{data.ahead ?? 0} ahead</span>
                  <span>{data.behind ?? 0} behind</span>
                </div>
              </div>

              <div className="space-y-1 text-slate-600">
                <p>
                  Target:{" "}
                  <span className="font-medium text-slate-900">
                    {data.publish_target ?? "Not configured"}
                  </span>
                </p>
                <p>
                  Scope: <span className="font-medium text-slate-900">{data.data_scope}</span>
                </p>
                <p className="truncate" title={data.last_commit_subject}>
                  Last save: <span className="font-medium text-slate-900">{data.last_commit_subject}</span>
                </p>
              </div>

              {data.scoped_changes.length > 0 && (
                <div className="rounded-lg border border-slate-200 bg-white p-3">
                  <p className="mb-2 font-medium text-slate-800">Changed planning files</p>
                  <div className="space-y-1 text-[11px] text-slate-600">
                    {data.scoped_changes.slice(0, 4).map((change) => (
                      <div key={`${change.code}-${change.path}`} className="flex gap-2">
                        <span className="w-6 font-mono text-slate-400">{change.code}</span>
                        <span className="truncate">{change.path}</span>
                      </div>
                    ))}
                    {data.scoped_changes.length > 4 && (
                      <p className="text-slate-400">
                        +{data.scoped_changes.length - 4} more file(s)
                      </p>
                    )}
                  </div>
                </div>
              )}

              {feedback && (
                <div
                  className={`rounded-lg border p-3 ${
                    feedback.tone === "success"
                      ? "border-emerald-200 bg-emerald-50 text-emerald-700"
                      : "border-red-200 bg-red-50 text-red-700"
                  }`}
                >
                  <div className="flex items-start gap-2">
                    {feedback.tone === "success" ? (
                      <CheckCircle2 className="mt-0.5 h-4 w-4 shrink-0" />
                    ) : (
                      <AlertTriangle className="mt-0.5 h-4 w-4 shrink-0" />
                    )}
                    <span>{feedback.message}</span>
                  </div>
                </div>
              )}

              <div className="flex gap-2">
                <Button
                  size="sm"
                  className="flex-1"
                  onClick={() => setPublishOpen(true)}
                  disabled={!data.has_publish_target || busy !== null}
                >
                  <Upload className="h-3.5 w-3.5" />
                  Publish
                </Button>
                <Button
                  size="sm"
                  variant="outline"
                  className="flex-1"
                  onClick={() => setRefreshOpen(true)}
                  disabled={!data.has_publish_target || busy !== null}
                >
                  <Download className="h-3.5 w-3.5" />
                  Refresh
                </Button>
              </div>
            </>
          )}
        </CardContent>
      </Card>

      <Dialog open={publishOpen} onOpenChange={setPublishOpen}>
        <DialogContent>
          <DialogHeader>
            <DialogTitle>Publish planning changes</DialogTitle>
          </DialogHeader>
          <div className="space-y-3 text-sm text-slate-600">
            <p>
              Create a versioned save for the current planning changes and send it to{" "}
              <span className="font-medium text-slate-900">
                {data?.publish_target ?? "the shared repository"}
              </span>.
            </p>
            {publishBlockedReason ? (
              <div className="rounded-lg border border-amber-200 bg-amber-50 p-3 text-amber-800">
                {publishBlockedReason}
              </div>
            ) : null}
            <label className="block space-y-1">
              <span className="text-xs font-medium uppercase tracking-wide text-slate-500">
                Change summary
              </span>
              <textarea
                value={summary}
                onChange={(e) => setSummary(e.target.value)}
                rows={4}
                placeholder="Example: updated Alice's April allocations and adjusted Project Helix milestones"
                className="w-full rounded-lg border border-slate-300 px-3 py-2 text-sm focus:outline-none focus:ring-1 focus:ring-blue-500"
              />
            </label>
            <div className="flex justify-end gap-2">
              <Button variant="outline" onClick={() => setPublishOpen(false)} disabled={busy !== null}>
                Cancel
              </Button>
              <Button
                onClick={handlePublish}
                disabled={busy !== null || !!publishBlockedReason || summary.trim().length === 0}
              >
                {busy === "publish" ? "Publishing..." : "Publish now"}
              </Button>
            </div>
          </div>
        </DialogContent>
      </Dialog>

      <Dialog open={refreshOpen} onOpenChange={setRefreshOpen}>
        <DialogContent>
          <DialogHeader>
            <DialogTitle>Refresh planning data</DialogTitle>
          </DialogHeader>
          <div className="space-y-3 text-sm text-slate-600">
            <p>
              Refresh pulls the latest shared planning data into this local workspace before you continue editing.
            </p>
            {refreshBlockedReason ? (
              <div className="rounded-lg border border-amber-200 bg-amber-50 p-3 text-amber-800">
                {refreshBlockedReason}
              </div>
            ) : (
              <div className="rounded-lg border border-blue-200 bg-blue-50 p-3 text-blue-700">
                {(data?.behind ?? 0) > 0
                  ? `This workspace is ${data?.behind} commit(s) behind ${data?.publish_target}.`
                  : "No remote changes are currently known, but refresh will safely check before continuing."}
              </div>
            )}
            <div className="flex justify-end gap-2">
              <Button variant="outline" onClick={() => setRefreshOpen(false)} disabled={busy !== null}>
                Cancel
              </Button>
              <Button onClick={handleRefresh} disabled={busy !== null || !!refreshBlockedReason}>
                {busy === "refresh" ? "Refreshing..." : "Refresh now"}
              </Button>
            </div>
          </div>
        </DialogContent>
      </Dialog>
    </>
  );
}
