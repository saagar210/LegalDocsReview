import { Routes, Route } from "react-router";
import MainLayout from "./components/layout/MainLayout";
import Dashboard from "./pages/Dashboard";
import Upload from "./pages/Upload";
import ReviewDetail from "./pages/ReviewDetail";
import Comparison from "./pages/Comparison";
import Templates from "./pages/Templates";
import Reports from "./pages/Reports";
import Settings from "./pages/Settings";

function App() {
  return (
    <Routes>
      <Route element={<MainLayout />}>
        <Route path="/" element={<Dashboard />} />
        <Route path="/upload" element={<Upload />} />
        <Route path="/documents/:id" element={<ReviewDetail />} />
        <Route path="/compare" element={<Comparison />} />
        <Route path="/templates" element={<Templates />} />
        <Route path="/reports/:id" element={<Reports />} />
        <Route path="/settings" element={<Settings />} />
      </Route>
    </Routes>
  );
}

export default App;
