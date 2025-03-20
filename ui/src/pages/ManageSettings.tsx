import { useEffect, useState } from "react";
import { useNavigate, useParams } from "react-router-dom";
import { Settings } from "../types/settings";
import { fetchSettingByKey, deleteSettingByKey, updateSetting } from "../api/settingsApi";
import KeyInput from "../components/KeyInput.tsx";
import TypeInput from '../components/TypeInput';
import { formatDateForManageSettings } from "../utils/FormatDate.ts";
import PageHeader from '../components/PageHeader';
import ValueInput from "../components/ValueInput.tsx";

const ManageSettingsPage: React.FC = () => {
  const { key } = useParams<{ key: string }>();
  const [setting, setSetting] = useState<Settings | null>(null);
  const [updatedValue, setUpdatedValue] = useState("");
  const [isEditing, setIsEditing] = useState(false);
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
        setUpdatedValue(data.value);
      } catch (err) {
        setError(err instanceof Error ? err.message : "Failed to fetch setting");
      }
    };
    getSetting();
  }, [key]);

  const handleDelete = async () => {
    if (!key) return;
    // Add a confirmation popup before deletion
    if (!window.confirm("Do you really want to delete setting?")) {
      return;
    }
    try {
      await deleteSettingByKey(key);
      navigate("/settings"); // Redirect after deletion
    } catch (err) {
      setError(err instanceof Error ? err.message : "Failed to delete setting");
    }
  };

  const handleUpdate = async () => {
    if (!key) return;
    try {
      await updateSetting(key, updatedValue);
      // After update, re-fetch the setting to reflect changes
      const updatedSetting = await fetchSettingByKey(key);
      setSetting(updatedSetting);
      setIsEditing(false);
    } catch (err) {
      setError(err instanceof Error ? err.message : "Failed to update setting");
    }
  };

  const handleCancelEdit = () => {
    // Revert changes if any and exit edit mode
    if (setting) {
      setUpdatedValue(setting.value);
    }
    setIsEditing(false);
  };

  return (
    <div className="max-w-5xl mx-auto p-6 bg-white rounded-lg shadow-md">
      {/* Top Bar: Back Button & Title */}
      <PageHeader
        title="Manage Setting"
        emoji="✏️"
      />

      {/* Show Setting */}
      {setting ? (
        <div className="space-y-4">
          {/* Key */}
          <KeyInput
            value={setting.key}
            disabled
          />

          {/* Type */}
          <TypeInput
            value={setting.type}
            disabled
          />

          <ValueInput
            type={setting.type}
            value={updatedValue}
            onChange={setUpdatedValue}
            disabled={!isEditing}
          />

          {/* Display Created & Updated Dates */}
          <div className="grid grid-cols-1 md:grid-cols-2 gap-6 text-gray-700">
            <div>
              <span className="block mb-1">📅 Created:</span>
              <div className="bg-gray-100 p-2 rounded-md border border-gray-300">
                {formatDateForManageSettings(new Date(setting.createdAt))}
              </div>
            </div>
            <div>
              <span className="block mb-1">🕒 Last Updated:</span>
              <div className="bg-gray-100 p-2 rounded-md border border-gray-300">
                {formatDateForManageSettings(new Date(setting.updatedAt))}
              </div>
            </div>
          </div>

          {/* Edit/Update Buttons */}
          {isEditing ? (
            <div className="flex space-x-4">
              <button
                onClick={handleUpdate}
                className="flex-1 bg-green-500 text-white px-4 py-2 rounded-md shadow hover:bg-green-600 transition"
              >
                Update
              </button>
              <button
                onClick={handleCancelEdit}
                className="flex-1 bg-gray-500 text-white px-4 py-2 rounded-md shadow hover:bg-gray-600 transition"
              >
                Cancel
              </button>
            </div>
          ) : (
            <button
              onClick={() => setIsEditing(true)}
              className="w-full bg-blue-500 text-white px-4 py-2 rounded-md shadow hover:bg-blue-600 transition"
            >
              Edit
            </button>
          )}

          {/* Delete Button */}
          <button
            onClick={handleDelete}
            className="w-full bg-red-500 text-white px-4 py-2 rounded-md shadow hover:bg-red-600 transition"
          >
            Delete
          </button>
        </div>
      ) : (
        <p className="text-gray-700">Loading setting...</p>
      )}
      {/* Display Backend Error */}
      {error && <p className="text-red-500 mb-4">{error}</p>}
    </div>
  );
};

export default ManageSettingsPage;
