import { BrowserRouter, Routes, Route, Navigate } from "react-router-dom";
import SettingsListPage from "./pages/SettingsList.tsx";
import CreateSettingsPage from "./pages/CreateSettings.tsx";
import ManageSettingsPage from "./pages/ManageSettings.tsx";

function App() {
  return (
    <>
    <BrowserRouter>
      <Routes>
        {/* Redirect "/" to "/settings" */}
        <Route path="/" element={<Navigate to="/settings" replace />} />
        <Route path="/settings" element={<SettingsListPage />} />
        <Route path="/settings/create" element={<CreateSettingsPage />} />
        <Route path="/settings/:key" element={<ManageSettingsPage />} />
      </Routes>
    </BrowserRouter>
    </>
  )
}

export default App
