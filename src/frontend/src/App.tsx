import { useEffect, useState, type ReactNode } from "react";
import {
  BrowserRouter,
  Routes,
  Route,
  Navigate,
  Link,
  useLocation,
} from "react-router-dom";
import { useQuery } from "@tanstack/react-query";
import { fetchDashboardData } from "@/api/dashboard";
import { useSettingsStore } from "@/store/settings.store";
import { GanttPage } from "@/modules/planning/GanttPage";
import { UtilizationPage } from "@/modules/planning/UtilizationPage";
import { PeoplePage } from "@/modules/people/PeoplePage";
import { ProjectsPage } from "@/modules/projects/ProjectsPage";
import { AllocationsPage } from "@/modules/allocations/AllocationsPage";
import { SettingsPage } from "@/modules/settings/SettingsPage";
import {
  GanttChartSquare,
  BarChart2,
  Users,
  FolderOpen,
  CalendarDays,
  Settings,
  Menu,
  X,
  Eye,
  EyeOff,
} from "lucide-react";

function AppLayout({ children }: { children: ReactNode }) {
  const location = useLocation();
  const [sidebarOpen, setSidebarOpen] = useState(false);
  const { includePii, setIncludePii } = useSettingsStore();

  const { data } = useQuery({
    queryKey: ["dashboard", includePii],
    queryFn: () => fetchDashboardData(includePii),
  });

  // Close sidebar on navigation (mobile)
  useEffect(() => {
    setSidebarOpen(false);
  }, [location.pathname]);

  const navLink = (
    to: string,
    icon: ReactNode,
    label: string
  ) => {
    const active = location.pathname.startsWith(to);
    return (
      <Link
        to={to}
        className={`flex items-center gap-2 px-3 py-2 rounded-md text-sm font-medium transition-colors ${
          active
            ? "bg-blue-50 text-blue-700"
            : "text-gray-700 hover:bg-gray-100"
        }`}
      >
        {icon}
        {label}
      </Link>
    );
  };

  return (
    <div className="flex h-screen bg-gray-50">
      {/* Mobile backdrop */}
      {sidebarOpen && (
        <div
          className="fixed inset-0 z-20 bg-black/30 md:hidden"
          onClick={() => setSidebarOpen(false)}
        />
      )}

      {/* Sidebar */}
      <aside
        className={`fixed md:static inset-y-0 left-0 z-30 w-56 bg-white border-r flex flex-col transform transition-transform duration-200 ${
          sidebarOpen ? "translate-x-0" : "-translate-x-full md:translate-x-0"
        }`}
      >
        {/* Brand */}
        <div className="p-4 border-b">
          <div className="flex items-center gap-2">
            <span className="text-xl">🧩</span>
            <span className="font-bold text-gray-900 tracking-tight">
              Pussla
            </span>
          </div>
          <p className="text-[10px] text-gray-400 mt-0.5 pl-0.5">
            Planning-as-Code dashboard
          </p>
        </div>

        {/* Navigation */}
        <nav className="flex-1 p-3 space-y-1 overflow-y-auto">
          <p className="px-3 pt-1 pb-0.5 text-[10px] font-semibold uppercase tracking-wider text-gray-400">
            Planning
          </p>
          {navLink(
            "/gantt",
            <GanttChartSquare className="h-4 w-4" />,
            "Gantt"
          )}
          {navLink(
            "/utilization",
            <BarChart2 className="h-4 w-4" />,
            "Utilization"
          )}

          <p className="px-3 pt-3 pb-0.5 text-[10px] font-semibold uppercase tracking-wider text-gray-400">
            Manage
          </p>
          {navLink("/people", <Users className="h-4 w-4" />, "People")}
          {navLink(
            "/projects",
            <FolderOpen className="h-4 w-4" />,
            "Projects"
          )}
          {navLink(
            "/allocations",
            <CalendarDays className="h-4 w-4" />,
            "Allocations"
          )}

          <p className="px-3 pt-3 pb-0.5 text-[10px] font-semibold uppercase tracking-wider text-gray-400">
            System
          </p>
          {navLink(
            "/settings",
            <Settings className="h-4 w-4" />,
            "Settings"
          )}
        </nav>

        {/* Stats + PII toggle */}
        <div className="p-3 border-t space-y-2">
          {data && (
            <div className="px-3 py-2 rounded-md bg-gray-50 text-xs text-gray-600 space-y-0.5">
              <div className="flex justify-between">
                <span>People</span>
                <span className="font-medium text-gray-800">
                  {data.metrics.users_count}
                </span>
              </div>
              <div className="flex justify-between">
                <span>Avg util.</span>
                <span className="font-medium text-gray-800">
                  {data.metrics.average_utilization}%
                </span>
              </div>
              {data.metrics.overbooked_slots > 0 && (
                <div className="flex justify-between text-orange-600">
                  <span>Overbooked</span>
                  <span className="font-medium">
                    {data.metrics.overbooked_slots}
                  </span>
                </div>
              )}
            </div>
          )}

          <button
            onClick={() => setIncludePii(!includePii)}
            className={`w-full flex items-center gap-2 px-3 py-1.5 rounded-md text-xs font-medium transition-colors ${
              includePii
                ? "bg-blue-50 text-blue-700 hover:bg-blue-100"
                : "bg-gray-100 text-gray-600 hover:bg-gray-200"
            }`}
            title={includePii ? "Hide real names (PII)" : "Show real names"}
          >
            {includePii ? (
              <Eye className="h-3.5 w-3.5" />
            ) : (
              <EyeOff className="h-3.5 w-3.5" />
            )}
            {includePii ? "PII visible" : "Aliases only"}
          </button>
        </div>
      </aside>

      {/* Main content */}
      <main className="flex-1 overflow-auto min-w-0">
        {/* Mobile top bar */}
        <div className="md:hidden sticky top-0 z-10 flex items-center gap-2 px-3 py-2 bg-white border-b">
          <button
            onClick={() => setSidebarOpen((o) => !o)}
            className="p-1.5 rounded hover:bg-gray-100 text-gray-600 transition-colors"
            aria-label="Toggle menu"
          >
            {sidebarOpen ? (
              <X className="h-5 w-5" />
            ) : (
              <Menu className="h-5 w-5" />
            )}
          </button>
          <span className="font-bold text-gray-900">🧩 Pussla</span>
        </div>

        {children}
      </main>
    </div>
  );
}

export default function App() {
  return (
    <BrowserRouter>
      <Routes>
        <Route
          path="/*"
          element={
            <AppLayout>
              <Routes>
                <Route path="/gantt" element={<GanttPage />} />
                <Route path="/utilization" element={<UtilizationPage />} />
                <Route path="/people" element={<PeoplePage />} />
                <Route path="/projects" element={<ProjectsPage />} />
                <Route path="/allocations" element={<AllocationsPage />} />
                <Route path="/settings" element={<SettingsPage />} />
                <Route path="*" element={<Navigate to="/utilization" replace />} />
              </Routes>
            </AppLayout>
          }
        />
      </Routes>
    </BrowserRouter>
  );
}
