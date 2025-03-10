import { BrowserRouter, Routes, Route, Navigate } from "react-router-dom";
import { useEffect, useState, ReactNode } from "react";
import SettingsListPage from "./pages/SettingsList.tsx";
import CreateSettingsPage from "./pages/CreateSettings.tsx";
import ManageSettingsPage from "./pages/ManageSettings.tsx";
import AuthTokenPage from "./pages/auth.tsx";
import { validateToken } from "./api/authApi.ts";

interface ProtectedRouteProps {
  element: ReactNode;
}

const ProtectedRoute: React.FC<ProtectedRouteProps> = ({ element }) => {
  const [isAuthenticated, setIsAuthenticated] = useState<boolean | null>(null);

  useEffect(() => {
    const checkToken = async () => {
      const token: string | null = localStorage.getItem("authToken");
      if (!token) {
        setIsAuthenticated(false);
        return;
      }
      try {
        await validateToken(token);
        setIsAuthenticated(true);
      } catch {
        localStorage.removeItem("authToken");
        setIsAuthenticated(false);
      }
    };

    checkToken();
  }, []);

  if (isAuthenticated === null) {
    return <p className="text-gray-600 text-center">Checking authentication...</p>;
  }

  return isAuthenticated ? <>{element}</> : <Navigate to="/auth-token" replace />;
};

function App() {
  return (
    <BrowserRouter>
      <Routes>
        <Route path="/auth-token" element={<AuthTokenPage />} />
        <Route path="/settings" element={<ProtectedRoute element={<SettingsListPage />} />} />
        <Route path="/settings/create" element={<ProtectedRoute element={<CreateSettingsPage />} />} />
        <Route path="/settings/:key" element={<ProtectedRoute element={<ManageSettingsPage />} />} />

        <Route path="*" element={<ProtectedRoute element={<SettingsListPage />} />} />
      </Routes>
    </BrowserRouter>
  );
}

export default App;
