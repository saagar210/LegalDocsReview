import { Outlet, NavLink } from "react-router";
import {
  LayoutDashboard,
  Upload,
  FileText,
  Settings,
  Scale,
  GitCompareArrows,
  FileCheck,
} from "lucide-react";

const navItems = [
  { to: "/", label: "Dashboard", icon: LayoutDashboard },
  { to: "/upload", label: "Upload", icon: Upload },
  { to: "/compare", label: "Comparison", icon: GitCompareArrows },
  { to: "/templates", label: "Templates", icon: FileCheck },
  { to: "/settings", label: "Settings", icon: Settings },
];

function MainLayout() {
  return (
    <div className="flex h-screen">
      <aside className="w-64 bg-white border-r border-gray-200 flex flex-col">
        <div className="p-6 border-b border-gray-200">
          <div className="flex items-center gap-2">
            <Scale className="h-6 w-6 text-brand-600" />
            <h1 className="text-lg font-bold text-gray-900">LegalDocs</h1>
          </div>
          <p className="text-xs text-gray-500 mt-1">
            Document Review Assistant
          </p>
        </div>

        <nav className="flex-1 p-4 space-y-1">
          {navItems.map((item) => (
            <NavLink
              key={item.to}
              to={item.to}
              end={item.to === "/"}
              className={({ isActive }) =>
                `flex items-center gap-3 px-3 py-2 rounded-lg text-sm font-medium transition-colors ${
                  isActive
                    ? "bg-brand-50 text-brand-700"
                    : "text-gray-600 hover:bg-gray-100 hover:text-gray-900"
                }`
              }
            >
              <item.icon className="h-4 w-4" />
              {item.label}
            </NavLink>
          ))}
        </nav>

        <div className="p-4 border-t border-gray-200">
          <div className="flex items-center gap-2 text-xs text-gray-400">
            <FileText className="h-3 w-3" />
            <span>v0.1.0</span>
          </div>
        </div>
      </aside>

      <main className="flex-1 overflow-y-auto">
        <Outlet />
      </main>
    </div>
  );
}

export default MainLayout;
