import { useEffect, useState } from "react";
import { useNavigate, useParams } from "react-router-dom";
import { Settings } from "../types/settings";
import { fetchSettingByKey, deleteSettingByKey } from "../api/settingsApi";

const ManageSettingsPage: React.FC = () => {
  const { key } = useParams<{ key: string }>();
  const [setting, setSetting] = useState<Settings | null>(null);
  const [error, setError] = useState<string | null>(null);
  const navigate = useNavigate();

  useEffect(() => {
    const getSetting = async () => {
      if (!key) {
        setError("Invalid setting key");
        return;
      }

      try {
        const data = await fetchSettingByKey(key);
        setSetting(data);
      } catch (err) {
        setError(err instanceof Error ? err.message : "Failed to fetch setting");
      }
    };
    getSetting();
  }, [key]);

  const handleDelete = async () => {
    if (!key) return;

    try {
      await deleteSettingByKey(key);
      navigate("/settings"); // Redirect after deletion
    } catch (err) {
      setError(err instanceof Error ? err.message : "Failed to delete setting");
    }
  };

  return (
    <div className="max-w-5xl mx-auto p-6 bg-white rounded-lg shadow-md">
      {/* Top Bar: Back Button & Title */}
      <div className="flex justify-between items-center mb-4">
        <button
          onClick={() => navigate("/settings")}
          className="bg-gray-500 text-white px-4 py-2 rounded-md shadow hover:bg-gray-600 transition"
        >
          ← Back
        </button>
        <h2 className="text-2xl font-semibold">Manage Setting ✏️</h2>
      </div>

      {/* Display Backend Error */}
      {error && <p className="text-red-500 mb-4">{error}</p>}

      {/* Show Setting */}
      {setting ? (
        <div className="space-y-4">
          <div>
            <label className="block text-gray-700">Key:</label>
            <input
              type="text"
              value={setting.key}
              disabled
              className="w-full p-2 border border-gray-300 rounded-md bg-gray-100 cursor-not-allowed"
            />
          </div>

          <div>
            <label className="block text-gray-700">Type:</label>
            <input
              type="text"
              value={setting.type}
              disabled
              className="w-full p-2 border border-gray-300 rounded-md bg-gray-100 cursor-not-allowed"
            />
          </div>

          <div>
            <label className="block text-gray-700">Value:</label>
            <textarea
              value={setting.value}
              disabled
              rows={5}
              className="w-full p-2 border border-gray-300 rounded-md bg-gray-100 cursor-not-allowed"
            />
          </div>

          {/* Delete Button */}
          <button
            onClick={handleDelete}
            className="w-full bg-red-500 text-white px-4 py-2 rounded-md shadow hover:bg-red-600 transition"
          >
            Delete
          </button>
        </div>
      ) : (
        <p className="text-gray-500">Loading setting...</p>
      )}
    </div>
  );
};

export default ManageSettingsPage;
