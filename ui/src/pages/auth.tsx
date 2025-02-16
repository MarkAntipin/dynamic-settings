import { useEffect, useState } from "react";
import { useNavigate } from "react-router-dom";
import { validateToken } from "../api/authApi.ts";

const AuthTokenPage: React.FC = () => {
  const [authToken, setAuthToken] = useState("");
  const [loading, setLoading] = useState<boolean>(false);
  const [error, setError] = useState<string | null>(null);
  const navigate = useNavigate();

  useEffect(() => {
    setLoading(false);
  }, []);

  const handleValidateToken = async () => {
    setLoading(true);
    setError(null);

    try {
      await validateToken(authToken);
      localStorage.setItem("authToken", authToken);
      navigate("/settings");
    } catch (err) {
      setError(err instanceof Error ? err.message : "Unknown error");
    } finally {
      setLoading(false);
    }
  };

  return (
    <div className="max-w-md mx-auto p-6 bg-white rounded-lg shadow-md">
      <h2 className="text-xl font-semibold mb-4 text-center">
        Enter Authentication Token 🔑
      </h2>

      <div className="flex flex-col gap-3">
        <input
          type="text"
          placeholder="Enter auth token..."
          value={authToken}
          onChange={(e) => setAuthToken(e.target.value)}
          className="w-full p-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-400"
        />

        <button
          onClick={handleValidateToken}
          disabled={loading || !authToken}
          className="w-full bg-blue-500 text-white px-5 py-2 rounded-md shadow hover:bg-blue-600 transition disabled:opacity-50"
        >
          {loading ? "Validating..." : "Submit"}
        </button>
      </div>

      {/* Display Backend Error at the Bottom */}
      {error && <p className="text-red-500 mt-4 text-center">{error}</p>}
    </div>
  );
};

export default AuthTokenPage;
