import { NavLink, Route, Routes } from"react-router";
import { PlayerBar } from"@/components/PlayerBar";
import { LibraryPage } from"@/pages/LibraryPage";
import { SearchPage } from"@/pages/SearchPage";
import { TorboxPage } from"@/pages/TorboxPage";
import { DownloadsPage } from"@/pages/DownloadsPage";
import { SettingsPage } from"@/pages/SettingsPage";

const NAV = [
  { to:"/", label:"Library" },
  { to:"/search", label:"Search" },
  { to:"/torbox", label:"TorBox" },
  { to:"/downloads", label:"Downloads" },
  { to:"/settings", label:"Settings" },
];

export default function App() {
  return (
    <div className="flex h-screen flex-col">
      <div className="flex min-h-0 flex-1">
        <nav className="flex w-48 shrink-0 flex-col gap-1 border-r border-border p-3">
          <h1 className="mb-4 px-2 text-sm font-semibold tracking-wide">Audiobook Manager</h1>
          {NAV.map((item) => (
            <NavLink
              key={item.to}
              to={item.to}
              end
              className={({ isActive }) =>
                `rounded-md px-2 py-1.5 text-sm ${
                  isActive
                    ?"bg-accent font-medium text-accent-foreground"
                    :"hover:bg-muted"
                }`
              }
            >
              {item.label}
            </NavLink>
          ))}
        </nav>
        <main className="min-w-0 flex-1 overflow-y-auto p-6">
          <Routes>
            <Route path="/" element={<LibraryPage />} />
            <Route path="/search" element={<SearchPage />} />
            <Route path="/torbox" element={<TorboxPage />} />
            <Route path="/downloads" element={<DownloadsPage />} />
            <Route path="/settings" element={<SettingsPage />} />
          </Routes>
        </main>
      </div>
      <PlayerBar />
    </div>
  );
}
